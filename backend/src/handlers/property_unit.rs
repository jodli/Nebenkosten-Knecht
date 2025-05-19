use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use log::{error, info};

use crate::db;
use crate::models::property_unit::{
    NewPropertyUnit, PropertyUnit, PropertyUnitDto, PropertyUnitUpdate,
};
use crate::DbPool;

// Configure routes for property units
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/property-units")
            .service(get_all_property_units)
            .service(get_property_unit_by_id)
            .service(create_property_unit)
            .service(update_property_unit)
            .service(delete_property_unit),
    );
}

// GET /api/property-units
#[get("")]
async fn get_all_property_units(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::property_units::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match property_units
        .order_by(name.asc())
        .load::<PropertyUnit>(conn)
    {
        Ok(results) => {
            let dtos: Vec<PropertyUnitDto> = results.into_iter().map(|unit| unit.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading property units: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading property units: {}", e))
        }
    }
}

// GET /api/property-units/{id}
#[get("/{id}")]
async fn get_property_unit_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::property_units::dsl::*;

    let unit_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Use .eq() instead of .find() to handle Option<i32>
    match property_units
        .filter(id.eq(unit_id))
        .first::<PropertyUnit>(conn)
    {
        Ok(unit) => HttpResponse::Ok().json(PropertyUnitDto::from(unit)),
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Property unit with ID {} not found", unit_id))
        }
        Err(e) => {
            error!("Error finding property unit {}: {}", unit_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding property unit: {}", e))
        }
    }
}

// POST /api/property-units
#[post("")]
async fn create_property_unit(
    mut new_unit_json: web::Json<NewPropertyUnit>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::property_units::dsl::*;

    let conn = &mut db::get_connection(&pool);
    let new_unit = new_unit_json.0;

    // Input validation
    if new_unit.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("Property unit name cannot be empty");
    }

    if new_unit.living_area_m2 <= 0.0 {
        return HttpResponse::BadRequest().json("Living area must be greater than 0");
    }

    let new_unit = NewPropertyUnit {
        name: new_unit.name.clone(),
        living_area_m2: new_unit.living_area_m2,
    };

    match diesel::insert_into(property_units)
        .values(&new_unit)
        .execute(conn)
    {
        Ok(_) => {
            // Get the newly created property unit to return its ID
            match property_units
                .order_by(id.desc())
                .first::<PropertyUnit>(conn)
            {
                Ok(created_unit) => {
                    info!("Created property unit: {:?}", created_unit);
                    HttpResponse::Created().json(PropertyUnitDto::from(created_unit))
                }
                Err(e) => {
                    error!("Error retrieving created property unit: {}", e);
                    HttpResponse::InternalServerError().json(format!(
                        "Property unit created but error retrieving it: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error creating property unit: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating property unit: {}", e))
        }
    }
}

// PUT /api/property-units/{id}
#[put("/{id}")]
async fn update_property_unit(
    path: web::Path<i32>,
    unit_update: web::Json<PropertyUnitUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::property_units::dsl::*;

    let unit_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Input validation
    if let Some(ref name_val) = unit_update.name {
        if name_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Property unit name cannot be empty");
        }
    }

    if let Some(area_val) = unit_update.living_area_m2 {
        if area_val <= 0.0 {
            return HttpResponse::BadRequest().json("Living area must be greater than 0");
        }
    }

    // Check if the property unit exists
    let exists = match property_units
        .filter(id.eq(unit_id))
        .first::<PropertyUnit>(conn)
    {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if property unit exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if property unit exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound()
            .json(format!("Property unit with ID {} not found", unit_id));
    }

    match diesel::update(property_units.filter(id.eq(unit_id)))
        .set(unit_update.into_inner())
        .execute(conn)
    {
        Ok(_) => {
            // Get the updated property unit
            match property_units
                .filter(id.eq(unit_id))
                .first::<PropertyUnit>(conn)
            {
                Ok(updated_unit) => {
                    info!("Updated property unit: {:?}", updated_unit);
                    HttpResponse::Ok().json(PropertyUnitDto::from(updated_unit))
                }
                Err(e) => {
                    error!("Error retrieving updated property unit: {}", e);
                    HttpResponse::InternalServerError().json(format!(
                        "Property unit updated but error retrieving it: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error updating property unit: {}", e);
            HttpResponse::InternalServerError().json(format!("Error updating property unit: {}", e))
        }
    }
}

// DELETE /api/property-units/{id}
#[delete("/{id}")]
async fn delete_property_unit(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::property_units::dsl::*;

    let unit_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the property unit exists
    let exists = match property_units
        .filter(id.eq(unit_id))
        .first::<PropertyUnit>(conn)
    {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if property unit exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if property unit exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound()
            .json(format!("Property unit with ID {} not found", unit_id));
    }

    match diesel::delete(property_units.filter(id.eq(unit_id))).execute(conn) {
        Ok(_) => {
            info!("Deleted property unit with ID: {}", unit_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            error!("Error deleting property unit: {}", e);
            HttpResponse::InternalServerError().json(format!("Error deleting property unit: {}", e))
        }
    }
}
