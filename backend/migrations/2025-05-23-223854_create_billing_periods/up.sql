-- Create the billing_periods table
CREATE TABLE billing_periods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    property_unit_id INTEGER NOT NULL,
    start_date TEXT NOT NULL, -- ISO 8601 format (YYYY-MM-DD)
    end_date TEXT NOT NULL,   -- ISO 8601 format (YYYY-MM-DD)
    name TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (property_unit_id) REFERENCES property_units (id) ON DELETE CASCADE,
    -- Ensure dates are valid
    CHECK (start_date <= end_date)
);

-- Index for faster lookups by property_unit
CREATE INDEX idx_billing_periods_property_unit ON billing_periods (property_unit_id);

-- Create a table to store the generated statements
CREATE TABLE billing_statements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    billing_period_id INTEGER NOT NULL,
    tenant_id INTEGER NOT NULL,
    total_amount REAL NOT NULL,
    generated_at TEXT NOT NULL DEFAULT (datetime('now')),
    html_content TEXT, -- Store the HTML representation of the statement
    FOREIGN KEY (billing_period_id) REFERENCES billing_periods (id) ON DELETE CASCADE,
    FOREIGN KEY (tenant_id) REFERENCES tenants (id) ON DELETE CASCADE
);

-- Index for faster statement lookups
CREATE INDEX idx_statements_period_tenant ON billing_statements (billing_period_id, tenant_id);