use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{billing_periods, billing_statements};
use crate::models::tenant::Tenant;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = billing_periods)]
pub struct BillingPeriod {
    pub id: Option<i32>,
    pub property_unit_id: i32,
    pub start_date: String,  // ISO 8601 format (YYYY-MM-DD)
    pub end_date: String,    // ISO 8601 format (YYYY-MM-DD)
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = billing_periods)]
pub struct NewBillingPeriod {
    pub property_unit_id: i32,
    pub start_date: String,  // ISO 8601 format (YYYY-MM-DD)
    pub end_date: String,    // ISO 8601 format (YYYY-MM-DD)
    pub name: String,
}

impl BillingPeriod {
    pub fn to_naive_date_range(&self) -> (NaiveDate, NaiveDate) {
        let start = NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap();
        let end = NaiveDate::parse_from_str(&self.end_date, "%Y-%m-%d").unwrap();
        (start, end)
    }
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, Serialize, Deserialize, Clone)]
#[diesel(belongs_to(BillingPeriod))]
#[diesel(belongs_to(Tenant))]
#[diesel(table_name = billing_statements)]
pub struct BillingStatement {
    pub id: Option<i32>,
    pub billing_period_id: i32,
    pub tenant_id: i32,
    pub total_amount: f32,
    pub generated_at: String,
    pub html_content: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = billing_statements)]
pub struct NewBillingStatement {
    pub billing_period_id: i32,
    pub tenant_id: i32,
    pub total_amount: f32,
    pub generated_at: String,
    pub html_content: Option<String>,
}

// Additional struct for API requests
#[derive(Deserialize, Debug)]
pub struct GenerateStatementRequest {
    pub billing_period_id: i32,
    pub tenant_id: i32,
}
