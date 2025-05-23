# Sprint 1 Spec: Foundation and Configuration Handling

## 1. Problem Statement
The project requires a foundational software architecture and a mechanism to load and utilize static configuration data that defines the building's structure, meters, and basic rules. This configuration needs to be externalized to avoid hardcoding and allow for easier future adjustments.

## 2. Solution Overview
This sprint focuses on setting up the basic project structure for both backend (Rust/Actix) and frontend (React). The backend will be responsible for parsing a `config.toml` file on startup, which contains definitions for Wohneinheiten (residential units), Zähler (meters), and Kostenarten (cost types). Basic API endpoints will be created to expose this configuration data to the frontend, which will then display it in a simple, read-only manner.

## 3. Implementation Details

### Backend (Rust/Actix with Serde & TOML)
*   **Project Setup:**
    *   `cargo new --bin nebenkosten_app_backend`
    *   Add dependencies: `actix-web`, `serde` (with `derive`), `toml`, `log`, `env_logger`.
*   **Configuration (`config.toml`):**
    *   Define structure for:
        ```toml
        # config.toml example structure
        [[units]]
        id = "UG"
        name = "Untergeschoss"
        wohnflaeche_real = 50.0
        wohnflaeche_heizung = 50.0
        personen = 1

        [[units]]
        id = "EG"
        # ... other units

        [[counters]]
        id = "heizung_brenner1_h"
        name = "Brennerstunden Stufe 1"
        unit = "h"
        type = "Heizung" # General type
        # unit_id = "UG" # If specific to a unit

        [[counters]]
        id = "wasser_ug_kalt_m3"
        # ... other counters

        [[cost_types]]
        id = "heizöl"
        name = "Heizöl"
        umlage_logik_ref = "A" # Reference to a predefined logic type

        # ... other cost types

        [heizung_config]
        oel_verbrauch_pro_stunde_stufe1 = 2.5
        oel_verbrauch_pro_stunde_stufe2 = 3.5
        ```
*   **Modules/Files:**
    *   `main.rs`: Initialize logging, load configuration, setup and run Actix server.
    *   `config.rs`:
        *   Define Rust structs (`UnitConfig`, `CounterConfig`, `CostTypeConfig`, `HeizungParams`, `AppConfig`) that mirror `config.toml` structure (using `serde::Deserialize`).
        *   Function `load_app_config(file_path: &str) -> Result<AppConfig, anyhow::Error>`.
    *   `api_handlers.rs` (or `handlers/config_handlers.rs`):
        *   `async fn get_units_config(app_config: web::Data<AppConfig>) -> impl Responder`
        *   `async fn get_counters_config(app_config: web::Data<AppConfig>) -> impl Responder`
        *   `async fn get_cost_types_config(app_config: web::Data<AppConfig>) -> impl Responder`
*   **API Endpoints:**
    *   `GET /api/config/units` -> returns `Vec<UnitConfig>`
    *   `GET /api/config/counters` -> returns `Vec<CounterConfig>`
    *   `GET /api/config/cost_types` -> returns `Vec<CostTypeConfig>`
*   **State Management (Actix):** Loaded `AppConfig` will be shared as `web::Data<AppConfig>`.

### Frontend (React)
*   **Project Setup:**
    *   `npx create-react-app nebenkosten_app_frontend`
*   **Components:**
    *   `App.js`: Main application component, routing (if any at this stage).
    *   `services/configService.js`: Functions to fetch data from backend config API endpoints.
    *   `components/ConfigViewer.js`:
        *   Fetches units, counters, cost types using `configService.js`.
        *   Displays the fetched lists in a simple, readable format (e.g., using HTML tables or lists).
*   **Styling:** Minimal, focus on readability.

### CI/CD
*   Setup basic GitHub Actions workflow:
    *   Checkout code.
    *   Setup Rust toolchain.
    *   `cargo build --verbose` for backend.
    *   `cargo test --verbose` for backend.
    *   Setup Node.js.
    *   `npm ci` & `npm run build` for frontend (optional: `npm test`).

## 4. Validation and Testing Plan

*   **Reference General Spec:** See `general.md` for overall testing requirements.
*   **Unit Tests (Rust Backend - `config.rs`):**
    *   Test `load_app_config` with a valid sample `config.toml`.
    *   Test `load_app_config` with a malformed `config.toml` (expect error).
    *   Test `load_app_config` with a missing `config.toml` file (expect error).
*   **API Testing (Manual - e.g., `curl` or Postman):**
    *   Verify `GET /api/config/units` returns HTTP 200 and correct JSON array.
    *   Verify `GET /api/config/counters` returns HTTP 200 and correct JSON array.
    *   Verify `GET /api/config/cost_types` returns HTTP 200 and correct JSON array.
*   **Manual Testing (UI):**
    *   Start backend and frontend applications.
    *   Open frontend in browser.
    *   Verify `ConfigViewer.js` component correctly fetches and displays the lists of units, counters, and cost types from the backend, matching the content of `config.toml`.
    *   Check browser console for any errors during data fetching or rendering.

## 5. Dependencies
*   A valid `config.toml` file must be present at the expected location for the backend to start.
*   Backend server must be running for the frontend to fetch configuration data.
*   Node.js and npm/yarn for frontend development.
*   Rust toolchain for backend development.

## 6. Acceptance Criteria
*   The backend application successfully parses the `config.toml` file upon startup.
*   API endpoints (`/api/config/units`, `/api/config/counters`, `/api/config/cost_types`) are implemented and return the respective configuration data in JSON format.
*   The frontend application fetches and displays the lists of configured units, counters, and cost types from the backend API.
*   No critical errors (e.g., crashes, inability to load config) occur during normal operation of this increment.
*   Basic CI/CD pipeline successfully compiles the backend and runs backend unit tests.
*   All code developed for this sprint is committed to the version control system.