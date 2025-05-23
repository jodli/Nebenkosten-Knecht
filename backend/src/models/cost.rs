use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{allocation_methods, cost_type_allocations, cost_types, fixed_costs, tariffs};

// Database model for cost types
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = cost_types)]
pub struct CostType {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub is_consumption_based: bool,
    pub unit: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New cost type data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = cost_types)]
pub struct NewCostType {
    pub name: String,
    pub description: Option<String>,
    pub is_consumption_based: bool,
    pub unit: Option<String>,
}

// Data transfer object for cost type updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = cost_types)]
pub struct CostTypeUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_consumption_based: Option<bool>,
    pub unit: Option<String>,
}

// Data transfer object for cost type responses
#[derive(Debug, Serialize, Deserialize)]
pub struct CostTypeDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_consumption_based: bool,
    pub unit: Option<String>,
    pub allocation_methods: Vec<AllocationMethodDto>,
}

// Database model for allocation methods
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = allocation_methods)]
pub struct AllocationMethod {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

// Data transfer object for allocation method responses
#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationMethodDto {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

// Junction table model for cost type allocations
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = cost_type_allocations)]
#[diesel(belongs_to(CostType))]
#[diesel(belongs_to(AllocationMethod))]
pub struct CostTypeAllocation {
    pub id: Option<i32>,
    pub cost_type_id: i32,
    pub allocation_method_id: i32,
}

// New cost type allocation for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = cost_type_allocations)]
pub struct NewCostTypeAllocation {
    pub cost_type_id: i32,
    pub allocation_method_id: i32,
}

// Database model for tariffs (consumption-based costs)
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = tariffs)]
#[diesel(belongs_to(CostType))]
pub struct Tariff {
    pub id: Option<i32>,
    pub cost_type_id: i32,
    pub price_per_unit: f32,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New tariff data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = tariffs)]
pub struct NewTariff {
    pub cost_type_id: i32,
    pub price_per_unit: f32,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

// Data transfer object for tariff updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = tariffs)]
pub struct TariffUpdate {
    pub price_per_unit: Option<f32>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<Option<NaiveDate>>,
}

// Data transfer object for tariff responses
#[derive(Debug, Serialize, Deserialize)]
pub struct TariffDto {
    pub id: i32,
    pub cost_type_id: i32,
    pub price_per_unit: f32,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
}

// Database model for fixed costs
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = fixed_costs)]
#[diesel(belongs_to(CostType))]
pub struct FixedCost {
    pub id: Option<i32>,
    pub cost_type_id: i32,
    pub amount: f32,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// New fixed cost data for insertions
#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = fixed_costs)]
pub struct NewFixedCost {
    pub cost_type_id: i32,
    pub amount: f32,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
}

// Data transfer object for fixed cost updates
#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = fixed_costs)]
pub struct FixedCostUpdate {
    pub amount: Option<f32>,
    pub billing_period_start: Option<NaiveDate>,
    pub billing_period_end: Option<NaiveDate>,
}

// Data transfer object for fixed cost responses
#[derive(Debug, Serialize, Deserialize)]
pub struct FixedCostDto {
    pub id: i32,
    pub cost_type_id: i32,
    pub amount: f32,
    pub billing_period_start: NaiveDate,
    pub billing_period_end: NaiveDate,
}

// Implementations for conversion between models and DTOs
impl From<CostType> for CostTypeDto {
    fn from(cost_type: CostType) -> Self {
        CostTypeDto {
            id: cost_type.id.unwrap_or(0),
            name: cost_type.name,
            description: cost_type.description,
            is_consumption_based: cost_type.is_consumption_based,
            unit: cost_type.unit,
            allocation_methods: Vec::new(), // Populated separately
        }
    }
}

impl From<AllocationMethod> for AllocationMethodDto {
    fn from(method: AllocationMethod) -> Self {
        AllocationMethodDto {
            id: method.id.unwrap_or(0),
            name: method.name,
            description: method.description,
        }
    }
}

impl From<Tariff> for TariffDto {
    fn from(tariff: Tariff) -> Self {
        TariffDto {
            id: tariff.id.unwrap_or(0),
            cost_type_id: tariff.cost_type_id,
            price_per_unit: tariff.price_per_unit,
            valid_from: tariff.valid_from,
            valid_to: tariff.valid_to,
        }
    }
}

impl From<FixedCost> for FixedCostDto {
    fn from(cost: FixedCost) -> Self {
        FixedCostDto {
            id: cost.id.unwrap_or(0),
            cost_type_id: cost.cost_type_id,
            amount: cost.amount,
            billing_period_start: cost.billing_period_start,
            billing_period_end: cost.billing_period_end,
        }
    }
}