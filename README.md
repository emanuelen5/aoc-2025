# Advent of Code 2025 in Rust

## Starting a new day

```bash
./new_day.sh 1
```

This will:
- Create `day01/` directory with `input.txt` and `example1.txt`
- Create `src/day01.rs` from the template `./template.rs`
- Update `src/lib.rs` to include the new module
- Optionally download your puzzle input (if `.session` file exists)

### Automatically download puzzle inputs

To set it up:

1. Log in to [adventofcode.com](https://adventofcode.com)
2. Get your session cookie from browser dev tools (Application/Storage → Cookies → session)
3. Save its value to file called `.session`


### Running solutions

```bash
# Whole day 1
cargo run 1
# Day 2, part 1
cargo run 2 1
```

### Running tests

Run all tests for a specific day:
```bash
cargo test --lib day01
```
