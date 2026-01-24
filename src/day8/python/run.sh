#!/bin/bash
# Convenience script to run the Day 8 3D visualization

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Activate virtual environment and run the visualization script
source "$SCRIPT_DIR/venv/bin/activate"
python "$SCRIPT_DIR/visualize.py" --input "$SCRIPT_DIR/../test_input.txt" "$@"