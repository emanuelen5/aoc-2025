use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <day> [part]");
        eprintln!("Example: cargo run 1      # Run both parts for day 1");
        eprintln!("Example: cargo run 1 1    # Run only part 1 for day 1");
        eprintln!("Example: cargo run 1 2    # Run only part 2 for day 1");
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().expect("Day must be a number");
    let part: Option<u8> = args.get(2).map(|s| s.parse().expect("Part must be 1 or 2"));

    let day_str = format!("{:02}", day);
    let input_path = format!("day{}/input.txt", day_str);

    let input =
        fs::read_to_string(&input_path).unwrap_or_else(|_| panic!("Could not read {}", input_path));

    println!("╔═══════════════════════════════════════════╗");
    println!("║  Advent of Code 2025 - Day {:2}           ║", day);
    println!("╚═══════════════════════════════════════════╝");
    println!();

    match day {
        // Add match arms as you implement each day
        // Example:
        // 1 => run_day(aoc_2025::day01::part1, aoc_2025::day01::part2, &input, part),
        _ => {
            eprintln!("Day {} not yet implemented!", day);
            eprintln!("Make sure to:");
            eprintln!("  1. Create the day module with ./new_day.sh {}", day);
            eprintln!("  2. Add a match arm in src/main.rs for day {}", day);
            std::process::exit(1);
        }
    }
}

fn run_day<F1, F2>(part1: F1, part2: F2, input: &str, selected_part: Option<u8>)
where
    F1: Fn(&str) -> String,
    F2: Fn(&str) -> String,
{
    match selected_part {
        Some(1) => run_part("Part 1", part1, input),
        Some(2) => run_part("Part 2", part2, input),
        None => {
            run_part("Part 1", part1, input);
            println!();
            run_part("Part 2", part2, input);
        }
        _ => {
            eprintln!("Part must be 1 or 2");
            std::process::exit(1);
        }
    }
}

fn run_part<F>(label: &str, func: F, input: &str)
where
    F: Fn(&str) -> String,
{
    println!("{}:", label);
    println!("─────────────────────────────────────────");

    let start = Instant::now();
    let result = func(input);
    let duration = start.elapsed();

    println!("│ {}", result);
    println!("─────────────────────────────────────────");
    println!("⏱  {:?}", duration);
}
