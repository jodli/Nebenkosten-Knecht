# Nebenkosten-Knecht Quick Start Guide

This guide will help you quickly get started with Nebenkosten-Knecht.

## 1. Installation & Setup

1. Clone the repository
2. Make sure you have the following prerequisites installed:
   - Rust and Cargo
   - Node.js and npm
   - SQLite

3. Run the convenience script to start both backend and frontend:
   ```
   ./run.sh
   ```

## 2. Initial Setup

Once the application is running, follow these steps to set up your property and tenants:

### Step 1: Define Your Property Units

1. Navigate to "Property Units" in the main menu
2. Click "Add Property Unit"
3. Enter details:
   - **Name**: A descriptive name for the unit (e.g., "Main Apartment", "Basement Flat")
   - **Living Area (m²)**: The size of the unit in square meters

### Step 2: Add Your Tenants

1. Navigate to "Tenants" in the main menu
2. Click "Add Tenant"
3. Enter details:
   - **Name**: The tenant's name
   - **Number of Persons**: How many people live in the unit
   - **Property Unit**: Select the unit this tenant occupies from the dropdown

### Step 3: Define Your Meters

1. Navigate to "Meters" in the main menu
2. Click "Add Meter"
3. Enter details:
   - **Name**: A descriptive name for the meter (e.g., "Main Electricity Meter", "Basement Water Meter")
   - **Meter Type**: The type of utility (e.g., "Electricity", "Water", "Gas")
   - **Unit**: The measurement unit (e.g., "kWh", "m³")
   - **Assignment Type**: Select whether this meter is assigned to a specific unit or is common
   - **Property Unit**: If assigned to a unit, select which one

## 3. Next Steps

With the basic setup complete, you can now:

- Add more property units, tenants, or meters as needed
- Update information for any existing items
- In future versions:
  - Record meter readings
  - Define cost types and allocation rules
  - Generate annual utility cost statements

## 4. Tips

- **Property Units**: Create a unit for all parts of your property, including your own apartment
- **Tenants**: Remember to include yourself as a tenant if you live in one of the units
- **Meters**: Label meters clearly to avoid confusion when recording readings
- **Common Meters**: Meters that track consumption for the entire property should be marked as "Common"

## Need Help?

If you encounter any issues, please check the project README.md file for more detailed information or raise an issue on the project repository.
