#!/bin/bash
# Start the Nebenkosten-Knecht frontend

cd "$(dirname "$0")"

# Check if node_modules exists, if not, install dependencies
if [ ! -d "node_modules" ]; then
  echo "Installing dependencies..."
  npm install
fi

# Start the development server
echo "Starting Nebenkosten-Knecht frontend development server..."
npm run serve
