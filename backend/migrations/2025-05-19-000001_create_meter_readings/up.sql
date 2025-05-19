-- Create meter_readings table
CREATE TABLE meter_readings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    meter_id INTEGER NOT NULL,
    reading_date TIMESTAMP NOT NULL,
    value FLOAT NOT NULL,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (meter_id) REFERENCES meters(id) ON DELETE CASCADE
);

-- Create index for faster queries by meter
CREATE INDEX idx_meter_readings_meter_id ON meter_readings(meter_id);

-- Create index for faster ordering by date
CREATE INDEX idx_meter_readings_date ON meter_readings(reading_date);
