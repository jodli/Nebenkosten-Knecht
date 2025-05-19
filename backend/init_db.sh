#!/bin/bash
# Initialize the SQLite database for Nebenkosten-Knecht

cd "$(dirname "$0")"

echo "Initializing Nebenkosten-Knecht database..."

# Check if diesel_cli is installed
if ! command -v diesel &> /dev/null; then
    echo "Installing diesel_cli..."
    cargo install diesel_cli --no-default-features --features sqlite
fi

# Set up the database
echo "Setting up database..."
diesel setup

# Run migrations
echo "Running migrations..."
diesel migration run

echo "Database initialization complete!"
echo "You can now start the application with ./run.sh"
