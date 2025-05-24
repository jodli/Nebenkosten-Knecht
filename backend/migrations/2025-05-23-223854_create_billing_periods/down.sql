-- Drop the tables and indexes created in up.sql
DROP INDEX IF EXISTS idx_statements_period_tenant;
DROP TABLE IF EXISTS billing_statements;
DROP INDEX IF EXISTS idx_billing_periods_property_unit;
DROP TABLE IF EXISTS billing_periods;
