use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::backend::Backend;
use diesel::BelongingToDsl;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::sqlite::Sqlite;
use diesel::sql_types::{Integer, Bool, Float, Text, Nullable, Timestamp};
use diesel::sql_query;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::db;
use crate::DbPool;
use crate::models::billing::{BillingPeriod, NewBillingPeriod, BillingStatement, NewBillingStatement, GenerateStatementRequest};
use crate::models::property_unit::PropertyUnit;
use crate::models::tenant::Tenant;
use crate::models::meter_reading::MeterReading;
use crate::models::meter::Meter;
use crate::models::cost::{CostType, FixedCost, Tariff};
use crate::schema::{billing_periods, billing_statements, property_units, tenants, meters, meter_readings, cost_types, fixed_costs, tariffs};

// Define a struct to hold SQL count result
#[derive(QueryableByName, Debug)]
struct SqliteBindedStatementCount {
    #[diesel(sql_type = Integer)]
    count: i32,
}

impl<DB> FromSql<Integer, DB> for SqliteBindedStatementCount
where
    DB: diesel::backend::Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: diesel::backend::RawValue<DB>) -> deserialize::Result<Self> {
        let count = i32::from_sql(bytes)?;
        Ok(SqliteBindedStatementCount { count })
    }
}

// Billing Period CRUD Operations

#[get("/billing-periods")]
pub async fn get_billing_periods() -> impl Responder {
    use crate::schema::billing_periods::dsl::*;

    let conn = &mut db::establish_connection();
    match billing_periods.load::<BillingPeriod>(conn) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(e) => {
            eprintln!("Error loading billing periods: {:?}", e);
            HttpResponse::InternalServerError().body("Error loading billing periods")
        }
    }
}

#[get("/billing-periods/{id}")]
pub async fn get_billing_period(path: web::Path<i32>) -> impl Responder {
    let period_id = path.into_inner();
    let conn = &mut db::establish_connection();

    match billing_periods::table
        .filter(billing_periods::id.eq(period_id))
        .first::<BillingPeriod>(conn)
    {
        Ok(period) => HttpResponse::Ok().json(period),
        Err(_) => HttpResponse::NotFound().body("Billing period not found"),
    }
}

#[post("/billing-periods")]
pub async fn create_billing_period(
    pool: web::Data<DbPool>,
    new_period_json: web::Json<NewBillingPeriod>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_period = new_period_json.into_inner();

    // First database operation - check for overlaps
    // We need to clone the pool for the first operation
    let pool_clone = pool.clone();
    let period_unit_id = new_period.property_unit_id;
    let start_date = new_period.start_date.clone();
    let end_date = new_period.end_date.clone();

    // Check for overlapping billing periods
    let overlapping_periods_query = diesel::sql_query(
        "SELECT COUNT(*) as count FROM billing_periods \
         WHERE property_unit_id = ?1 AND \
         (start_date < ?3 AND end_date > ?2)",
    )
    .bind::<Integer, _>(period_unit_id)
    .bind::<Text, _>(start_date)
    .bind::<Text, _>(end_date);

    // Use proper error handling with map_err
    let statement_count = match web::block(move || {
        let mut conn = pool_clone.get().expect("couldn't get db connection from pool");
        overlapping_periods_query.get_result::<SqliteBindedStatementCount>(&mut conn)
    })
    .await
    .map_err(|e| {
        eprintln!("Error checking overlapping periods: {:?}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    }) {
        Ok(result) => match result {
            Ok(count) => count,
            Err(e) => {
                eprintln!("Database error: {:?}", e);
                return Err(actix_web::error::ErrorInternalServerError("Database error"));
            }
        },
        Err(e) => return Err(e),
    };

    if statement_count.count > 0 {
        return Ok(HttpResponse::BadRequest().body("Overlapping billing periods found"));
    }

    // Validate date format
    if let Err(_) = NaiveDate::parse_from_str(&new_period.start_date, "%Y-%m-%d") {
        return Ok(HttpResponse::BadRequest().body("Invalid start_date format. Use YYYY-MM-DD"));
    }

    if let Err(_) = NaiveDate::parse_from_str(&new_period.end_date, "%Y-%m-%d") {
        return Ok(HttpResponse::BadRequest().body("Invalid end_date format. Use YYYY-MM-DD"));
    }

    // Create current timestamp for created_at and updated_at fields
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    // Insert the new billing period
    let new_period_record = (
        billing_periods::property_unit_id.eq(new_period.property_unit_id),
        billing_periods::start_date.eq(&new_period.start_date),
        billing_periods::end_date.eq(&new_period.end_date),
        billing_periods::name.eq(&new_period.name),
        billing_periods::created_at.eq(&now),
        billing_periods::updated_at.eq(&now),
    );

    let mut conn = pool.get().expect("couldn't get db connection from pool");
    match diesel::insert_into(billing_periods::table)
        .values(new_period_record)
        .execute(&mut conn)
    {
        Ok(_) => {
            // Get the newly created billing period
            match billing_periods::table
                .filter(billing_periods::property_unit_id.eq(new_period.property_unit_id))
                .filter(billing_periods::start_date.eq(&new_period.start_date))
                .filter(billing_periods::end_date.eq(&new_period.end_date))
                .first::<BillingPeriod>(&mut conn)
            {
                Ok(period) => Ok(HttpResponse::Created().json(period)),
                Err(e) => {
                    eprintln!("Error retrieving created billing period: {:?}", e);
                    Ok(HttpResponse::InternalServerError().body("Error retrieving created billing period"))
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating billing period: {:?}", e);
            Ok(HttpResponse::InternalServerError().body("Error creating billing period"))
        }
    }
}

#[put("/billing-periods/{id}")]
pub async fn update_billing_period(path: web::Path<i32>, updated_period: web::Json<NewBillingPeriod>) -> impl Responder {
    let period_id = path.into_inner();
    let conn = &mut db::establish_connection();

    // Validate that the billing period exists
    match billing_periods::table
        .filter(billing_periods::id.eq(period_id))
        .first::<BillingPeriod>(conn)
    {
        Ok(_) => {},
        Err(_) => return HttpResponse::NotFound().body("Billing period not found"),
    }

    // Validate that the property unit exists
    match property_units::table
        .filter(property_units::id.eq(updated_period.property_unit_id))
        .first::<PropertyUnit>(conn)
    {
        Ok(_) => {},
        Err(_) => return HttpResponse::BadRequest().body("Property unit not found"),
    }

    // Check for overlapping periods for the same property unit (excluding the current period)
    let overlap = diesel::sql_query("
        SELECT COUNT(*) as count FROM billing_periods
        WHERE property_unit_id = ? AND id != ?
        AND ((start_date <= ? AND end_date >= ?) OR (start_date <= ? AND end_date >= ?))
    ")
    .bind::<diesel::sql_types::Integer, _>(updated_period.property_unit_id)
    .bind::<diesel::sql_types::Integer, _>(period_id)
    .bind::<diesel::sql_types::Text, _>(&updated_period.end_date)
    .bind::<diesel::sql_types::Text, _>(&updated_period.start_date)
    .bind::<diesel::sql_types::Text, _>(&updated_period.start_date)
    .bind::<diesel::sql_types::Text, _>(&updated_period.end_date)
    .get_result::<SqliteBindedStatementCount>(conn);

    match overlap {
        Ok(result) => {
            if result.count > 0 {
                return HttpResponse::BadRequest().body("This period overlaps with an existing billing period for this property unit");
            }
        },
        Err(e) => {
            eprintln!("Error checking for overlapping periods: {:?}", e);
            return HttpResponse::InternalServerError().body("Error checking for overlapping periods");
        }
    }

    // Create current timestamp for updated_at field
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    match diesel::update(billing_periods::table.filter(billing_periods::id.eq(period_id)))
        .set((
            billing_periods::property_unit_id.eq(updated_period.property_unit_id),
            billing_periods::start_date.eq(&updated_period.start_date),
            billing_periods::end_date.eq(&updated_period.end_date),
            billing_periods::name.eq(&updated_period.name),
            billing_periods::updated_at.eq(&now),
        ))
        .execute(conn)
    {
        Ok(_) => match billing_periods::table.filter(billing_periods::id.eq(period_id)).first::<BillingPeriod>(conn) {
            Ok(period) => HttpResponse::Ok().json(period),
            Err(e) => {
                eprintln!("Error retrieving updated billing period: {:?}", e);
                HttpResponse::InternalServerError().body("Error retrieving updated billing period")
            }
        },
        Err(e) => {
            eprintln!("Error updating billing period: {:?}", e);
            HttpResponse::InternalServerError().body("Error updating billing period")
        }
    }
}

#[delete("/billing-periods/{id}")]
pub async fn delete_billing_period(path: web::Path<i32>) -> impl Responder {
    let period_id = path.into_inner();
    let conn = &mut db::establish_connection();

    match diesel::delete(billing_periods::table.filter(billing_periods::id.eq(period_id))).execute(conn) {
        Ok(0) => HttpResponse::NotFound().body("Billing period not found"),
        Ok(_) => HttpResponse::Ok().body("Billing period deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting billing period: {:?}", e);
            HttpResponse::InternalServerError().body("Error deleting billing period")
        }
    }
}

#[post("/billing-statements/generate")]
pub async fn generate_billing_statement(request: web::Json<GenerateStatementRequest>) -> impl Responder {
    let conn = &mut db::establish_connection();

    // Validate that the billing period exists
    let billing_period = match billing_periods::table
        .filter(billing_periods::id.eq(request.billing_period_id))
        .first::<BillingPeriod>(conn)
    {
        Ok(period) => period,
        Err(_) => return HttpResponse::NotFound().body("Billing period not found"),
    };

    // Validate that the tenant exists
    let tenant = match tenants::table
        .filter(tenants::id.eq(request.tenant_id))
        .first::<Tenant>(conn)
    {
        Ok(tenant) => tenant,
        Err(_) => return HttpResponse::NotFound().body("Tenant not found"),
    };

    // Calculate costs for this tenant and billing period
    let total_amount = match calculate_tenant_costs(conn, &billing_period, &tenant) {
        Ok(amount) => amount,
        Err(e) => {
            eprintln!("Error calculating tenant costs: {:?}", e);
            return HttpResponse::InternalServerError().body("Error calculating tenant costs");
        }
    };

    // Generate HTML content
    let html_content = generate_billing_statement_html(&billing_period, &tenant, total_amount, conn);

    // Create current timestamp for generated_at field
    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%d %H:%M:%S").to_string();

    // Create a new statement
    let new_statement = NewBillingStatement {
        billing_period_id: request.billing_period_id,
        tenant_id: request.tenant_id,
        total_amount,
        generated_at: now.clone(),
        html_content: Some(html_content),
    };

    match diesel::insert_into(billing_statements::table)
        .values(&new_statement)
        .execute(conn)
    {
        Ok(_) => {
            // Get the newly created statement
            match billing_statements::table
                .filter(billing_statements::billing_period_id.eq(request.billing_period_id))
                .filter(billing_statements::tenant_id.eq(request.tenant_id))
                .filter(billing_statements::generated_at.eq(&now))
                .first::<BillingStatement>(conn)
            {
                Ok(statement) => HttpResponse::Created().json(statement),
                Err(e) => {
                    eprintln!("Error retrieving created billing statement: {:?}", e);
                    HttpResponse::InternalServerError().body("Error retrieving created billing statement")
                }
            }
        }
        Err(e) => {
            eprintln!("Error creating billing statement: {:?}", e);
            HttpResponse::InternalServerError().body("Error creating billing statement")
        }
    }
}

#[get("/billing-statements/{id}")]
pub async fn get_billing_statement(path: web::Path<i32>) -> impl Responder {
    let statement_id = path.into_inner();
    let conn = &mut db::establish_connection();

    match billing_statements::table
        .filter(billing_statements::id.eq(statement_id))
        .first::<BillingStatement>(conn)
    {
        Ok(statement) => HttpResponse::Ok().json(statement),
        Err(_) => HttpResponse::NotFound().body("Billing statement not found"),
    }
}

#[get("/billing-statements")]
pub async fn get_billing_statements() -> impl Responder {
    let conn = &mut db::establish_connection();

    match billing_statements::table
        .load::<BillingStatement>(conn)
    {
        Ok(statements) => HttpResponse::Ok().json(statements),
        Err(e) => {
            eprintln!("Error loading billing statements: {:?}", e);
            HttpResponse::InternalServerError().body("Error loading billing statements")
        }
    }
}

#[get("/billing-statements/tenant/{id}")]
pub async fn get_tenant_billing_statements(path: web::Path<i32>) -> impl Responder {
    let tenant_id = path.into_inner();
    let conn = &mut db::establish_connection();

    match billing_statements::table
        .filter(billing_statements::tenant_id.eq(tenant_id))
        .load::<BillingStatement>(conn)
    {
        Ok(statements) => HttpResponse::Ok().json(statements),
        Err(e) => {
            eprintln!("Error loading tenant billing statements: {:?}", e);
            HttpResponse::InternalServerError().body("Error loading tenant billing statements")
        }
    }
}

#[get("/billing-statements/html/{id}")]
pub async fn get_billing_statement_html(path: web::Path<i32>) -> impl Responder {
    let statement_id = path.into_inner();
    let conn = &mut db::establish_connection();

    match billing_statements::table
        .filter(billing_statements::id.eq(statement_id))
        .first::<BillingStatement>(conn)
    {
        Ok(statement) => {
            match statement.html_content {
                Some(html) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(html),
                None => HttpResponse::NotFound().body("HTML content not found for this statement")
            }
        },
        Err(_) => HttpResponse::NotFound().body("Billing statement not found"),
    }
}

#[delete("/billing-statements/{id}")]
pub async fn delete_billing_statement(path: web::Path<i32>) -> impl Responder {
    let statement_id = path.into_inner();
    let conn = &mut db::establish_connection();

    // Check if the statement exists
    match billing_statements::table
        .filter(billing_statements::id.eq(statement_id))
        .first::<BillingStatement>(conn)
    {
        Ok(_) => {},
        Err(_) => return HttpResponse::NotFound().body("Billing statement not found"),
    }

    // Delete the statement
    match diesel::delete(billing_statements::table.filter(billing_statements::id.eq(statement_id))).execute(conn) {
        Ok(_) => HttpResponse::Ok().body("Billing statement deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting billing statement: {:?}", e);
            HttpResponse::InternalServerError().body("Error deleting billing statement")
        }
    }
}

// Helper functions

fn calculate_tenant_costs(
    conn: &mut SqliteConnection,
    billing_period: &BillingPeriod,
    tenant: &Tenant,
) -> Result<f32, diesel::result::Error> {
    let (start_date, end_date) = billing_period.to_naive_date_range();
    let mut total_cost = 0.0;

    // Get the property unit
    let property_unit = property_units::table
        .filter(property_units::id.eq(billing_period.property_unit_id))
        .first::<PropertyUnit>(conn)?;

    // Get all cost types
    let all_cost_types = cost_types::table.load::<CostType>(conn)?;

    // For each cost type, calculate the tenant's share
    for cost_type in all_cost_types {
        if cost_type.is_consumption_based {
            // Handle consumption-based costs
            // Calculate costs allocated by area proportion
            let total_area = property_unit.living_area_m2 as f64;
            let tenant_area = total_area; // Assuming the tenant occupies the entire unit

            if total_area > 0.0 {
                // Get all meters for the property unit
                let meters_for_unit = meters::table
                    .filter(meters::property_unit_id.eq(property_unit.id))
                    .select(Meter::as_select())
                    .load::<Meter>(conn)?;

                for meter in meters_for_unit {
                    if let Some(meter_id) = meter.id {
                        // Convert NaiveDate to NaiveDateTime for comparisons with reading_date (Timestamp)
                        let start_datetime = NaiveDateTime::new(
                            start_date,
                            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                        );

                        let start_reading_val = meter_readings::table
                            .filter(meter_readings::meter_id.eq(meter_id))
                            .filter(meter_readings::reading_date.ge(start_datetime))
                            .order(meter_readings::reading_date.asc())
                            .select(meter_readings::value)
                            .first::<f32>(conn)
                            .optional()?;

                        // Convert NaiveDate to NaiveDateTime for comparisons
                        let end_datetime = NaiveDateTime::new(
                            end_date,
                            chrono::NaiveTime::from_hms_opt(23, 59, 59).unwrap()
                        );

                        let end_reading_val = meter_readings::table
                            .filter(meter_readings::meter_id.eq(meter_id))
                            .filter(meter_readings::reading_date.le(end_datetime))
                            .order(meter_readings::reading_date.desc())
                            .select(meter_readings::value)
                            .first::<f32>(conn)
                            .optional()?;

                        if let (Some(start_reading), Some(end_reading)) = (start_reading_val, end_reading_val) {
                            let consumption = end_reading - start_reading;

                            // Get tariff for the meter type and billing period
                            let tariffs_for_cost_type = tariffs::table
                                .filter(tariffs::cost_type_id.eq(cost_type.id.unwrap())) // Assuming cost_type.id is Option<i32>
                                // We'll filter dates after loading due to type conversion complexities
                                .load::<Tariff>(conn)?
                                .into_iter()
                                .filter(|tariff| {
                                    // Check if tariff is valid for the billing period
                                    tariff.valid_from <= end_date &&
                                    (tariff.valid_to.is_none() || tariff.valid_to.unwrap() >= start_date)
                                })
                                .collect::<Vec<Tariff>>();

                            if let Some(tariff) = tariffs_for_cost_type.first() {
                                total_cost += consumption * tariff.price_per_unit;
                            }
                        }
                    }
                }
            }
        } else {
            // Handle fixed costs
            // Get fixed costs for this cost type during the billing period
            let fixed_costs_for_type = fixed_costs::table
                .filter(fixed_costs::cost_type_id.eq(cost_type.id.unwrap())) // Assuming cost_type.id is Option<i32>
                .load::<FixedCost>(conn)?
                .into_iter()
                .filter(|fixed_cost| {
                    // Check if fixed cost period overlaps with billing period
                    fixed_cost.billing_period_start <= end_date &&
                    fixed_cost.billing_period_end >= start_date
                })
                .collect::<Vec<FixedCost>>();

            for fixed_cost in fixed_costs_for_type {
                // By default, distribute by area
                let total_area = property_unit.living_area_m2 as f64;

                // Get all tenants in the property unit to calculate proportions
                let tenants_in_unit = tenants::table
                    .filter(tenants::property_unit_id.eq(property_unit.id.unwrap())) // Assuming property_unit.id is Option<i32>
                    .select(Tenant::as_select())
                    .load::<Tenant>(conn)?;

                let number_of_tenants_in_unit = tenants_in_unit.len() as f32;

                if number_of_tenants_in_unit > 0.0 {
                    total_cost += fixed_cost.amount / number_of_tenants_in_unit;
                }
            }
        }
    }

    Ok(total_cost)
}

fn generate_billing_statement_html(billing_period: &BillingPeriod, tenant: &Tenant, total_amount: f32, conn: &mut SqliteConnection) -> String {
    // Get the tenant's property unit living area
    let area = property_units::table
                .filter(property_units::id.eq(tenant.property_unit_id))
                .select(property_units::living_area_m2)
                .first::<f32>(conn)
                .unwrap_or(0.0);

    format!(r###"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>Nebenkostenabrechnung</title>
        <style>
            body {{ font-family: Arial, sans-serif; margin: 40px; }}
            .header {{ text-align: center; margin-bottom: 30px; }}
            .info {{ margin-bottom: 20px; }}
            .total {{ margin-top: 30px; font-weight: bold; }}
        </style>
    </head>
    <body>
        <div class="header">
            <h1>Nebenkostenabrechnung</h1>
            <p>Abrechnungszeitraum: {start_date} bis {end_date}</p>
        </div>

        <div class="info">
            <h2>Mieter: {tenant_name}</h2>
            <p><strong>Personen:</strong> {persons}</p>
            <p><strong>Wohnfläche:</strong> {area} m²</p>
        </div>

        <div class="total">
            <p>Gesamtbetrag: {total_amount:.2} €</p>
        </div>
    </body>
    </html>
    "###,
    start_date = billing_period.start_date,
    end_date = billing_period.end_date,
    tenant_name = tenant.name,
    persons = tenant.number_of_persons,
    area = area,
    total_amount = total_amount)
}

// Configure function to register routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_billing_periods)
            .service(get_billing_period)
            .service(create_billing_period)
            .service(update_billing_period)
            .service(delete_billing_period)
            .service(generate_billing_statement)
            .service(get_billing_statement)
            .service(get_billing_statements)
            .service(get_tenant_billing_statements)
            .service(get_billing_statement_html)
            .service(delete_billing_statement)
    );
}
