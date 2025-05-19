#!/bin/bash
# Start both the backend and frontend for Nebenkosten-Knecht

cd "$(dirname "$0")"

echo "Starting Nebenkosten-Knecht..."
echo "Make sure you have two terminal windows open:"
echo "1. For the backend server (Rust/Actix-Web)"
echo "2. For the frontend development server (Vue.js)"
echo ""
echo "You can start each component individually with:"
echo "- Backend: ./backend/run.sh"
echo "- Frontend: ./frontend/run.sh"
echo ""
echo "To start the complete system:"

# Check if tmux is available
if command -v tmux &> /dev/null; then
  echo "Starting both servers with tmux..."

  # Create a new tmux session for the backend
  tmux new-session -d -s nebenkosten-knecht -n backend "cd backend && ./run.sh"

  # Create a new window for the frontend
  tmux new-window -t nebenkosten-knecht -n frontend "cd frontend && ./run.sh"

  # Attach to the session
  tmux attach-session -t nebenkosten-knecht
else
  echo "For the best experience, install tmux to run both servers simultaneously."
  echo "For now, please start each component in a separate terminal:"
  echo ""
  echo "Terminal 1: ./backend/run.sh"
  echo "Terminal 2: ./frontend/run.sh"
fi
