# General Product Specification: Nebenkostenabrechnungs-App

## 1. Product Goal

The primary goal of this product is to develop a personal web application that replaces an existing Excel-based system for managing and calculating utility bills (Nebenkostenabrechnung) for a 3-party residential building. The application will focus on the user's specific needs as the landlord and resident, allowing for:
*   Erfassung von Zählerständen (manually and potentially via Home Assistant API in the future).
*   Erfassung von anfallenden Kosten (Heizöl, Wartung, Versicherungen, etc.).
*   Erfassung von Mieter-Vorauszahlungen.
*   Berechnung der Nebenkostenverteilung basierend auf spezifischen, im System hinterlegten Regeln (z.B. nach Wohnfläche, Verbrauch, Personenzahl, mit Sonderregeln für EG-Holzofen).
*   Generierung eines einfachen, druckbaren Berichts pro Wohneinheit (insbesondere für den Mieter im UG), der die umgelegten Kosten, die Vorauszahlungen und den resultierenden Saldo (Nachzahlung/Guthaben) klar darstellt.

The MVP (Minimum Viable Product) will be tailored specifically to the user's current 3-Wohneinheiten-Setup and calculation rules, prioritizing a quick time-to-value for personal use.

## 2. Technologies Used

*   **Frontend:** React
*   **Backend:** Rust
    *   Web Framework: Actix
    *   Serialization: Serde, Serde_json
    *   Configuration Parsing: `toml` crate
*   **Data Storage:** JSON files (segregated by year, e.g., `ablesungen_YYYY.json`, `kosten_YYYY.json`, `vorauszahlungen_YYYY.json`)
*   **Configuration:** TOML file (`config.toml`) for static application parameters (units, meters, cost types, calculation rules).
*   **Version Control:** Git
*   **CI/CD:** Basic setup (e.g., GitHub Actions) for automated builds and tests.

## 3. Testing Requirements

*   **Unit Tests:** Mandatory for core backend logic, especially:
    *   Configuration parsing.
    *   JSON data serialization/deserialization (I/O operations).
    *   Calculation logic for utility bill distribution (once implemented).
*   **Integration Tests:** For API endpoints to ensure request/response cycles work as expected and data persistence is correct.
*   **Automation:** Basic CI/CD pipeline to run compilations and automated tests on each commit/push to the main branch.
*   **Manual Testing:** Required for UI workflows and end-to-end functionality verification during development and before considering a sprint complete.

## 4. Cross-Cutting Concerns

*   **Configuration-Driven Core:** The application's behavior regarding building structure (units, their properties like `wohnflaeche_real`, `wohnflaeche_heizung`), meter definitions, and cost type definitions (including their specific distribution logic identifiers) will be driven by a central `config.toml` file. This avoids hardcoding these specific parameters into the application logic, allowing for easier adjustments if needed.
*   **Yearly Data Segregation:** All variable data (meter readings, costs, prepayments) will be stored in JSON files named with the corresponding year (e.g., `ablesungen_2023.json`). This keeps individual data files manageable and aligns with the annual billing cycle.
*   **Single-User Focus (MVP):** The initial application is designed for a single administrator (the user). No multi-user authentication or complex role management will be implemented in the MVP.
*   **Simplicity and Pragmatism for MVP:** UI/UX will be functional and straightforward, prioritizing core feature delivery over elaborate design. Development will focus on achieving the product goal for the user's specific scenario efficiently.
*   **Error Handling:** Basic error handling must be implemented for file I/O operations (config and data files), API request processing, and user input validation where critical.
*   **State Management (Frontend):** Basic React state management (e.g., `useState`, `useEffect`, Context API if needed for simple global state like configuration).
*   **API Design:** RESTful principles for API endpoints. Clear request/response formats (JSON).