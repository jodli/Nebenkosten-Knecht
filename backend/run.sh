#!/bin/bash
# Start the Nebenkosten-Knecht backend

cd "$(dirname "$0")"

# Create the database if it doesn't exist
if [ ! -f "nebenkosten_knecht.db" ]; then
  echo "Setting up the database..."

  # First, try using diesel cli if available
  if command -v diesel &> /dev/null; then
    diesel setup
    diesel migration run
  else
    # If diesel cli is not installed, explain how to set up the database
    echo "Diesel CLI is not installed. To set up the database, run:"
    echo "cargo install diesel_cli --no-default-features --features sqlite"
    echo "diesel setup"
    echo "diesel migration run"
  fi
fi

# Run the server
echo "Starting Nebenkosten-Knecht backend server..."
cargo run
