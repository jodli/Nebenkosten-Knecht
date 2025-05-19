use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use log::{error, info};

use crate::db;
use crate::models::meter::{Meter, MeterDto, MeterInputDto, MeterUpdate, NewMeter};
use crate::models::property_unit::PropertyUnit;
use crate::DbPool;

// Configure routes for meters
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/meters")
            .service(get_all_meters)
            .service(get_meter_by_id)
            .service(get_meters_by_property_unit)
            .service(get_common_meters)
            .service(create_meter)
            .service(update_meter)
            .service(delete_meter),
    );
}

// GET /api/meters
#[get("")]
async fn get_all_meters(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meters::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match meters.order_by(name.asc()).load::<Meter>(conn) {
        Ok(results) => {
            let dtos: Vec<MeterDto> = results.into_iter().map(|meter| meter.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading meters: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading meters: {}", e))
        }
    }
}

// GET /api/meters/{id}
#[get("/{id}")]
async fn get_meter_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meters::dsl::*;

    let meter_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match meters.filter(id.eq(meter_id)).first::<Meter>(conn) {
        Ok(meter) => HttpResponse::Ok().json(MeterDto::from(meter)),
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Meter with ID {} not found", meter_id))
        }
        Err(e) => {
            error!("Error finding meter {}: {}", meter_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding meter: {}", e))
        }
    }
}

// GET /api/meters/by-property-unit/{property_unit_id}
#[get("/by-property-unit/{property_unit_id}")]
async fn get_meters_by_property_unit(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::meters::dsl::*;
    use crate::schema::property_units;

    let property_unit_id_val = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the property unit exists
    match property_units::table
        .filter(property_units::id.eq(property_unit_id_val))
        .first::<PropertyUnit>(conn)
    {
        Ok(_) => (),
        Err(diesel::NotFound) => {
            return HttpResponse::NotFound().json(format!(
                "Property unit with ID {} not found",
                property_unit_id_val
            ));
        }
        Err(e) => {
            error!("Error checking if property unit exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if property unit exists: {}", e));
        }
    }

    match meters
        .filter(property_unit_id.eq(property_unit_id_val))
        .order_by(name.asc())
        .load::<Meter>(conn)
    {
        Ok(results) => {
            let dtos: Vec<MeterDto> = results.into_iter().map(|meter| meter.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading meters for property unit {}: {}",
                property_unit_id_val, e
            );
            HttpResponse::InternalServerError()
                .json(format!("Error loading meters for property unit: {}", e))
        }
    }
}

// GET /api/meters/common
#[get("/common")]
async fn get_common_meters(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meters::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match meters
        .filter(assignment_type.eq("common"))
        .order_by(name.asc())
        .load::<Meter>(conn)
    {
        Ok(results) => {
            let dtos: Vec<MeterDto> = results.into_iter().map(|meter| meter.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading common meters: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading common meters: {}", e))
        }
    }
}

// POST /api/meters
#[post("")]
async fn create_meter(
    new_meter: web::Json<MeterInputDto>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::models::meter::MeterAssignment;
    use crate::schema::meters::dsl::*;
    use crate::schema::property_units;

    let conn = &mut db::get_connection(&pool);

    // Input validation
    if new_meter.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("Meter name cannot be empty");
    }

    if new_meter.meter_type.trim().is_empty() {
        return HttpResponse::BadRequest().json("Meter type cannot be empty");
    }

    if new_meter.unit.trim().is_empty() {
        return HttpResponse::BadRequest().json("Meter unit cannot be empty");
    }

    // Check if property unit exists if this is a unit meter
    if new_meter.assignment_type == MeterAssignment::Unit {
        if let Some(property_unit_id_val) = new_meter.property_unit_id {
            match property_units::table
                .filter(property_units::id.eq(property_unit_id_val))
                .first::<PropertyUnit>(conn)
            {
                Ok(_) => (),
                Err(diesel::NotFound) => {
                    return HttpResponse::BadRequest().json(format!(
                        "Property unit with ID {} not found",
                        property_unit_id_val
                    ));
                }
                Err(e) => {
                    error!("Error checking if property unit exists: {}", e);
                    return HttpResponse::InternalServerError()
                        .json(format!("Error checking if property unit exists: {}", e));
                }
            }
        } else {
            return HttpResponse::BadRequest()
                .json("Property unit ID is required for meters assigned to a unit");
        }
    }

    let new_meter = NewMeter::from(new_meter.into_inner());

    match diesel::insert_into(meters).values(&new_meter).execute(conn) {
        Ok(_) => {
            // Get the newly created meter to return its ID
            match meters.order_by(id.desc()).first::<Meter>(conn) {
                Ok(created_meter) => {
                    info!("Created meter: {:?}", created_meter);
                    HttpResponse::Created().json(MeterDto::from(created_meter))
                }
                Err(e) => {
                    error!("Error retrieving created meter: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Meter created but error retrieving it: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Error creating meter: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating meter: {}", e))
        }
    }
}

// PUT /api/meters/{id}
#[put("/{id}")]
async fn update_meter(
    path: web::Path<i32>,
    meter_update: web::Json<MeterUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::meters::dsl::*;
    use crate::schema::property_units;

    let meter_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Input validation
    if let Some(ref name_val) = meter_update.name {
        if name_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Meter name cannot be empty");
        }
    }

    if let Some(ref type_val) = meter_update.meter_type {
        if type_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Meter type cannot be empty");
        }
    }

    if let Some(ref unit_val) = meter_update.unit {
        if unit_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Meter unit cannot be empty");
        }
    }

    // Check if property unit exists if it's being updated
    if let Some(Some(property_unit_id_val)) = meter_update.property_unit_id {
        match property_units::table
            .filter(property_units::id.eq(property_unit_id_val))
            .first::<PropertyUnit>(conn)
        {
            Ok(_) => (),
            Err(diesel::NotFound) => {
                return HttpResponse::BadRequest().json(format!(
                    "Property unit with ID {} not found",
                    property_unit_id_val
                ));
            }
            Err(e) => {
                error!("Error checking if property unit exists: {}", e);
                return HttpResponse::InternalServerError()
                    .json(format!("Error checking if property unit exists: {}", e));
            }
        }
    }

    // Check if the meter exists
    let exists = match meters.filter(id.eq(meter_id)).first::<Meter>(conn) {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound().json(format!("Meter with ID {} not found", meter_id));
    }

    // For assignment_type changes, we need special handling to ensure consistency
    let current_meter = meters.filter(id.eq(meter_id)).first::<Meter>(conn).unwrap();
    let mut update = meter_update.into_inner();

    // If changing to "unit" assignment type, we must have a property_unit_id
    if let Some(ref assignment_type_val) = update.assignment_type {
        if assignment_type_val == "unit"
            && (update.property_unit_id.is_none() || update.property_unit_id == Some(None))
        {
            // If we're changing to unit but not setting a property_unit_id, keep the existing one if it exists
            if current_meter.property_unit_id.is_none() {
                return HttpResponse::BadRequest()
                    .json("Property unit ID is required for meters assigned to a unit");
            }
        } else if assignment_type_val == "common" {
            // If changing to common, set property_unit_id to None
            update.property_unit_id = Some(None);
        }
    }

    match diesel::update(meters.filter(id.eq(meter_id)))
        .set(update)
        .execute(conn)
    {
        Ok(_) => {
            // Get the updated meter
            match meters.filter(id.eq(meter_id)).first::<Meter>(conn) {
                Ok(updated_meter) => {
                    info!("Updated meter: {:?}", updated_meter);
                    HttpResponse::Ok().json(MeterDto::from(updated_meter))
                }
                Err(e) => {
                    error!("Error retrieving updated meter: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Meter updated but error retrieving it: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Error updating meter: {}", e);
            HttpResponse::InternalServerError().json(format!("Error updating meter: {}", e))
        }
    }
}

// DELETE /api/meters/{id}
#[delete("/{id}")]
async fn delete_meter(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meters::dsl::*;

    let meter_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the meter exists
    let exists = match meters.filter(id.eq(meter_id)).first::<Meter>(conn) {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound().json(format!("Meter with ID {} not found", meter_id));
    }

    match diesel::delete(meters.filter(id.eq(meter_id))).execute(conn) {
        Ok(_) => {
            info!("Deleted meter with ID: {}", meter_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            error!("Error deleting meter: {}", e);
            HttpResponse::InternalServerError().json(format!("Error deleting meter: {}", e))
        }
    }
}
