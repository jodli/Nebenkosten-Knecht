use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::meter::Meter;
use crate::schema::meter_readings;

// Database model for meter readings
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = meter_readings)]
#[diesel(belongs_to(Meter, foreign_key = meter_id))]
pub struct MeterReading {
    pub id: Option<i32>,
    pub meter_id: i32,
    pub reading_date: NaiveDateTime,
    pub value: f32,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New meter reading data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = meter_readings)]
pub struct NewMeterReading {
    pub meter_id: i32,
    pub reading_date: NaiveDateTime,
    pub value: f32,
    pub notes: Option<String>,
}

// Data transfer object for meter reading updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = meter_readings)]
pub struct MeterReadingUpdate {
    pub reading_date: Option<NaiveDateTime>,
    pub value: Option<f32>,
    pub notes: Option<Option<String>>, // Double option for handling nulls
}

// Data transfer object for API request
#[derive(Debug, Deserialize)]
pub struct MeterReadingInputDto {
    pub meter_id: i32,
    pub reading_date: NaiveDate, // Use just the date for simpler input
    pub value: f32,
    pub notes: Option<String>,
}

// Data transfer object for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct MeterReadingDto {
    pub id: i32,
    pub meter_id: i32,
    pub reading_date: NaiveDate, // Use just the date for simpler output
    pub value: f32,
    pub notes: Option<String>,
}

// Data transfer object for meter reading with consumption calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct MeterReadingWithConsumption {
    pub id: i32,
    pub meter_id: i32,
    pub reading_date: NaiveDate,
    pub value: f32,
    pub notes: Option<String>,
    pub consumption: Option<f32>, // Consumption since last reading
    pub days_since_last_reading: Option<i64>, // Days since last reading
}

impl From<MeterReading> for MeterReadingDto {
    fn from(reading: MeterReading) -> Self {
        MeterReadingDto {
            id: reading.id.unwrap_or(0),
            meter_id: reading.meter_id,
            reading_date: reading.reading_date.date(),
            value: reading.value,
            notes: reading.notes,
        }
    }
}

impl From<MeterReadingInputDto> for NewMeterReading {
    fn from(dto: MeterReadingInputDto) -> Self {
        // Convert date to datetime by setting the time to midnight
        let date_time = dto.reading_date.and_hms_opt(0, 0, 0).unwrap();

        NewMeterReading {
            meter_id: dto.meter_id,
            reading_date: date_time,
            value: dto.value,
            notes: dto.notes,
        }
    }
}
