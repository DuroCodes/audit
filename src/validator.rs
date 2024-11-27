use crate::config::Solution;
use crate::runner::DayOutput;

fn symbol(actual: &Option<String>, expected: &str) -> &'static str {
    match actual {
        Some(actual) if actual == expected => "✅",
        Some(_) => "❌",
        None => "💥",
    }
}

fn part_info(actual: &Option<String>, expected: &str) -> (&'static str, String) {
    let symbol = symbol(actual, expected);
    let msg = match actual {
        Some(actual) => format!("expected: {expected}, got: {actual}"),
        None => format!("expected: {expected}, unimplemented"),
    };

    (symbol, msg)
}

pub fn validate_parts(days: &[DayOutput], solutions: &[Solution]) {
    for (i, (day, actual1, actual2)) in days.iter().enumerate() {
        let solution = solutions
            .iter()
            .find(|s| s.day == *day)
            .expect("Day not found in config");

        let (part1_symbol, part1_msg) = part_info(actual1, &solution.part1);
        let (part2_symbol, part2_msg) = part_info(actual2, &solution.part2);

        println!("Day {day}:");
        println!("├─ Part 1: {part1_symbol} • {part1_msg}");
        println!("╰─ Part 2: {part2_symbol} • {part2_msg}");

        if i < days.len() - 1 {
            println!("");
        }
    }
}
