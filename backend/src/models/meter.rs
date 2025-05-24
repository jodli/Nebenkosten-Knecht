use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{db, schema::meters};

// Assignment type enum for meters
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MeterAssignment {
    Unit,
    Common,
}

impl ToString for MeterAssignment {
    fn to_string(&self) -> String {
        match self {
            MeterAssignment::Unit => "unit".to_string(),
            MeterAssignment::Common => "common".to_string(),
        }
    }
}

impl From<String> for MeterAssignment {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "unit" => MeterAssignment::Unit,
            "common" => MeterAssignment::Common,
            _ => MeterAssignment::Common, // Default to common for unknown values
        }
    }
}

// Database model for meters
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(table_name = meters)]
#[diesel(belongs_to(super::property_unit::PropertyUnit, foreign_key = property_unit_id))]
pub struct Meter {
    pub id: Option<i32>,
    pub name: String,
    pub meter_type: String,            // e.g., electricity, water, gas
    pub unit: String,                  // e.g., kWh, m³
    pub assignment_type: String,       // unit or common
    pub property_unit_id: Option<i32>, // Nullable for common meters
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New meter data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = meters)]
pub struct NewMeter {
    pub name: String,
    pub meter_type: String,
    pub unit: String,
    pub assignment_type: String,
    pub property_unit_id: Option<i32>,
}

// Data transfer object for meter updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = meters)]
pub struct MeterUpdate {
    pub name: Option<String>,
    pub meter_type: Option<String>,
    pub unit: Option<String>,
    pub assignment_type: Option<String>,
    pub property_unit_id: Option<Option<i32>>, // Double option for handling nulls
}

// Data transfer object for API responses
#[derive(Debug, Serialize, Deserialize)]
pub struct MeterDto {
    pub id: i32,
    pub name: String,
    pub meter_type: String,
    pub unit: String,
    pub assignment_type: MeterAssignment,
    pub property_unit_id: Option<i32>,
}

// DTO with additional validation for creating/updating
#[derive(Debug, Deserialize)]
pub struct MeterInputDto {
    pub name: String,
    pub meter_type: String,
    pub unit: String,
    pub assignment_type: MeterAssignment,
    pub property_unit_id: Option<i32>,
}

impl From<Meter> for MeterDto {
    fn from(meter: Meter) -> Self {
        MeterDto {
            id: meter.id.unwrap_or(0),
            name: meter.name,
            meter_type: meter.meter_type,
            unit: meter.unit,
            assignment_type: MeterAssignment::from(meter.assignment_type),
            property_unit_id: meter.property_unit_id,
        }
    }
}

impl From<MeterInputDto> for NewMeter {
    fn from(dto: MeterInputDto) -> Self {
        // Validate that unit meters have a property_unit_id
        let assignment_type = dto.assignment_type.to_string();
        let property_unit_id = match dto.assignment_type {
            MeterAssignment::Unit => dto.property_unit_id,
            MeterAssignment::Common => None, // Common meters don't need a property_unit_id
        };

        NewMeter {
            name: dto.name,
            meter_type: dto.meter_type,
            unit: dto.unit,
            assignment_type,
            property_unit_id,
        }
    }
}
