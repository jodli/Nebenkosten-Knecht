use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use log::{error, info};

use crate::db;
use crate::models::property_unit::PropertyUnit;
use crate::models::tenant::{NewTenant, Tenant, TenantDto, TenantUpdate};
use crate::DbPool;

// Configure routes for tenants
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/tenants")
            .service(get_all_tenants)
            .service(get_tenant_by_id)
            .service(get_tenants_by_property_unit)
            .service(create_tenant)
            .service(update_tenant)
            .service(delete_tenant),
    );
}

// GET /api/tenants
#[get("")]
async fn get_all_tenants(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::tenants::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match tenants.order_by(name.asc()).load::<Tenant>(conn) {
        Ok(results) => {
            let dtos: Vec<TenantDto> = results.into_iter().map(|tenant| tenant.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading tenants: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading tenants: {}", e))
        }
    }
}

// GET /api/tenants/{id}
#[get("/{id}")]
async fn get_tenant_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::tenants::dsl::*;

    let tenant_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match tenants.filter(id.eq(tenant_id)).first::<Tenant>(conn) {
        Ok(tenant) => HttpResponse::Ok().json(TenantDto::from(tenant)),
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Tenant with ID {} not found", tenant_id))
        }
        Err(e) => {
            error!("Error finding tenant {}: {}", tenant_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding tenant: {}", e))
        }
    }
}

// GET /api/tenants/by-property-unit/{property_unit_id}
#[get("/by-property-unit/{property_unit_id}")]
async fn get_tenants_by_property_unit(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::property_units;
    use crate::schema::tenants::dsl::*;

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

    match tenants
        .filter(property_unit_id.eq(property_unit_id_val))
        .order_by(name.asc())
        .load::<Tenant>(conn)
    {
        Ok(results) => {
            let dtos: Vec<TenantDto> = results.into_iter().map(|tenant| tenant.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading tenants for property unit {}: {}",
                property_unit_id_val, e
            );
            HttpResponse::InternalServerError()
                .json(format!("Error loading tenants for property unit: {}", e))
        }
    }
}

// POST /api/tenants
#[post("")]
async fn create_tenant(
    new_tenant: web::Json<NewTenant>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::property_units;
    use crate::schema::tenants::dsl::*;

    let conn = &mut db::get_connection(&pool);

    // Input validation
    if new_tenant.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("Tenant name cannot be empty");
    }

    if new_tenant.number_of_persons <= 0 {
        return HttpResponse::BadRequest().json("Number of persons must be greater than 0");
    }

    // Check if the property unit exists
    match property_units::table
        .filter(property_units::id.eq(new_tenant.property_unit_id))
        .first::<PropertyUnit>(conn)
    {
        Ok(_) => (),
        Err(diesel::NotFound) => {
            return HttpResponse::BadRequest().json(format!(
                "Property unit with ID {} not found",
                new_tenant.property_unit_id
            ));
        }
        Err(e) => {
            error!("Error checking if property unit exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if property unit exists: {}", e));
        }
    }

    let new_tenant = NewTenant {
        name: new_tenant.name.clone(),
        number_of_persons: new_tenant.number_of_persons,
        property_unit_id: new_tenant.property_unit_id,
    };

    match diesel::insert_into(tenants)
        .values(&new_tenant)
        .execute(conn)
    {
        Ok(_) => {
            // Get the newly created tenant to return its ID
            match tenants.order_by(id.desc()).first::<Tenant>(conn) {
                Ok(created_tenant) => {
                    info!("Created tenant: {:?}", created_tenant);
                    HttpResponse::Created().json(TenantDto::from(created_tenant))
                }
                Err(e) => {
                    error!("Error retrieving created tenant: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Tenant created but error retrieving it: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Error creating tenant: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating tenant: {}", e))
        }
    }
}

// PUT /api/tenants/{id}
#[put("/{id}")]
async fn update_tenant(
    path: web::Path<i32>,
    tenant_update: web::Json<TenantUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::property_units;
    use crate::schema::tenants::dsl::*;

    let tenant_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Input validation
    if let Some(ref name_val) = tenant_update.name {
        if name_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Tenant name cannot be empty");
        }
    }

    if let Some(persons_val) = tenant_update.number_of_persons {
        if persons_val <= 0 {
            return HttpResponse::BadRequest().json("Number of persons must be greater than 0");
        }
    }

    // Check if property unit exists if it's being updated
    if let Some(property_unit_id_val) = tenant_update.property_unit_id {
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

    // Check if the tenant exists
    let exists = match tenants.filter(id.eq(tenant_id)).first::<Tenant>(conn) {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if tenant exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if tenant exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound().json(format!("Tenant with ID {} not found", tenant_id));
    }

    match diesel::update(tenants.filter(id.eq(tenant_id)))
        .set(tenant_update.into_inner())
        .execute(conn)
    {
        Ok(_) => {
            // Get the updated tenant
            match tenants.filter(id.eq(tenant_id)).first::<Tenant>(conn) {
                Ok(updated_tenant) => {
                    info!("Updated tenant: {:?}", updated_tenant);
                    HttpResponse::Ok().json(TenantDto::from(updated_tenant))
                }
                Err(e) => {
                    error!("Error retrieving updated tenant: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Tenant updated but error retrieving it: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Error updating tenant: {}", e);
            HttpResponse::InternalServerError().json(format!("Error updating tenant: {}", e))
        }
    }
}

// DELETE /api/tenants/{id}
#[delete("/{id}")]
async fn delete_tenant(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::tenants::dsl::*;

    let tenant_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the tenant exists
    let exists = match tenants.filter(id.eq(tenant_id)).first::<Tenant>(conn) {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if tenant exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if tenant exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound().json(format!("Tenant with ID {} not found", tenant_id));
    }

    match diesel::delete(tenants.filter(id.eq(tenant_id))).execute(conn) {
        Ok(_) => {
            info!("Deleted tenant with ID: {}", tenant_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            error!("Error deleting tenant: {}", e);
            HttpResponse::InternalServerError().json(format!("Error deleting tenant: {}", e))
        }
    }
}
