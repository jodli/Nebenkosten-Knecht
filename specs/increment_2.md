# Sprint 2 Spec: Data Erfassung (Input and Persistence)

## 1. Problem Statement
The application needs a way for the user to input variable data such as meter readings, incurred costs, and tenant prepayments. This data must be persistently stored, segregated by year, to be used later for the utility bill calculation.

## 2. Solution Overview
This sprint will extend the application by adding UI forms in React for data entry. These forms will allow the user to input:
1.  Meter readings (`Zählerstände`) for specific meters (defined in `config.toml`).
2.  Cost entries (`Kosten`) for specific cost types (defined in `config.toml`).
3.  Prepayments (`Vorauszahlungen`) per residential unit (defined in `config.toml`).

The backend (Rust/Actix) will provide new API endpoints to receive this data. Upon receiving the data, the backend will serialize it into JSON format and save/append it to year-specific files (e.g., `ablesungen_2023.json`, `kosten_2023.json`, `vorauszahlungen_2023.json`).

## 3. Implementation Details

### Data Structures (Rust/Serde - to be defined in backend, e.g., in `models.rs` or `data_types.rs`)
*   `MeterReading`:
    ```rust
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct MeterReading {
        pub zaehler_id: String, // ID from config.toml
        pub datum: String,      // YYYY-MM-DD
        pub wert: f64,
    }
    ```
*   `CostEntry`:
    ```rust
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CostEntry {
        pub kostenart_id: String, // ID from config.toml
        pub betrag: f64,
        pub bezeichnung: Option<String>, // Optional user-defined description
        // pub zeitraum_start: String, // YYYY-MM-DD (Consider if needed for individual costs or if global report period is enough)
        // pub zeitraum_ende: String,   // YYYY-MM-DD
    }
    ```
*   `Prepayment`:
    ```rust
    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PrepaymentEntry { // Renamed to avoid conflict if Prepayment struct used elsewhere
        pub einheit_id: String, // ID from config.toml
        pub betrag: f64,
        // pub datum_erfassung: String // YYYY-MM-DD
    }
    ```
*   Wrapper for yearly data files (e.g., a list of `MeterReading`):
    ```rust
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct YearlyMeterReadings {
        pub readings: Vec<MeterReading>,
    }
    // Similar structs for YearlyCostEntries, YearlyPrepayments
    ```

### Backend (Rust/Actix with Serde/Serde_json)
*   **Modules/Files:**
    *   `data_storage.rs` (or `storage/json_storage.rs`):
        *   `fn get_data_file_path(year: u16, data_type: &str) -> PathBuf` (e.g., `data_type` = "ablesungen", "kosten", "vorauszahlungen")
        *   `async fn load_yearly_data<T: for<'de> Deserialize<'de> + Default>(year: u16, data_type: &str) -> Result<T, io::Error>`
        *   `async fn save_yearly_data<T: Serialize>(year: u16, data_type: &str, data: &T) -> Result<(), io::Error>`
        *   Logic to append to lists within JSON files:
            *   `async fn add_meter_reading(year: u16, reading: MeterReading) -> Result<(), io::Error>`
            *   `async fn add_cost_entry(year: u16, cost: CostEntry) -> Result<(), io::Error>`
            *   `async fn add_prepayment(year: u16, prepayment: PrepaymentEntry) -> Result<(), io::Error>` (This might overwrite if only one prepayment per unit per year is assumed for simplicity, or append if multiple are allowed).
    *   `api_handlers.rs` (or `handlers/data_entry_handlers.rs`):
        *   `async fn submit_meter_reading(year: web::Path<u16>, reading_data: web::Json<MeterReading>, ... ) -> impl Responder`
        *   `async fn submit_cost_entry(year: web::Path<u16>, cost_data: web::Json<CostEntry>, ... ) -> impl Responder`
        *   `async fn submit_prepayment(year: web::Path<u16>, prepayment_data: web::Json<PrepaymentEntry>, ... ) -> impl Responder`
*   **API Endpoints:**
    *   `POST /api/data/{year}/readings` (Payload: `MeterReading`)
    *   `POST /api/data/{year}/costs` (Payload: `CostEntry`)
    *   `POST /api/data/{year}/prepayments` (Payload: `PrepaymentEntry`)
    *   The `{year}` path parameter indicates which year's data file to use.

### Frontend (React)
*   **Shared Components:**
    *   `YearSelector.js`: A component to select the target year for data entry.
*   **New Components & Forms:**
    *   `forms/MeterReadingForm.js`:
        *   Dropdown to select `zaehler_id` (populated from `/api/config/counters` via `configService.js` from Sprint 1).
        *   Input for `datum` (date picker recommended).
        *   Input for `wert` (number).
        *   Submit button.
    *   `forms/CostEntryForm.js`:
        *   Dropdown to select `kostenart_id` (populated from `/api/config/cost_types`).
        *   Input for `betrag` (number).
        *   Optional input for `bezeichnung`.
        *   Submit button.
    *   `forms/PrepaymentForm.js`:
        *   Dropdown to select `einheit_id` (populated from `/api/config/units`).
        *   Input for `betrag` (number).
        *   Submit button.
*   **Services:**
    *   `services/dataEntryService.js`: Functions to POST data to the new backend API endpoints.
*   **UI:**
    *   Create separate views/pages for each data entry type, or a tabbed interface.
    *   Include the `YearSelector` on these views.
    *   Provide user feedback on successful submission or errors.

## 4. Validation and Testing Plan
*   **Reference General Spec:** See `general.md` for overall testing requirements.
*   **Unit Tests (Rust Backend - `data_storage.rs`):**
    *   Test `add_meter_reading`:
        *   Correctly creates `ablesungen_YYYY.json` if not exists.
        *   Correctly appends new reading to existing `readings` array in JSON.
        *   Handles file I/O errors.
    *   Similar tests for `add_cost_entry` and `add_prepayment`.
    *   Test correct (de)serialization of `YearlyMeterReadings`, etc.
*   **Integration Tests (Backend API - using `actix-web::test` or Postman/curl):**
    *   `POST` valid `MeterReading` JSON to `/api/data/{year}/readings`. Verify HTTP 200/201 and check `ablesungen_YYYY.json` content.
    *   `POST` valid `CostEntry` JSON to `/api/data/{year}/costs`. Verify HTTP 200/201 and check `kosten_YYYY.json` content.
    *   `POST` valid `PrepaymentEntry` JSON to `/api/data/{year}/prepayments`. Verify HTTP 200/201 and check `vorauszahlungen_YYYY.json` content.
    *   Test with invalid payloads (e.g., missing fields, wrong data types) – expect HTTP 400.
    *   Test with non-existent year – ensure appropriate handling (e.g., file creation).
*   **Manual Testing (UI):**
    *   Navigate to each data entry form.
    *   Select a year using `YearSelector`.
    *   Populate dropdowns (for meters, cost types, units) correctly from config.
    *   Enter valid data into all fields and submit.
        *   Verify success feedback.
        *   Manually inspect the corresponding `_YYYY.json` files on the server to confirm data is saved correctly and in the right year's file.
    *   Attempt to submit forms with invalid or missing data – verify appropriate error messages or prevention.

## 5. Dependencies
*   Successful completion of Sprint 1 (backend setup, config loading, and API endpoints for config data used to populate dropdowns).
*   File system write access for the backend process in the directory where JSON data files will be stored (e.g., a `data/` subdirectory).

## 6. Acceptance Criteria
*   The user can successfully submit meter readings through the UI, and this data is appended to the correct `ablesungen_YYYY.json` file.
*   The user can successfully submit cost entries through the UI, and this data is appended to the correct `kosten_YYYY.json` file.
*   The user can successfully submit prepayments through the UI, and this data is saved/appended to the correct `vorauszahlungen_YYYY.json` file.
*   Data entry forms correctly populate selection dropdowns using configuration data fetched from Sprint 1 APIs.
*   Data is stored in a structured JSON format within the year-specific files.
*   Basic user feedback (success/error) is provided upon data submission.
*   All code developed for this sprint is committed to the version control system.