use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use log::error;

use crate::db;
use crate::models::cost::{
    AllocationMethod, AllocationMethodDto, CostType, CostTypeAllocation, CostTypeDto,
    CostTypeUpdate, FixedCost, FixedCostDto, FixedCostUpdate, NewCostType, NewCostTypeAllocation,
    NewFixedCost, NewTariff, Tariff, TariffDto, TariffUpdate,
};
use crate::DbPool;

// Configure routes for cost management
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/cost-types")
            .service(get_all_allocation_methods) // Move this to be the first service registered
            .service(get_all_cost_types)
            .service(get_cost_type_by_id)
            .service(create_cost_type)
            .service(update_cost_type)
            .service(delete_cost_type)
            .service(assign_allocation_method)
            .service(remove_allocation_method),
    );

    cfg.service(
        web::scope("/api/tariffs")
            .service(get_tariffs_by_cost_type)
            .service(create_tariff)
            .service(update_tariff)
            .service(delete_tariff),
    );

    cfg.service(
        web::scope("/api/fixed-costs")
            .service(get_fixed_costs_by_cost_type)
            .service(create_fixed_cost)
            .service(update_fixed_cost)
            .service(delete_fixed_cost),
    );
}

// Helper function to load allocation methods for a cost type
fn load_allocation_methods_for_cost_type(
    cost_type_id_val: i32,
    conn: &mut SqliteConnection,
) -> Vec<AllocationMethodDto> {
    use crate::schema::allocation_methods::dsl::*;
    use crate::schema::cost_type_allocations::dsl::*;

    match cost_type_allocations
        .filter(cost_type_id.eq(cost_type_id_val))
        .inner_join(allocation_methods)
        .select(allocation_methods::all_columns())
        .load::<AllocationMethod>(conn)
    {
        Ok(methods) => methods.into_iter().map(|m| m.into()).collect(),
        Err(e) => {
            error!(
                "Error loading allocation methods for cost type {}: {}",
                cost_type_id_val, e
            );
            Vec::new()
        }
    }
}

// GET /api/cost-types
#[get("")]
async fn get_all_cost_types(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::cost_types::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match cost_types.order_by(name.asc()).load::<CostType>(conn) {
        Ok(results) => {
            let mut cost_type_dtos: Vec<CostTypeDto> = Vec::new();

            for cost_type in results {
                let cost_type_id = cost_type.id.unwrap_or(0);
                let mut dto = CostTypeDto::from(cost_type);
                dto.allocation_methods = load_allocation_methods_for_cost_type(cost_type_id, conn);
                cost_type_dtos.push(dto);
            }

            HttpResponse::Ok().json(cost_type_dtos)
        }
        Err(e) => {
            error!("Error loading cost types: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading cost types: {}", e))
        }
    }
}

// GET /api/cost-types/{id}
#[get("/{id}")]
async fn get_cost_type_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::cost_types::dsl::*;

    let cost_type_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match cost_types
        .filter(id.eq(cost_type_id))
        .first::<CostType>(conn)
    {
        Ok(cost_type) => {
            let mut dto = CostTypeDto::from(cost_type);
            dto.allocation_methods = load_allocation_methods_for_cost_type(cost_type_id, conn);
            HttpResponse::Ok().json(dto)
        }
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Cost type with ID {} not found", cost_type_id))
        }
        Err(e) => {
            error!("Error finding cost type {}: {}", cost_type_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding cost type: {}", e))
        }
    }
}

// POST /api/cost-types
#[post("")]
async fn create_cost_type(
    new_cost_type_json: web::Json<NewCostType>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::cost_types::dsl::*;

    let conn = &mut db::get_connection(&pool);
    let new_cost_type = new_cost_type_json.0;

    // Input validation
    if new_cost_type.name.trim().is_empty() {
        return HttpResponse::BadRequest().json("Cost type name cannot be empty");
    }

    // If consumption-based, unit is required
    if new_cost_type.is_consumption_based && new_cost_type.unit.is_none() {
        return HttpResponse::BadRequest()
            .json("Unit is required for consumption-based cost types");
    }

    match diesel::insert_into(cost_types)
        .values(&new_cost_type)
        .execute(conn)
    {
        Ok(_) => {
            match cost_types.order_by(id.desc()).first::<CostType>(conn) {
                Ok(created_cost_type) => {
                    let _cost_type_id = created_cost_type.id.unwrap_or(0);
                    let mut dto = CostTypeDto::from(created_cost_type);
                    dto.allocation_methods = Vec::new(); // New cost type has no allocation methods yet
                    HttpResponse::Created().json(dto)
                }
                Err(e) => {
                    error!("Error retrieving created cost type: {}", e);
                    HttpResponse::InternalServerError().json(format!(
                        "Cost type created but could not be retrieved: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error creating cost type: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating cost type: {}", e))
        }
    }
}

// PUT /api/cost-types/{id}
#[put("/{id}")]
async fn update_cost_type(
    path: web::Path<i32>,
    update_json: web::Json<CostTypeUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::cost_types::dsl::*;

    let cost_type_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);
    let update = update_json.0;

    // Input validation
    if let Some(ref name_val) = update.name {
        if name_val.trim().is_empty() {
            return HttpResponse::BadRequest().json("Cost type name cannot be empty");
        }
    }

    // Check if the cost type exists
    match cost_types
        .filter(id.eq(cost_type_id))
        .first::<CostType>(conn)
    {
        Ok(existing_cost_type) => {
            // If is_consumption_based is being updated to true, make sure unit is provided
            let will_be_consumption_based = update
                .is_consumption_based
                .unwrap_or(existing_cost_type.is_consumption_based);
            let unit_will_be_present = update.unit.is_some() || existing_cost_type.unit.is_some();

            if will_be_consumption_based && !unit_will_be_present {
                return HttpResponse::BadRequest()
                    .json("Unit is required for consumption-based cost types");
            }

            match diesel::update(cost_types.filter(id.eq(cost_type_id)))
                .set(&update)
                .execute(conn)
            {
                Ok(_) => {
                    match cost_types
                        .filter(id.eq(cost_type_id))
                        .first::<CostType>(conn)
                    {
                        Ok(updated_cost_type) => {
                            let mut dto = CostTypeDto::from(updated_cost_type);
                            dto.allocation_methods =
                                load_allocation_methods_for_cost_type(cost_type_id, conn);
                            HttpResponse::Ok().json(dto)
                        }
                        Err(e) => {
                            error!("Error retrieving updated cost type: {}", e);
                            HttpResponse::InternalServerError().json(format!(
                                "Cost type updated but could not be retrieved: {}",
                                e
                            ))
                        }
                    }
                }
                Err(e) => {
                    error!("Error updating cost type {}: {}", cost_type_id, e);
                    HttpResponse::InternalServerError()
                        .json(format!("Error updating cost type: {}", e))
                }
            }
        }
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Cost type with ID {} not found", cost_type_id))
        }
        Err(e) => {
            error!("Error finding cost type {}: {}", cost_type_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding cost type: {}", e))
        }
    }
}

// DELETE /api/cost-types/{id}
#[delete("/{id}")]
async fn delete_cost_type(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::cost_types::dsl::*;

    let cost_type_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match diesel::delete(cost_types.filter(id.eq(cost_type_id))).execute(conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().json("Cost type deleted successfully")
            } else {
                HttpResponse::NotFound()
                    .json(format!("Cost type with ID {} not found", cost_type_id))
            }
        }
        Err(e) => {
            error!("Error deleting cost type {}: {}", cost_type_id, e);
            HttpResponse::InternalServerError().json(format!("Error deleting cost type: {}", e))
        }
    }
}

// GET /api/cost-types/allocation-methods
#[get("/allocation-methods")]
async fn get_all_allocation_methods(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::allocation_methods::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match allocation_methods
        .order_by(name.asc())
        .load::<AllocationMethod>(conn)
    {
        Ok(results) => {
            let dtos: Vec<AllocationMethodDto> = results.into_iter().map(|m| m.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading allocation methods: {}", e);
            HttpResponse::InternalServerError()
                .json(format!("Error loading allocation methods: {}", e))
        }
    }
}

// POST /api/cost-types/{id}/allocation-methods/{method_id}
#[post("/{id}/allocation-methods/{method_id}")]
async fn assign_allocation_method(
    path: web::Path<(i32, i32)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::cost_type_allocations::dsl::*;

    let (cost_type_id_val, allocation_method_id_val) = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if assignment already exists
    match cost_type_allocations
        .filter(crate::schema::cost_type_allocations::cost_type_id.eq(cost_type_id_val))
        .filter(
            crate::schema::cost_type_allocations::allocation_method_id.eq(allocation_method_id_val),
        )
        .first::<CostTypeAllocation>(conn)
    {
        Ok(_) => {
            // Already exists
            let methods = load_allocation_methods_for_cost_type(cost_type_id_val, conn);
            HttpResponse::Ok().json(methods)
        }
        Err(diesel::NotFound) => {
            // Create new assignment
            let new_allocation = NewCostTypeAllocation {
                cost_type_id: cost_type_id_val,
                allocation_method_id: allocation_method_id_val,
            };

            match diesel::insert_into(cost_type_allocations)
                .values(&new_allocation)
                .execute(conn)
            {
                Ok(_) => {
                    let methods = load_allocation_methods_for_cost_type(cost_type_id_val, conn);
                    HttpResponse::Created().json(methods)
                }
                Err(e) => {
                    error!("Error assigning allocation method: {}", e);
                    HttpResponse::InternalServerError()
                        .json(format!("Error assigning allocation method: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Error checking allocation method assignment: {}", e);
            HttpResponse::InternalServerError().json(format!(
                "Error checking allocation method assignment: {}",
                e
            ))
        }
    }
}

// DELETE /api/cost-types/{id}/allocation-methods/{method_id}
#[delete("/{id}/allocation-methods/{method_id}")]
async fn remove_allocation_method(
    path: web::Path<(i32, i32)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::cost_type_allocations::dsl::*;

    let (cost_type_id_val, allocation_method_id_val) = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match diesel::delete(
        cost_type_allocations
            .filter(crate::schema::cost_type_allocations::cost_type_id.eq(cost_type_id_val))
            .filter(
                crate::schema::cost_type_allocations::allocation_method_id
                    .eq(allocation_method_id_val),
            ),
    )
    .execute(conn)
    {
        Ok(count) => {
            if count > 0 {
                let methods = load_allocation_methods_for_cost_type(cost_type_id_val, conn);
                HttpResponse::Ok().json(methods)
            } else {
                HttpResponse::NotFound().json("Allocation method not assigned to this cost type")
            }
        }
        Err(e) => {
            error!("Error removing allocation method: {}", e);
            HttpResponse::InternalServerError()
                .json(format!("Error removing allocation method: {}", e))
        }
    }
}

// GET /api/tariffs/cost-type/{id}
#[get("/cost-type/{id}")]
async fn get_tariffs_by_cost_type(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::tariffs::dsl::*;

    let cost_type_id_param = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match tariffs
        .filter(cost_type_id.eq(cost_type_id_param))
        .order_by(valid_from.desc())
        .load::<Tariff>(conn)
    {
        Ok(results) => {
            let dtos: Vec<TariffDto> = results.into_iter().map(|t| t.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading tariffs for cost type {}: {}",
                cost_type_id_param, e
            );
            HttpResponse::InternalServerError().json(format!("Error loading tariffs: {}", e))
        }
    }
}

// POST /api/tariffs
#[post("")]
async fn create_tariff(
    new_tariff_json: web::Json<NewTariff>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::tariffs::dsl::*;

    let conn = &mut db::get_connection(&pool);
    let new_tariff = new_tariff_json.0;

    // Input validation
    if new_tariff.price_per_unit <= 0.0 {
        return HttpResponse::BadRequest().json("Price per unit must be greater than 0");
    }

    match diesel::insert_into(tariffs)
        .values(&new_tariff)
        .execute(conn)
    {
        Ok(_) => match tariffs.order_by(id.desc()).first::<Tariff>(conn) {
            Ok(created_tariff) => HttpResponse::Created().json(TariffDto::from(created_tariff)),
            Err(e) => {
                error!("Error retrieving created tariff: {}", e);
                HttpResponse::InternalServerError()
                    .json(format!("Tariff created but could not be retrieved: {}", e))
            }
        },
        Err(e) => {
            error!("Error creating tariff: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating tariff: {}", e))
        }
    }
}

// PUT /api/tariffs/{id}
#[put("/{id}")]
async fn update_tariff(
    path: web::Path<i32>,
    update_json: web::Json<TariffUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::tariffs::dsl::*;

    let tariff_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);
    let update = update_json.0;

    // Input validation
    if let Some(price) = update.price_per_unit {
        if price <= 0.0 {
            return HttpResponse::BadRequest().json("Price per unit must be greater than 0");
        }
    }

    match diesel::update(tariffs.filter(id.eq(tariff_id)))
        .set(&update)
        .execute(conn)
    {
        Ok(count) => {
            if count > 0 {
                match tariffs.filter(id.eq(tariff_id)).first::<Tariff>(conn) {
                    Ok(updated_tariff) => HttpResponse::Ok().json(TariffDto::from(updated_tariff)),
                    Err(e) => {
                        error!("Error retrieving updated tariff: {}", e);
                        HttpResponse::InternalServerError()
                            .json(format!("Tariff updated but could not be retrieved: {}", e))
                    }
                }
            } else {
                HttpResponse::NotFound().json(format!("Tariff with ID {} not found", tariff_id))
            }
        }
        Err(e) => {
            error!("Error updating tariff {}: {}", tariff_id, e);
            HttpResponse::InternalServerError().json(format!("Error updating tariff: {}", e))
        }
    }
}

// DELETE /api/tariffs/{id}
#[delete("/{id}")]
async fn delete_tariff(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::tariffs::dsl::*;

    let tariff_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match diesel::delete(tariffs.filter(id.eq(tariff_id))).execute(conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().json("Tariff deleted successfully")
            } else {
                HttpResponse::NotFound().json(format!("Tariff with ID {} not found", tariff_id))
            }
        }
        Err(e) => {
            error!("Error deleting tariff {}: {}", tariff_id, e);
            HttpResponse::InternalServerError().json(format!("Error deleting tariff: {}", e))
        }
    }
}

// GET /api/fixed-costs/cost-type/{id}
#[get("/cost-type/{id}")]
async fn get_fixed_costs_by_cost_type(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::fixed_costs::dsl::*;

    let cost_type_id_param = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match fixed_costs
        .filter(cost_type_id.eq(cost_type_id_param))
        .order_by(billing_period_end.desc())
        .load::<FixedCost>(conn)
    {
        Ok(results) => {
            let dtos: Vec<FixedCostDto> = results.into_iter().map(|c| c.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading fixed costs for cost type {}: {}",
                cost_type_id_param, e
            );
            HttpResponse::InternalServerError().json(format!("Error loading fixed costs: {}", e))
        }
    }
}

// POST /api/fixed-costs
#[post("")]
async fn create_fixed_cost(
    new_fixed_cost_json: web::Json<NewFixedCost>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::fixed_costs::dsl::*;

    let conn = &mut db::get_connection(&pool);
    let new_cost = new_fixed_cost_json.0;

    // Input validation
    if new_cost.amount <= 0.0 {
        return HttpResponse::BadRequest().json("Amount must be greater than 0");
    }

    if new_cost.billing_period_end < new_cost.billing_period_start {
        return HttpResponse::BadRequest().json("Billing period end date must be after start date");
    }

    match diesel::insert_into(fixed_costs)
        .values(&new_cost)
        .execute(conn)
    {
        Ok(_) => match fixed_costs.order_by(id.desc()).first::<FixedCost>(conn) {
            Ok(created_cost) => HttpResponse::Created().json(FixedCostDto::from(created_cost)),
            Err(e) => {
                error!("Error retrieving created fixed cost: {}", e);
                HttpResponse::InternalServerError().json(format!(
                    "Fixed cost created but could not be retrieved: {}",
                    e
                ))
            }
        },
        Err(e) => {
            error!("Error creating fixed cost: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating fixed cost: {}", e))
        }
    }
}

// PUT /api/fixed-costs/{id}
#[put("/{id}")]
async fn update_fixed_cost(
    path: web::Path<i32>,
    update_json: web::Json<FixedCostUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::fixed_costs::dsl::*;

    let fixed_cost_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);
    let update = update_json.0;

    // Get existing record for validation
    match fixed_costs
        .filter(id.eq(fixed_cost_id))
        .first::<FixedCost>(conn)
    {
        Ok(existing_cost) => {
            // Input validation
            if let Some(amount_val) = update.amount {
                if amount_val <= 0.0 {
                    return HttpResponse::BadRequest().json("Amount must be greater than 0");
                }
            }

            let start_date = update
                .billing_period_start
                .unwrap_or(existing_cost.billing_period_start);
            let end_date = update
                .billing_period_end
                .unwrap_or(existing_cost.billing_period_end);

            if end_date < start_date {
                return HttpResponse::BadRequest()
                    .json("Billing period end date must be after start date");
            }

            match diesel::update(fixed_costs.filter(id.eq(fixed_cost_id)))
                .set(&update)
                .execute(conn)
            {
                Ok(count) => {
                    if count > 0 {
                        match fixed_costs
                            .filter(id.eq(fixed_cost_id))
                            .first::<FixedCost>(conn)
                        {
                            Ok(updated_cost) => {
                                HttpResponse::Ok().json(FixedCostDto::from(updated_cost))
                            }
                            Err(e) => {
                                error!("Error retrieving updated fixed cost: {}", e);
                                HttpResponse::InternalServerError().json(format!(
                                    "Fixed cost updated but could not be retrieved: {}",
                                    e
                                ))
                            }
                        }
                    } else {
                        HttpResponse::NotFound()
                            .json(format!("Fixed cost with ID {} not found", fixed_cost_id))
                    }
                }
                Err(e) => {
                    error!("Error updating fixed cost {}: {}", fixed_cost_id, e);
                    HttpResponse::InternalServerError()
                        .json(format!("Error updating fixed cost: {}", e))
                }
            }
        }
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Fixed cost with ID {} not found", fixed_cost_id))
        }
        Err(e) => {
            error!("Error finding fixed cost {}: {}", fixed_cost_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding fixed cost: {}", e))
        }
    }
}

// DELETE /api/fixed-costs/{id}
#[delete("/{id}")]
async fn delete_fixed_cost(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::fixed_costs::dsl::*;

    let fixed_cost_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match diesel::delete(fixed_costs.filter(id.eq(fixed_cost_id))).execute(conn) {
        Ok(count) => {
            if count > 0 {
                HttpResponse::Ok().json("Fixed cost deleted successfully")
            } else {
                HttpResponse::NotFound()
                    .json(format!("Fixed cost with ID {} not found", fixed_cost_id))
            }
        }
        Err(e) => {
            error!("Error deleting fixed cost {}: {}", fixed_cost_id, e);
            HttpResponse::InternalServerError().json(format!("Error deleting fixed cost: {}", e))
        }
    }
}
