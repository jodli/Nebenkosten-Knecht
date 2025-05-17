## Project Overview

*   **Product Name**: Nebenkosten-Knecht
*   **Core Purpose**: To solve the cumbersome, error-prone, and time-consuming manual process of tracking monthly utility meter readings, consolidating these with other annual shared costs, and generating an accurate annual utility cost statement (Nebenkostenabrechnung) for landlords. It aims to replace inefficient spreadsheet-based methods by providing automation, better data management, and streamlined report generation.
*   **MVP Goal**:
    *   Enable users to easily record their monthly utility meter readings, input other relevant annual shared costs, and automatically generate a basic, printable annual cost statement (Nebenkostenabrechnung) that clearly outlines the shared costs for themselves and their tenants.
    *   Successfully complete the core task of generating an accurate Nebenkostenabrechnung for at least one property and tenant.
    *   Demonstrate clear time savings and/or improved satisfaction compared to the user's previous manual methods.
*   **Target Audience**: Homeowners with an annexed apartment (Einliegerwohnung) or landlords with a small number of tenants (e.g., 1-5 tenants) who are responsible for creating annual utility cost statements (Nebenkostenabrechnungen) for their tenants.

## Technical Specifications (from Tech Design Doc)

*   **Platform**: Web app (intended for local deployment/use initially).
*   **Tech Stack (Frontend)**:
    *   Language: JavaScript (or TypeScript)
    *   Framework: Vue.js
    *   Styling: Tailwind CSS
*   **Tech Stack (Backend/Core)**:
    *   Language: Rust
    *   Framework: Actix-Web
    *   Database ORM: Diesel
    *   Database System: SQLite
*   **Key Libraries/APIs**:
    *   `cargo` (Rust's package manager and build tool)
    *   `npm` or `yarn` for JS package management
    *   Templating Engine for HTML report generation (e.g., Tera, Askama, or Handlebars-rust)
*   **Architecture Overview**:
    *   The system is a web application designed to run locally, consisting of a Vue.js frontend and a Rust/Actix-Web backend.
    *   Key Components: Frontend UI, Backend API (RESTful via Actix-Web), Data Management Module (Rust logic using Diesel ORM with SQLite), Consumption Calculation Engine (Rust), Cost Allocation Engine (Rust), Report Generation Module (Rust with a templating engine).
    *   Data Flow: User interacts with Vue.js frontend -> API calls to Actix-Web backend -> Backend processes requests, uses Data Management Module for DB interactions, and employs calculation/report engines -> Results/reports sent back to frontend.
*   **Data Handling Notes**:
    *   Storage: Data stored locally in an SQLite database file, managed by Diesel.
    *   Privacy/Security: Standard local file system security applies. No specific app-level encryption for MVP.
    *   Data Backup: The user is responsible for backing up their local database file.
*   **Error Handling Approach**:
    *   Provide clear, user-friendly error messages stating what went wrong, indicating problematic input/fields, and suggesting corrective actions.
    *   Backend (Rust/Actix-Web) will use `Result<T, E>` for internal error propagation; user-facing errors mapped to appropriate HTTP responses. Detailed technical errors logged server-side.
    *   Proactive input validation in frontend forms.

## Core MVP Features & Implementation Plan (from PRD & Tech Design Doc)

### Feature: M1: Property Unit Management
*   **Description**: Allow the landlord to create and manage property units (e.g., main apartment, tenant apartment) including details like name and living area (m²) for correct cost allocation.
*   **Key Acceptance Criteria/User Story**: Landlord can create, view, edit, and delete property units. Each unit must have a name and living area (m²). Data is persisted.
*   **Technical Implementation Notes**:
    *   Backend: Define database schema using Diesel migrations for property units. Create Rust structs for Units and implement Diesel query logic. Expose CRUD operations via Actix-Web handlers/routes.
    *   Frontend: Vue.js components with forms for creating/editing, and tables for listing units.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Define `PropertyUnit` model (struct) & database schema (migration for `property_units` table with columns: id, name, living_area_m2). Implement CRUD functions.
    2.  Backend (Actix-Web): Create API endpoints (e.g., `POST /units`, `GET /units`, `GET /units/{id}`, `PUT /units/{id}`, `DELETE /units/{id}`).
    3.  Frontend (Vue.js): Develop a component for listing property units and a form component for creating/editing units.
    4.  Frontend: Implement API calls from Vue components to backend endpoints for unit management.

### Feature: M2: Basic Tenant Management
*   **Description**: Allow the landlord to create tenants and assign them to a property unit, including the number of occupants for allocation keys based on person count.
*   **Key Acceptance Criteria/User Story**: Landlord can create, view, edit, and delete tenants. Each tenant must have a name, number of occupants, and be assignable to a property unit.
*   **Technical Implementation Notes**:
    *   Backend: Diesel schema/struct for Tenants (linked to PropertyUnit). Actix-Web handlers for CRUD.
    *   Frontend: Vue.js components for tenant forms and lists.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Define `Tenant` model & DB schema (migration for `tenants` table with columns: id, name, number_of_persons, property_unit_id (foreign key)). Implement CRUD functions.
    2.  Backend (Actix-Web): Expose CRUD API endpoints for tenants.
    3.  Frontend (Vue.js): Develop components for listing tenants and a form for creating/editing tenants (including assigning to a unit).
    4.  Frontend: Implement API calls for tenant management.

### Feature: M3: Meter Management
*   **Description**: Allow the landlord to define and manage various utility meters (e.g., electricity, water, oil) with their type, unit (kWh, m³), and assignment (to a unit or shared/common).
*   **Key Acceptance Criteria/User Story**: Landlord can create, view, edit, and delete meters. Each meter must have a type (e.g., electricity), unit (e.g., kWh), and an assignment (to a specific property unit or marked as 'common').
*   **Technical Implementation Notes**:
    *   Backend: Diesel schema/struct for Meters (type, unit, assignment - possibly linked to PropertyUnit or a common flag). Actix-Web handlers for CRUD.
    *   Frontend: Vue.js components for meter forms and lists.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Define `Meter` model & DB schema (migration for `meters` table with columns: id, type, unit, assignment_type (e.g., 'unit', 'common'), optional property_unit_id). Implement CRUD functions.
    2.  Backend (Actix-Web): Expose CRUD API endpoints for meters.
    3.  Frontend (Vue.js): Develop components for listing meters and a form for creating/editing meters.
    4.  Frontend: Implement API calls for meter management.

### Feature: M4: Manual Meter Reading Entry
*   **Description**: Enable the landlord to manually input meter readings with date and value for each meter.
*   **Key Acceptance Criteria/User Story**: Landlord can select a meter and input a new reading with a specific date and value. Readings should be associated with the correct meter.
*   **Technical Implementation Notes**:
    *   Backend: Diesel schema/struct for MeterReadings (linked to Meter, value, date). Actix-Web handler to save new readings. Validation in Rust: ensure reading date is chronologically consistent.
    *   Frontend: Form within the context of a selected meter to input new readings.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Define `MeterReading` model & DB schema (migration for `meter_readings` table with columns: id, meter_id (foreign key), date, value). Implement logic to add new readings.
    2.  Backend (Actix-Web): Create API endpoint to submit a new meter reading. Implement validation (e.g., date > last reading date for the meter).
    3.  Frontend (Vue.js): Within a meter's detail view, provide a form to add a new reading (date, value).
    4.  Frontend: Implement API call to submit the new reading.

### Feature: M5: View Meter Reading History
*   **Description**: Allow the landlord to view a list/history of recorded meter readings per meter for verification.
*   **Key Acceptance Criteria/User Story**: Landlord can select a meter and see all its recorded readings, ordered by date, for review and verification.
*   **Technical Implementation Notes**:
    *   Backend: Actix-Web handler to fetch all readings for a specific meter using Diesel, ordered by date.
    *   Frontend: Display readings in a table/list format.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Actix-Web): Create an API endpoint that accepts a meter ID and returns all its readings, sorted by date, using Diesel.
    2.  Frontend (Vue.js): In the meter detail view, display the fetched meter readings in a chronological list or table.

### Feature: M6: Basic Consumption Calculation
*   **Description**: Automatically calculate consumption per meter for a defined period (e.g., month, year) based on start and end readings.
*   **Key Acceptance Criteria/User Story**: For a given meter and a specified period (or between two selected readings), the system should display the calculated consumption (end reading - start reading).
*   **Technical Implementation Notes**:
    *   Backend: Rust function that takes two readings (or a series for a period) for a meter and calculates the difference.
    *   Frontend: Display calculated consumption.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust): Implement a utility function/service method that takes a list of relevant `MeterReading`s (or start/end readings) and calculates total consumption.
    2.  Backend (Actix-Web): Potentially an API endpoint to request consumption for a meter over a period, or this logic can be part of other services.
    3.  Frontend (Vue.js): Display calculated consumption, e.g., next to meter reading history or when selecting a period for a report.

### Feature: M7: Cost Type & Tariff Management
*   **Description**: Allow definition of various cost types (e.g., "General Electricity," "Heating Oil"). Allow input of prices per unit (€/kWh, €/m³) for consumption-based cost types for a period. Allow input of total costs for non-consumption-based cost types for a billing period (e.g., Insurance €500/year).
*   **Key Acceptance Criteria/User Story**: Landlord can define cost types. For consumption-based types, they can input tariffs (price/unit, valid period). For fixed cost types, they can input total amounts for a billing period.
*   **Technical Implementation Notes**:
    *   Backend: Diesel schema/structs for CostTypes (name, type - consumption-based/fixed) and Tariffs (linked to CostType, price_per_unit, valid_from, valid_to for consumption-based; or a way to store total fixed costs per period). Actix-Web handlers for CRUD.
    *   Frontend: Forms and lists for managing cost types and their associated tariffs/fixed costs.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Define `CostType` model (id, name, type: 'consumption_based'/'fixed'). Define `Tariff` model (id, cost_type_id, price_per_unit, valid_from, valid_to) for consumption-based costs. Define `FixedCost` model (id, cost_type_id, total_amount, billing_period_start, billing_period_end). Implement CRUD logic.
    2.  Backend (Actix-Web): Expose API endpoints for managing cost types, tariffs, and fixed costs.
    3.  Frontend (Vue.js): Develop UI for defining/editing cost types. For consumption-based types, allow adding/editing tariffs. For fixed types, allow specifying total costs for a period.
    4.  Frontend: Implement API calls for cost type and tariff/fixed cost management.

### Feature: M8: Allocation Key Definition
*   **Description**: Allow the landlord to define available allocation keys (e.g., "by living area," "by number of persons," "by individual consumption," "fixed share (common)").
*   **Key Acceptance Criteria/User Story**: System provides predefined allocation keys. Landlord can understand and select these keys when associating them with cost types.
*   **Technical Implementation Notes**:
    *   Backend: Could be a predefined Rust enum initially. If user-defined, then a Diesel-managed table. For MVP, an enum is simpler.
    *   Frontend: Display these options in a dropdown/selection UI.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust): Define an `enum AllocationKey { ByLivingArea, ByNumberOfPersons, ByIndividualConsumption, FixedShareCommon }`.
    2.  Backend: Ensure this enum can be serialized/deserialized if needed for API communication or stored (if linking to CostTypes in DB).
    3.  Frontend (Vue.js): When assigning to cost types, present these enum values as options in a dropdown.

### Feature: M9: Assign Allocation Keys to Cost Types
*   **Description**: Allow the landlord to assign the correct allocation key to each cost type.
*   **Key Acceptance Criteria/User Story**: Landlord can edit a cost type and assign one of the defined allocation keys to it.
*   **Technical Implementation Notes**:
    *   Backend: Link CostType struct/table to an AllocationKey (e.g., store the enum variant name as a string or an integer in the CostType table).
    *   Frontend: UI (e.g., dropdown in CostType form) to make this assignment.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust/Diesel): Modify the `CostType` model/DB schema to include a field for `allocation_key` (e.g., storing the string representation of the enum variant). Update CRUD logic for `CostType`.
    2.  Backend (Actix-Web): Ensure API for updating `CostType` can handle the allocation key assignment.
    3.  Frontend (Vue.js): In the CostType creation/editing form, add a dropdown menu populated with the available allocation keys.
    4.  Frontend: Ensure the selected allocation key is sent when saving/updating a cost type.

### Feature: M10: Define Billing Period
*   **Description**: Allow the landlord to set a global billing period (start/end date) for the annual statement.
*   **Key Acceptance Criteria/User Story**: Before generating a statement, the landlord can specify the start and end dates for the billing period.
*   **Technical Implementation Notes**:
    *   Backend: No specific storage for this if it's dynamic per report generation. User inputs this via API parameters when requesting a report.
    *   Frontend: Date pickers for start/end date for report generation.
*   **Agent Implementation Steps (Suggested)**:
    1.  Frontend (Vue.js): When initiating report generation, provide date picker inputs for the billing period (start and end date).
    2.  Frontend: Pass these dates as parameters in the API request for report generation.
    3.  Backend (Actix-Web): The report generation endpoint will accept these dates as parameters.

### Feature: M11: Core Billing Logic
*   **Description**: Calculate total costs per cost type and distribute them to units/tenants based on assigned allocation keys and master data (units, tenants, readings, consumptions, tariffs).
*   **Key Acceptance Criteria/User Story**: For a given billing period, the system correctly calculates each tenant's share of all applicable costs according to the defined data and allocation rules.
*   **Technical Implementation Notes**:
    *   Backend: Complex Rust service/module. Fetch all relevant costs, readings for the period using Diesel. For each CostType: if consumption-based, calculate total consumed units * tariff; if fixed, use total cost. Apply the assigned AllocationKey (Rust logic for 'by living area', 'by_persons', 'by_individual_consumption', 'fixed_share_common').
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust): Create a new module/service (e.g., `billing_service.rs`).
    2.  Backend: Implement logic to fetch all necessary data for the given billing period (costs, tariffs, fixed costs, meter readings, unit details, tenant details) using Diesel.
    3.  Backend: For each `CostType`:
        *   Calculate total cost for the period (summing consumptions * tariffs or using total fixed costs).
        *   Implement distribution logic for each `AllocationKey` type:
            *   `ByLivingArea`: Pro-rate cost based on `unit.living_area_m2`.
            *   `ByNumberOfPersons`: Pro-rate cost based on `tenant.number_of_persons` (summed per unit if multiple tenants per unit, though MVP seems simpler).
            *   `ByIndividualConsumption`: Directly link costs from specific meters to units/tenants based on their consumption.
            *   `FixedShareCommon`: Distribute equally or by a predefined rule (e.g., per unit).
    4.  Backend: Structure the output to show calculated shares per unit/tenant for each cost type.

### Feature: M12: Simple Billing Report Generation
*   **Description**: Enable the landlord to generate and print a simple, clear HTML report per tenant showing allocated costs and calculation basis.
*   **Key Acceptance Criteria/User Story**: Landlord can select a tenant and a billing period, and the system generates a printable HTML report detailing that tenant's share of all costs, including how these shares were calculated.
*   **Technical Implementation Notes**:
    *   Backend: Actix-Web handler that takes tenant_id and billing_period. Gathers all allocated costs (using M11 logic) for that tenant for the period. Uses a Rust templating engine (e.g., Tera, Askama) with an HTML template to structure the report.
    *   Frontend: Button to trigger report generation. Display the returned HTML. Browser's print functionality.
*   **Agent Implementation Steps (Suggested)**:
    1.  Backend (Rust): Choose and integrate a templating engine (e.g., Tera). Create an HTML template for the billing report.
    2.  Backend (Actix-Web): Create an API endpoint (e.g., `GET /reports/tenant/{tenant_id}?start_date=...&end_date=...`).
    3.  Backend: This endpoint will call the M11 core billing logic to get the calculated cost shares for the specified tenant and period.
    4.  Backend: Pass the data to the templating engine to render the HTML report. Return the HTML string.
    5.  Frontend (Vue.js): Provide a UI (e.g., button) to trigger report generation for a selected tenant and billing period.
    6.  Frontend: On receiving the HTML response, display it (e.g., in a new tab, iframe, or dedicated view). Allow printing via browser's standard print function.

## UI/UX Concept (from PRD)

*   The UI should be clean, well-structured, and significantly more user-friendly than a complex spreadsheet.
*   Navigation will be intuitive, guiding the user through setup (properties, tenants, meters, costs) and data entry (readings).
*   Key elements include:
    *   Clear Data Entry Forms for units, tenants, meters, readings, and costs.
    *   Organized Lists/Tables for displaying existing data for easy review and management.
    *   A Guided Workflow or clear steps for report generation.
    *   A Simple Report Display (HTML, printable) itemizing costs clearly for the tenant.
    *   Clear presentation of meter reading history and calculated consumptions to aid verification.

## Out of Scope for MVP (from PRD)

*   Home Assistant Integration (S1)
*   Prepayment (Vorauszahlungen) Management (S2)
*   Detailed Tenant Management (mid-year moves) (S3)
*   Advanced/Enhanced Billing Report (PDF export, custom styling) (S4)
*   Monthly Consumption Overview/Trends (charts) (S5)
*   Advanced Data Editing/Deletion (S6) (Basic CRUD is in scope, "Advanced" implies more complex scenarios like cascading deletes or historical record protection)
*   Irregular/Unexpected Cost Management (S7)
*   Advanced Analytics & Comparisons (C1)
*   Document Uploads (C2)
*   User Accounts & Login System (C3 - MVP assumes local deployment and single-user context)
*   Dashboard/Overview Page (C4)
*   Dedicated API for Future Extensions (C5)
*   Full Accounting Features (W1)
*   Direct Bank Integration (W2)
*   Tenant Login/Portal (W3)
*   Continuous Real-time Sync (W4)

## Key Agent Instructions

*   Agent: Please generate the MVP codebase based on the details above.
*   Prioritize implementing the features M1-M12 exactly as specified in the 'Core MVP Features & Implementation Plan' section.
*   Strictly adhere to the 'Technical Specifications' regarding platform, stack (Vue.js frontend, Rust/Actix-Web backend with Diesel and SQLite), and architecture.
*   Refer to the full PRD (`PRD-MVP.txt`) and Tech Design Doc (`Tech-Design-MVP.txt`) files in the project root for complete details if needed.
*   Create files and directory structures as logically required by the Tech Design Doc and the implementation plan. For Rust, follow standard cargo project structure. For Vue.js, use a standard project structure (e.g., Vue CLI generated or similar).
*   Add comments to explain complex logic, especially in the Rust backend for calculation and allocation engines.
*   For the templating engine in Rust, please choose one from the suggestions (Tera, Askama, or Handlebars-rust) and implement accordingly for M12.
*   Focus on functionality for the MVP. UI styling with Tailwind CSS should be clean and functional but not overly elaborate.