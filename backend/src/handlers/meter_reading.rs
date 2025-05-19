use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use log::{error, info};

use crate::db;
use crate::models::meter::Meter;
use crate::models::meter_reading::{
    MeterReading, MeterReadingDto, MeterReadingInputDto, MeterReadingUpdate,
    MeterReadingWithConsumption, NewMeterReading,
};
use crate::DbPool;

// Configure routes for meter readings
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/meter-readings")
            .service(get_all_readings)
            .service(get_reading_by_id)
            .service(get_readings_by_meter)
            .service(get_readings_by_date_range)
            .service(create_reading)
            .service(update_reading)
            .service(delete_reading)
            .service(calculate_consumption),
    );
}

// GET /api/meter-readings
#[get("")]
async fn get_all_readings(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;

    let conn = &mut db::get_connection(&pool);

    match meter_readings
        .order_by(reading_date.desc())
        .load::<MeterReading>(conn)
    {
        Ok(results) => {
            let dtos: Vec<MeterReadingDto> =
                results.into_iter().map(|reading| reading.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!("Error loading meter readings: {}", e);
            HttpResponse::InternalServerError().json(format!("Error loading meter readings: {}", e))
        }
    }
}

// GET /api/meter-readings/{id}
#[get("/{id}")]
async fn get_reading_by_id(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;

    let reading_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    match meter_readings
        .filter(id.eq(reading_id))
        .first::<MeterReading>(conn)
    {
        Ok(reading) => HttpResponse::Ok().json(MeterReadingDto::from(reading)),
        Err(diesel::NotFound) => {
            HttpResponse::NotFound().json(format!("Meter reading with ID {} not found", reading_id))
        }
        Err(e) => {
            error!("Error finding meter reading {}: {}", reading_id, e);
            HttpResponse::InternalServerError().json(format!("Error finding meter reading: {}", e))
        }
    }
}

// GET /api/meter-readings/by-meter/{meter_id}
#[get("/by-meter/{meter_id}")]
async fn get_readings_by_meter(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;
    use crate::schema::meters;

    let meter_id_val = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the meter exists
    match meters::table
        .filter(meters::id.eq(meter_id_val))
        .first::<Meter>(conn)
    {
        Ok(_) => (),
        Err(diesel::NotFound) => {
            return HttpResponse::NotFound()
                .json(format!("Meter with ID {} not found", meter_id_val));
        }
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    }

    match meter_readings
        .filter(meter_id.eq(meter_id_val))
        .order_by(reading_date.desc())
        .load::<MeterReading>(conn)
    {
        Ok(results) => {
            let dtos: Vec<MeterReadingDto> =
                results.into_iter().map(|reading| reading.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading meter readings for meter {}: {}",
                meter_id_val, e
            );
            HttpResponse::InternalServerError()
                .json(format!("Error loading meter readings for meter: {}", e))
        }
    }
}

// GET /api/meter-readings/by-date-range/{meter_id}/{start_date}/{end_date}
#[get("/by-date-range/{meter_id}/{start_date}/{end_date}")]
async fn get_readings_by_date_range(
    path: web::Path<(i32, String, String)>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;
    use crate::schema::meters;

    let (meter_id_val, start_date_str, end_date_str) = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Parse dates
    let start_date = match NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest().json(format!(
                "Invalid start date format: {}. Use YYYY-MM-DD",
                start_date_str
            ));
        }
    };

    let end_date = match NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest().json(format!(
                "Invalid end date format: {}. Use YYYY-MM-DD",
                end_date_str
            ));
        }
    };

    // Convert dates to datetime for database comparison
    let start_datetime = start_date.and_hms_opt(0, 0, 0).unwrap();
    let end_datetime = end_date.and_hms_opt(23, 59, 59).unwrap();

    // Check if the meter exists
    match meters::table
        .filter(meters::id.eq(meter_id_val))
        .first::<Meter>(conn)
    {
        Ok(_) => (),
        Err(diesel::NotFound) => {
            return HttpResponse::NotFound()
                .json(format!("Meter with ID {} not found", meter_id_val));
        }
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    }

    match meter_readings
        .filter(meter_id.eq(meter_id_val))
        .filter(reading_date.ge(start_datetime))
        .filter(reading_date.le(end_datetime))
        .order_by(reading_date.asc())
        .load::<MeterReading>(conn)
    {
        Ok(results) => {
            let dtos: Vec<MeterReadingDto> =
                results.into_iter().map(|reading| reading.into()).collect();
            HttpResponse::Ok().json(dtos)
        }
        Err(e) => {
            error!(
                "Error loading meter readings for meter {} between {} and {}: {}",
                meter_id_val, start_date_str, end_date_str, e
            );
            HttpResponse::InternalServerError().json(format!(
                "Error loading meter readings for date range: {}",
                e
            ))
        }
    }
}

// POST /api/meter-readings
#[post("")]
async fn create_reading(
    new_reading: web::Json<MeterReadingInputDto>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;
    use crate::schema::meters;

    let conn = &mut db::get_connection(&pool);

    // Input validation
    if new_reading.value < 0.0 {
        return HttpResponse::BadRequest().json("Reading value cannot be negative");
    }

    // Check if meter exists
    match meters::table
        .filter(meters::id.eq(new_reading.meter_id))
        .first::<Meter>(conn)
    {
        Ok(_) => (),
        Err(diesel::NotFound) => {
            return HttpResponse::BadRequest()
                .json(format!("Meter with ID {} not found", new_reading.meter_id));
        }
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    }

    // Check if a reading with this date already exists for this meter
    let reading_date_timestamp = new_reading.reading_date.and_hms_opt(0, 0, 0).unwrap();
    let existing_reading = meter_readings
        .filter(meter_id.eq(new_reading.meter_id))
        .filter(reading_date.eq(reading_date_timestamp))
        .first::<MeterReading>(conn)
        .optional();

    match existing_reading {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().json(format!(
                "A reading for meter ID {} on date {} already exists",
                new_reading.meter_id, new_reading.reading_date
            ));
        }
        Ok(None) => (), // No existing reading, proceed
        Err(e) => {
            error!("Error checking for existing reading: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking for existing reading: {}", e));
        }
    }

    // Check if the reading value is greater than the previous reading
    // Get the latest reading for this meter before the new reading date
    let latest_reading = meter_readings
        .filter(meter_id.eq(new_reading.meter_id))
        .filter(reading_date.lt(reading_date_timestamp))
        .order_by(reading_date.desc())
        .first::<MeterReading>(conn)
        .optional();

    match latest_reading {
        Ok(Some(prev_reading)) if prev_reading.value > new_reading.value => {
            return HttpResponse::BadRequest().json(format!(
                "New reading value ({}) is less than the previous reading value ({}) from {}",
                new_reading.value,
                prev_reading.value,
                prev_reading.reading_date.date()
            ));
        }
        Ok(_) => (), // No previous reading or value is higher, proceed
        Err(e) => {
            error!("Error checking previous reading: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking previous reading: {}", e));
        }
    }

    let new_reading = NewMeterReading::from(new_reading.into_inner());

    match diesel::insert_into(meter_readings)
        .values(&new_reading)
        .execute(conn)
    {
        Ok(_) => {
            // Get the newly created reading to return its ID
            match meter_readings
                .order_by(id.desc())
                .first::<MeterReading>(conn)
            {
                Ok(created_reading) => {
                    info!("Created meter reading: {:?}", created_reading);
                    HttpResponse::Created().json(MeterReadingDto::from(created_reading))
                }
                Err(e) => {
                    error!("Error retrieving created meter reading: {}", e);
                    HttpResponse::InternalServerError().json(format!(
                        "Meter reading created but error retrieving it: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error creating meter reading: {}", e);
            HttpResponse::InternalServerError().json(format!("Error creating meter reading: {}", e))
        }
    }
}

// PUT /api/meter-readings/{id}
#[put("/{id}")]
async fn update_reading(
    path: web::Path<i32>,
    reading_update: web::Json<MeterReadingUpdate>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;

    let reading_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Input validation
    if let Some(value_val) = reading_update.value {
        if value_val < 0.0 {
            return HttpResponse::BadRequest().json("Reading value cannot be negative");
        }
    }

    // Check if the reading exists
    let exists = match meter_readings
        .filter(id.eq(reading_id))
        .first::<MeterReading>(conn)
    {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if meter reading exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter reading exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound()
            .json(format!("Meter reading with ID {} not found", reading_id));
    }

    // Get the current reading
    let current_reading = meter_readings
        .filter(id.eq(reading_id))
        .first::<MeterReading>(conn)
        .unwrap();
    let meter_id_val = current_reading.meter_id;

    // If updating the date, check if a reading with the new date already exists for this meter
    if let Some(new_date) = reading_update.reading_date {
        let existing_reading = meter_readings
            .filter(meter_id.eq(meter_id_val))
            .filter(reading_date.eq(new_date))
            .filter(id.ne(reading_id)) // Exclude the current reading
            .first::<MeterReading>(conn)
            .optional();

        match existing_reading {
            Ok(Some(_)) => {
                return HttpResponse::BadRequest().json(format!(
                    "A reading for meter ID {} on date {} already exists",
                    meter_id_val,
                    new_date.date()
                ));
            }
            Ok(None) => (), // No existing reading, proceed
            Err(e) => {
                error!("Error checking for existing reading: {}", e);
                return HttpResponse::InternalServerError()
                    .json(format!("Error checking for existing reading: {}", e));
            }
        }
    }

    // If updating the value, check if the new value is greater than the previous reading
    // and less than the next reading
    if let Some(new_value) = reading_update.value {
        // Get the date we're checking against (either the updated date or the current date)
        let check_date = reading_update
            .reading_date
            .unwrap_or(current_reading.reading_date);

        // Check previous reading
        let prev_reading = meter_readings
            .filter(meter_id.eq(meter_id_val))
            .filter(reading_date.lt(check_date))
            .order_by(reading_date.desc())
            .first::<MeterReading>(conn)
            .optional();

        match prev_reading {
            Ok(Some(prev)) if prev.value > new_value => {
                return HttpResponse::BadRequest().json(format!(
                    "Updated reading value ({}) is less than the previous reading value ({}) from {}",
                    new_value,
                    prev.value,
                    prev.reading_date.date()
                ));
            }
            Ok(_) => (), // No previous reading or value is higher, proceed
            Err(e) => {
                error!("Error checking previous reading: {}", e);
                return HttpResponse::InternalServerError()
                    .json(format!("Error checking previous reading: {}", e));
            }
        }

        // Check next reading
        let next_reading = meter_readings
            .filter(meter_id.eq(meter_id_val))
            .filter(reading_date.gt(check_date))
            .order_by(reading_date.asc())
            .first::<MeterReading>(conn)
            .optional();

        match next_reading {
            Ok(Some(next)) if next.value < new_value => {
                return HttpResponse::BadRequest().json(format!(
                    "Updated reading value ({}) is greater than the next reading value ({}) from {}",
                    new_value,
                    next.value,
                    next.reading_date.date()
                ));
            }
            Ok(_) => (), // No next reading or value is lower, proceed
            Err(e) => {
                error!("Error checking next reading: {}", e);
                return HttpResponse::InternalServerError()
                    .json(format!("Error checking next reading: {}", e));
            }
        }
    }

    match diesel::update(meter_readings.filter(id.eq(reading_id)))
        .set(reading_update.into_inner())
        .execute(conn)
    {
        Ok(_) => {
            // Get the updated reading
            match meter_readings
                .filter(id.eq(reading_id))
                .first::<MeterReading>(conn)
            {
                Ok(updated_reading) => {
                    info!("Updated meter reading: {:?}", updated_reading);
                    HttpResponse::Ok().json(MeterReadingDto::from(updated_reading))
                }
                Err(e) => {
                    error!("Error retrieving updated meter reading: {}", e);
                    HttpResponse::InternalServerError().json(format!(
                        "Meter reading updated but error retrieving it: {}",
                        e
                    ))
                }
            }
        }
        Err(e) => {
            error!("Error updating meter reading: {}", e);
            HttpResponse::InternalServerError().json(format!("Error updating meter reading: {}", e))
        }
    }
}

// DELETE /api/meter-readings/{id}
#[delete("/{id}")]
async fn delete_reading(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;

    let reading_id = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the reading exists
    let exists = match meter_readings
        .filter(id.eq(reading_id))
        .first::<MeterReading>(conn)
    {
        Ok(_) => true,
        Err(diesel::NotFound) => false,
        Err(e) => {
            error!("Error checking if meter reading exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter reading exists: {}", e));
        }
    };

    if !exists {
        return HttpResponse::NotFound()
            .json(format!("Meter reading with ID {} not found", reading_id));
    }

    match diesel::delete(meter_readings.filter(id.eq(reading_id))).execute(conn) {
        Ok(_) => {
            info!("Deleted meter reading with ID: {}", reading_id);
            HttpResponse::NoContent().finish()
        }
        Err(e) => {
            error!("Error deleting meter reading {}: {}", reading_id, e);
            HttpResponse::InternalServerError().json(format!("Error deleting meter reading: {}", e))
        }
    }
}

// GET /api/meter-readings/consumption/{meter_id}
#[get("/consumption/{meter_id}")]
async fn calculate_consumption(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::meter_readings::dsl::*;
    use crate::schema::meters;

    let meter_id_val = path.into_inner();
    let conn = &mut db::get_connection(&pool);

    // Check if the meter exists
    let meter = match meters::table
        .filter(meters::id.eq(meter_id_val))
        .first::<Meter>(conn)
    {
        Ok(m) => m,
        Err(diesel::NotFound) => {
            return HttpResponse::NotFound()
                .json(format!("Meter with ID {} not found", meter_id_val));
        }
        Err(e) => {
            error!("Error checking if meter exists: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error checking if meter exists: {}", e));
        }
    };

    // Get all readings for this meter, ordered by date
    let readings = match meter_readings
        .filter(meter_id.eq(meter_id_val))
        .order_by(reading_date.asc())
        .load::<MeterReading>(conn)
    {
        Ok(r) => r,
        Err(e) => {
            error!("Error loading meter readings: {}", e);
            return HttpResponse::InternalServerError()
                .json(format!("Error loading meter readings: {}", e));
        }
    };

    if readings.is_empty() {
        return HttpResponse::Ok().json(Vec::<MeterReadingWithConsumption>::new());
    }

    // Calculate consumption between consecutive readings
    let mut result = Vec::new();
    let mut prev_reading: Option<&MeterReading> = None;

    for reading in &readings {
        let mut consumption = None;
        let mut days_since_last = None;

        if let Some(prev) = prev_reading {
            // Calculate consumption since previous reading
            consumption = Some(reading.value - prev.value);

            // Calculate days between readings
            let duration = reading
                .reading_date
                .signed_duration_since(prev.reading_date);
            days_since_last = Some(duration.num_days());
        }

        result.push(MeterReadingWithConsumption {
            id: reading.id.unwrap_or(0),
            meter_id: reading.meter_id,
            reading_date: reading.reading_date.date(),
            value: reading.value,
            notes: reading.notes.clone(),
            consumption,
            days_since_last_reading: days_since_last,
        });

        prev_reading = Some(reading);
    }

    HttpResponse::Ok().json(result)
}
