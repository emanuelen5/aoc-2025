# Advent of Code 2025 - Rust TDD Setup

A Rust project template for solving Advent of Code puzzles using Test-Driven Development.

## Features

- 🚀 **Quick Setup**: Create a new day with a single command
- 🧪 **Test-Driven**: Built-in test structure with example and actual input tests
- 📦 **Organized**: Each day in its own directory with separate input files
- ⚡ **Fast Iteration**: Easy two-part implementation workflow
- 📋 **Copy-Paste Ready**: Formatted output for easy submission
- 🌿 **Git-Friendly**: Suggestions for branch-per-day workflow

## Quick Start

### Starting a New Day

```bash
./new_day.sh 1
```

This will:
- Create `day01/` directory with `input.txt` and `example.txt`
- Create `src/day01.rs` from the template
- Update `src/lib.rs` to include the new module
- Optionally download your puzzle input (if `.session` file exists)

### Optional: Automatic Input Download

To automatically download puzzle inputs:

1. Log in to [adventofcode.com](https://adventofcode.com)
2. Get your session cookie from browser dev tools (Application/Storage → Cookies → session)
3. Save it to a `.session` file in the project root:
   ```bash
   echo "your_session_cookie_here" > .session
   ```

### Working on a Day (TDD Workflow)

1. **Read the problem** on adventofcode.com

2. **Update the example in `src/dayXX.rs`**:
   ```rust
   const EXAMPLE: &str = "\
   your example input here
   ";
   ```

3. **Update the expected result in the test**:
   ```rust
   #[test]
   fn test_part1_example() {
       assert_eq!(part1(EXAMPLE), "expected answer");
   }
   ```

4. **Implement `part1` until the test passes**:
   ```bash
   cargo test --lib day01::tests::test_part1_example
   ```

5. **Run against actual input** (remove `#[ignore]` or use `--ignored`):
   ```bash
   cargo test --lib day01::tests::test_part1_input -- --ignored --nocapture
   ```
   Or get the formatted output:
   ```bash
   cargo run 1 1
   ```

6. **Submit part 1, then repeat for part 2**:
   - Update `test_part2_example` with the new expected result
   - Remove `#[ignore]` from `test_part2_example`
   - Implement `part2`
   - Run tests and get your answer

### Running Solutions

Run both parts:
```bash
cargo run 1
```

Run only part 1:
```bash
cargo run 1 1
```

Run only part 2:
```bash
cargo run 1 2
```

Example output:
```
╔═══════════════════════════════════════════╗
║  Advent of Code 2025 - Day  1            ║
╚═══════════════════════════════════════════╝

Part 1:
─────────────────────────────────────────
│ 42
─────────────────────────────────────────
⏱  245.2µs
```

### Running Tests

Run all tests for a specific day:
```bash
cargo test --lib day01
```

Run a specific test:
```bash
cargo test --lib day01::tests::test_part1_example
```

Run ignored tests (input tests) and show output:
```bash
cargo test --lib day01 -- --ignored --nocapture
```

## Git Workflow (Recommended)

Work on each day in a separate branch:

```bash
# Start a new day
./new_day.sh 1
git checkout -b day01
git add .
git commit -m "Day 1 setup"

# After solving
git commit -am "Day 1 complete"
git checkout main
git merge day01
```

## Project Structure

```
aoc-2025/
├── Cargo.toml           # Project configuration
├── new_day.sh           # Script to create new day
├── template.rs          # Template for new days
├── .session             # Your AoC session cookie (gitignored)
├── src/
│   ├── main.rs          # Runner for solutions
│   ├── lib.rs           # Module declarations
│   ├── day01.rs         # Day 1 solution
│   ├── day02.rs         # Day 2 solution
│   └── ...
└── day01/
    ├── input.txt        # Your puzzle input (gitignored)
    └── example.txt      # Example from problem
```

## Tips

- **Start with the example**: Always make the example test pass first
- **Use `todo!()`**: The template includes `todo!()` macros - implement incrementally
- **Print debugging**: Use `println!` in tests with `-- --nocapture` flag
- **Ignore slow tests**: Keep `#[ignore]` on input tests to avoid running them constantly
- **Profile if needed**: The runner shows execution time for each part

## Customization

### Adding Dependencies

Edit `Cargo.toml` to add crates you commonly use:

```toml
[dependencies]
itertools = "0.12"
regex = "1.10"
```

### Updating the Template

Modify `template.rs` to change the default structure for new days.

## Troubleshooting

**"Day X not yet implemented"**: Add a match arm in `src/main.rs`:
```rust
match day {
    1 => run_day(aoc_2025::day01::part1, aoc_2025::day01::part2, &input, part),
    // Add your day here
    _ => { /* error */ }
}
```

**Input download fails**:
- Check your `.session` cookie is valid
- Make sure you've unlocked that day's puzzle
- Manually download from `https://adventofcode.com/2025/day/X/input`

## License

This template is free to use for Advent of Code participation.

Happy coding! 🎄⭐
