# Nebenkosten-Knecht

Nebenkosten-Knecht is a utility for landlords to track utility meters, manage costs, and generate accurate annual utility cost statements (Nebenkostenabrechnungen) for tenants.

## Project Overview

Nebenkosten-Knecht simplifies the process of creating annual utility cost statements by providing:

- Easy tracking of property units, tenants, and utility meters
- Simple input of meter readings and costs throughout the year
- Automatic calculation and allocation of costs based on defined rules
- Generation of clear, accurate statements for tenants

The application is designed for homeowners with an annexed apartment (Einliegerwohnung) or landlords with a small number of tenants who need to create and manage Nebenkostenabrechnungen efficiently.

## Development Status

This project is being developed in increments:

### Increment 1 (Current): Foundational Setup
- ✅ Property Unit Management
- ✅ Basic Tenant Management
- ✅ Meter Management

### Planned Increments
- **Increment 2**: Meter Data Entry & Consumption Tracking
- **Increment 3**: Financial Framework & Cost Allocation Rules
- **Increment 4**: Annual Statement Generation

## Features

- Property Unit Management: Create, edit, and manage property units with their living area
- Tenant Management: Manage tenants and assign them to property units
- Meter Management: Track various utility meters (electricity, water, etc.) and their assignments
- (Future) Meter Reading Entry: Record meter readings over time
- (Future) Cost Management: Define various cost types and their allocation methods
- (Future) Annual Statement Generation: Generate accurate Nebenkostenabrechnungen for tenants

## Project Structure

This project consists of two main components:

- **Backend**: A Rust-based API server using Actix-Web and Diesel ORM with SQLite
- **Frontend**: A Vue.js web application with Tailwind CSS for styling

## Getting Started

### Prerequisites

- Rust (stable) with Cargo
- Node.js (>=14) and npm/yarn
- SQLite

### Setup and Running

There are two ways to run the application:

#### Option 1: Using the convenience scripts

1. Start both backend and frontend with one command:

```bash
./run.sh
```

This script will check if tmux is installed and will use it to run both servers. If tmux is not available, it will provide instructions.

2. Alternatively, you can start each component individually:

```bash
# Start the backend
./backend/run.sh

# Start the frontend (in a different terminal)
./frontend/run.sh
```

#### Option 2: Manual setup

##### Backend

1. Navigate to the backend directory:

```bash
cd backend
```

2. Run database migrations to set up the database:

```bash
cargo install diesel_cli --no-default-features --features sqlite
diesel migration run
```

3. Run the server:

```bash
cargo run
```

The backend will start at http://localhost:8080.

#### Frontend

1. Navigate to the frontend directory:

```bash
cd frontend
```

2. Install dependencies:

```bash
npm install
# or with yarn
yarn
```

3. Run the development server:

```bash
npm run serve
# or with yarn
yarn serve
```

The frontend development server will start at http://localhost:8080 and will proxy API requests to the backend.

## Development

### Backend (Rust)

- Models are defined in `backend/src/models/`
- API handlers are in `backend/src/handlers/`
- Database operations are managed via Diesel ORM

### Frontend (Vue.js)

- Components are in `frontend/src/components/`
- Views and pages are in `frontend/src/views/`
- API services are defined in `frontend/src/services/`

## License

This project is licensed under the [MIT License](LICENSE).
