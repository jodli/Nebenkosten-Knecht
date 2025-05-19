// @generated automatically by Diesel CLI.

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

diesel::joinable!(meters -> property_units (property_unit_id));
diesel::joinable!(tenants -> property_units (property_unit_id));

diesel::allow_tables_to_appear_in_same_query!(meters, property_units, tenants,);
