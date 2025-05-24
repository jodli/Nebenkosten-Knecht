use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::tenants;

// Database model for tenants
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(table_name = tenants)]
#[diesel(belongs_to(super::property_unit::PropertyUnit, foreign_key = property_unit_id))]
pub struct Tenant {
    pub id: Option<i32>,
    pub name: String,
    pub number_of_persons: i32,
    pub property_unit_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New tenant data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tenants)]
pub struct NewTenant {
    pub name: String,
    pub number_of_persons: i32,
    pub property_unit_id: i32,
}

// Data transfer object for tenant updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = tenants)]
pub struct TenantUpdate {
    pub name: Option<String>,
    pub number_of_persons: Option<i32>,
    pub property_unit_id: Option<i32>,
}

// Data transfer object for responses
#[derive(Debug, Serialize, Deserialize)]
pub struct TenantDto {
    pub id: i32,
    pub name: String,
    pub number_of_persons: i32,
    pub property_unit_id: i32,
}

impl From<Tenant> for TenantDto {
    fn from(tenant: Tenant) -> Self {
        TenantDto {
            id: tenant.id.unwrap_or(0),
            name: tenant.name,
            number_of_persons: tenant.number_of_persons,
            property_unit_id: tenant.property_unit_id,
        }
    }
}
