// @generated automatically by Diesel CLI.

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
    tenants (id) {
        id -> Nullable<Integer>,
        name -> Text,
        number_of_persons -> Integer,
        property_unit_id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(meter_readings -> meters (meter_id));
diesel::joinable!(meters -> property_units (property_unit_id));
diesel::joinable!(tenants -> property_units (property_unit_id));

diesel::allow_tables_to_appear_in_same_query!(
    meter_readings,
    meters,
    property_units,
    tenants,
);
