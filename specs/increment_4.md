# Nebenkosten-Knecht - Increment 4 Implementation Plan

## Overview

Increment 4 focuses on the final step of the MVP: generating the annual Nebenkostenabrechnung (utility cost statement) for tenants. This increment covers billing period definition, core billing logic, and simple billing report generation.

---

## 1. Define Billing Period (M10)

### Backend
- Add a `billing_periods` table to the database.
- Implement API endpoints for creating, reading, updating, and deleting billing periods.
- Validate that billing periods do not overlap for the same property.

### Frontend
- Build a UI for managing billing periods (create, edit, delete, list).
- Add a date range selector component.
- Display validation errors for overlapping or invalid periods.

---

## 2. Core Billing Logic (M11)

### Backend
- Implement a calculation service that:
  - Retrieves all relevant meter readings for the selected period.
  - Calculates consumption for each meter.
  - Applies tariffs to consumption values.
  - Distributes costs according to allocation keys (area, persons, consumption, etc.).
  - Calculates tenant-specific totals and balances.
- Create API endpoints to trigger and retrieve calculations.
- Optimize performance (e.g., caching results).

### Frontend
- Build UI to trigger calculations and monitor status.
- Display calculated results in an overview (tables, charts).
- Show errors or warnings if data is missing or inconsistent.

---

## 3. Simple Billing Report Generation (M12)

### Backend
- Create an HTML template for the tenant bill.
- Implement an endpoint to generate a bill for a selected tenant and period.
- Allow retrieval of all bills for a given period.

### Frontend
- Build a bill preview and print component.
- Allow selection of tenant and billing period for report generation.
- Add print and simple export (PDF, print dialog) options.

---

## Technical Details
- Extend database schema for billing periods and statements.
- Use a Rust template engine for HTML report generation.
- Ensure robust error handling for calculation and reporting.
- Implement print-friendly CSS for reports in the frontend.

---

## Testing
- Unit tests for calculation logic.
- Integration tests for the billing workflow.
- Manual verification of generated statements.
- Test with various allocation and consumption scenarios.

---

## Timeline (Estimate)
- Billing Period Management: 1.5 weeks
- Core Calculation Logic: 3 weeks
- Report Generation: 1.5 weeks

---

## Next Steps
1. Design and implement the database schema for billing periods.
2. Create API endpoints for billing period management.
3. Build frontend components for managing periods.
4. Implement the core calculation service.
5. Develop the report generation and export features.
