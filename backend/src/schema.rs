// @generated automatically by Diesel CLI.

diesel::table! {
    allocation_methods (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    cost_type_allocations (id) {
        id -> Nullable<Integer>,
        cost_type_id -> Integer,
        allocation_method_id -> Integer,
    }
}

diesel::table! {
    cost_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Nullable<Text>,
        is_consumption_based -> Bool,
        unit -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    fixed_costs (id) {
        id -> Nullable<Integer>,
        cost_type_id -> Integer,
        amount -> Float,
        billing_period_start -> Date,
        billing_period_end -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    meter_readings (id) {
        id -> Nullable<Integer>,
        meter_id -> Integer,
        reading_date -> Timestamp,
        value -> Float,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    meters (id) {
        id -> Nullable<Integer>,
        name -> Text,
        meter_type -> Text,
        unit -> Text,
        assignment_type -> Text,
        property_unit_id -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    property_units (id) {
        id -> Nullable<Integer>,
        name -> Text,
        living_area_m2 -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tariffs (id) {
        id -> Nullable<Integer>,
        cost_type_id -> Integer,
        price_per_unit -> Float,
        valid_from -> Date,
        valid_to -> Nullable<Date>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    tenants (id) {
        id -> Nullable<Integer>,
        name -> Text,
        number_of_persons -> Integer,
        property_unit_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(cost_type_allocations -> allocation_methods (allocation_method_id));
diesel::joinable!(cost_type_allocations -> cost_types (cost_type_id));
diesel::joinable!(fixed_costs -> cost_types (cost_type_id));
diesel::joinable!(meter_readings -> meters (meter_id));
diesel::joinable!(meters -> property_units (property_unit_id));
diesel::joinable!(tariffs -> cost_types (cost_type_id));
diesel::joinable!(tenants -> property_units (property_unit_id));

diesel::allow_tables_to_appear_in_same_query!(
    allocation_methods,
    cost_type_allocations,
    cost_types,
    fixed_costs,
    meter_readings,
    meters,
    property_units,
    tariffs,
    tenants,
);
