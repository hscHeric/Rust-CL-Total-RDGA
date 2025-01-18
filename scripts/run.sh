#!/bin/bash

# Default parameters
MAX_STAGNANT=100
GENERATIONS=1000
TOURNAMENT_SIZE=5
CROSSOVER_PROB=0.9
POP_SIZE=50

# Help function
show_help() {
  echo "Usage: $0 [-s max_stagnant] [-g generations] [-t tournament_size] [-c crossover_prob] [-p pop_size]"
  echo "Options:"
  echo "  -s: Maximum stagnant generations (default: 100)"
  echo "  -g: Maximum generations (default: 1000)"
  echo "  -t: Tournament size (default: 5)"
  echo "  -c: Crossover probability (default: 0.9)"
  echo "  -p: Population size (default: 50)"
  exit 1
}

# Parse command line options
while getopts "h:s:g:t:c:p:" opt; do
  case $opt in
  h) show_help ;;
  s) MAX_STAGNANT=$OPTARG ;;
  g) GENERATIONS=$OPTARG ;;
  t) TOURNAMENT_SIZE=$OPTARG ;;
  c) CROSSOVER_PROB=$OPTARG ;;
  p) POP_SIZE=$OPTARG ;;
  ?) show_help ;;
  esac
done

# Get the script's directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Get the project root directory (parent of scripts directory)
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Verify directories exist
if [ ! -d "$PROJECT_ROOT/data/edges" ]; then
  echo "Error: edges directory not found at $PROJECT_ROOT/data/edges"
  exit 1
fi

# Ensure the target binary exists
if [ ! -f "$PROJECT_ROOT/target/release/cl-total-rdga" ]; then
  echo "Error: cl-total-rdga binary not found in $PROJECT_ROOT/target/release/"
  echo "Please build the project first with 'cargo build --release'"
  exit 1
fi

# Create results directory if it doesn't exist
mkdir -p "$PROJECT_ROOT/data/results"

# Create a temporary file to store file sizes and paths
temp_file=$(mktemp)

# Check if there are any .txt files
edge_files=("$PROJECT_ROOT/data/edges/"*.txt)
if [ ! -e "${edge_files[0]}" ]; then
  echo "Error: No .txt files found in $PROJECT_ROOT/data/edges/"
  rm "$temp_file"
  exit 1
fi

# Get all .txt files and their sizes, sort by size
for edge_file in "$PROJECT_ROOT"/data/edges/*.txt; do
  if [ -f "$edge_file" ]; then
    # Count number of lines in file
    num_lines=$(wc -l <"$edge_file")
    echo "$num_lines $edge_file" >>"$temp_file"
  fi
done

# Sort files by size (number of lines)
sorted_files=$(sort -n "$temp_file" | cut -d' ' -f2-)

# Remove temporary file
rm "$temp_file"

# Get total number of files
total_files=$(echo "$sorted_files" | wc -l)
current_file=0

echo "Found $total_files edge list files to process"
echo "Starting processing..."
echo "----------------------------------------"

# Process each file in sorted order
echo "$sorted_files" | while read -r edge_file; do
  ((current_file++))
  filename=$(basename "$edge_file")
  filename_no_ext="${filename%.txt}"
  output_file="$PROJECT_ROOT/data/results/${filename_no_ext}.csv"
  num_lines=$(wc -l <"$edge_file")

  echo "Processing file ${current_file}/${total_files}: $filename (Lines: $num_lines)"
  echo "Output will be saved to: $output_file"

  # Execute the algorithm with full paths
  "$PROJECT_ROOT/target/release/cl-total-rdga" \
    "$edge_file" \
    30 \
    "$output_file"

  # Check if the execution was successful
  if [ $? -ne 0 ]; then
    echo "Error processing $filename"
    echo "Command failed with exit code $?"
  fi

  echo "----------------------------------------"
done

echo "Processing complete!"
