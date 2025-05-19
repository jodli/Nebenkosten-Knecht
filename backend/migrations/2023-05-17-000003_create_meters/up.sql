-- Create meters table
CREATE TABLE meters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    meter_type VARCHAR NOT NULL,
    unit VARCHAR NOT NULL,
    assignment_type VARCHAR NOT NULL CHECK (assignment_type IN ('unit', 'common')),
    property_unit_id INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (property_unit_id) REFERENCES property_units(id) ON DELETE SET NULL
);
