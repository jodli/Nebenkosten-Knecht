use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::property_units;

// Database model for property units
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = property_units)]
pub struct PropertyUnit {
    pub id: Option<i32>,
    pub name: String,
    pub living_area_m2: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New property unit data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = property_units)]
pub struct NewPropertyUnit {
    pub name: String,
    pub living_area_m2: f32,
}

// Data transfer object for property unit updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = property_units)]
pub struct PropertyUnitUpdate {
    pub name: Option<String>,
    pub living_area_m2: Option<f32>,
}

// Data transfer object for responses
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyUnitDto {
    pub id: i32,
    pub name: String,
    pub living_area_m2: f32,
}

impl From<PropertyUnit> for PropertyUnitDto {
    fn from(unit: PropertyUnit) -> Self {
        PropertyUnitDto {
            id: unit.id.unwrap_or(0),
            name: unit.name,
            living_area_m2: unit.living_area_m2,
        }
    }
}
