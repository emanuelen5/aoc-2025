#!/usr/bin/env bash

# Script to create a new day directory and optionally download the input
# Usage: ./new_day.sh <day_number>
# Example: ./new_day.sh 1

set -e

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 1"
    exit 1
fi

DAY=$(printf "%02d" "$1")
DIR="day$DAY"

if [ -d "$DIR" ]; then
    echo "Directory $DIR already exists!"
    exit 1
fi

echo "Creating directory structure for Day $DAY..."

# Create the day directory
mkdir -p "$DIR"
echo "✓ Created $DIR/"

# Copy template to the day directory
cp template.rs "src/$DIR.rs"

# Replace placeholder in the template
sed -i "s/dayXX/day$DAY/g" "src/$DIR.rs"
echo "✓ Created src/$DIR.rs"

# Add module declaration to lib.rs
sed -i "s|// pub mod day$DAY;|pub mod day$DAY;|" src/lib.rs
echo "✓ Updated src/lib.rs"

# Create input.txt placeholder
touch "$DIR/input.txt"
echo "✓ Created $DIR/input.txt (empty)"

# Create empty example file
touch "$DIR/example1.txt"
echo "✓ Created $DIR/example1.txt"
echo ""

download-input() {
    local session
    session=$(cat .session)
    echo "Attempting to download input for day $1..."

    if curl -s -f -b "session=$session" "https://adventofcode.com/2025/day/$1/input" -o "$DIR/input.txt"; then
        echo "✓ Downloaded input to $DIR/input.txt"
    else
        echo "✗ Failed to download input. Please add it manually to $DIR/input.txt"
    fi

}

# Check if session cookie exists for downloading input
if [ ! -f ".session" ]; then
    echo "No .session file found. See readme how to set it up" >&2
else
    download-input "$1"
fi

echo ""
echo "Ready to start! Edit src/$DIR.rs and run:"
echo "  ./run_day.sh $1"
