-- Create cost types
CREATE TABLE cost_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    is_consumption_based BOOLEAN NOT NULL DEFAULT FALSE,
    unit TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create allocation methods
CREATE TABLE allocation_methods (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT
);

-- Create cost type allocations
CREATE TABLE cost_type_allocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cost_type_id INTEGER NOT NULL,
    allocation_method_id INTEGER NOT NULL,
    FOREIGN KEY (cost_type_id) REFERENCES cost_types(id) ON DELETE CASCADE,
    FOREIGN KEY (allocation_method_id) REFERENCES allocation_methods(id) ON DELETE CASCADE
);

-- Create tariffs for consumption-based costs
CREATE TABLE tariffs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cost_type_id INTEGER NOT NULL,
    price_per_unit REAL NOT NULL,
    valid_from DATE NOT NULL,
    valid_to DATE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cost_type_id) REFERENCES cost_types(id) ON DELETE CASCADE
);

-- Create fixed costs
CREATE TABLE fixed_costs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cost_type_id INTEGER NOT NULL,
    amount REAL NOT NULL,
    billing_period_start DATE NOT NULL,
    billing_period_end DATE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (cost_type_id) REFERENCES cost_types(id) ON DELETE CASCADE
);

-- Insert default allocation methods
INSERT INTO allocation_methods (name, description) VALUES
('LivingArea', 'Allocate costs based on living area in square meters'),
('PersonCount', 'Allocate costs based on number of persons'),
('Consumption', 'Allocate costs based on individual consumption'),
('EqualShare', 'Allocate costs equally among all units');