#!/usr/bin/env bash
 
set -euo pipefail

DAY=${1:-}
if [[ -z "$DAY" ]]; then
  echo "Usage: $0 <day-number>"
  exit 1
fi

lib_name="$(printf "day%02d" "$DAY")"

available_days=$(echo day* | sed 's/day0//g' | sed 's/day//g' | xargs printf '%d ' | column)

if [ ! -f "src/$lib_name.rs" ]; then
  echo "The library ./src/$lib_name.rs does not exist." >&2
  echo "Available days are: $available_days" >&2
  exit 1
fi

cargo test --lib "$lib_name"
