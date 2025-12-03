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

# Copy template to the day directory
cp template.rs "src/$DIR.rs"

# Replace placeholder in the template
sed -i "s/dayXX/day$DAY/g" "src/$DIR.rs"

# Add module declaration to lib.rs
sed -i "s|// pub mod day$DAY;|pub mod day$DAY;|" src/lib.rs

# Create input.txt placeholder
touch "$DIR/input.txt"

# Create example.txt placeholder
cat > "$DIR/example.txt" << 'EOF'
TODO: Add example input here
EOF

echo "✓ Created $DIR/"
echo "✓ Created src/$DIR.rs"
echo "✓ Created $DIR/input.txt (empty)"
echo "✓ Created $DIR/example.txt"
echo "✓ Updated src/lib.rs"
echo ""

# Check if session cookie exists for downloading input
if [ -f ".session" ]; then
    SESSION=$(cat .session)
    echo "Attempting to download input for day $1..."

    if curl -s -f -b "session=$SESSION" "https://adventofcode.com/2025/day/$1/input" -o "$DIR/input.txt"; then
        echo "✓ Downloaded input to $DIR/input.txt"
    else
        echo "✗ Failed to download input. Please add it manually to $DIR/input.txt"
    fi
else
    echo "No .session file found. To automatically download inputs:"
    echo "  1. Log in to adventofcode.com"
    echo "  2. Copy your session cookie"
    echo "  3. Save it to .session file in project root"
    echo ""
    echo "Please manually download input from:"
    echo "  https://adventofcode.com/2025/day/$1/input"
    echo "  and save to $DIR/input.txt"
fi

echo ""
echo "Ready to start! Edit src/$DIR.rs and run:"
echo "  cargo test --lib day$DAY"
echo ""
echo "To create a git branch for this day:"
echo "  git checkout -b day$DAY"
