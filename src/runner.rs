use regex::Regex;
use std::{
    io::{self, BufRead, BufReader},
    process::{Command, Stdio},
};

pub type DayOutput = (u8, Option<String>, Option<String>);

pub fn from_pipe() -> Result<Vec<DayOutput>, String> {
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());
    parse_output(reader)
}

pub fn from_command(args: &[String]) -> Result<Vec<DayOutput>, String> {
    if args.is_empty() {
        return Err("No command provided".into());
    }

    let process = Command::new(&args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start process: {e}"))?;

    let stdout = process.stdout.ok_or("Failed to capture stdout")?;
    let reader = BufReader::new(stdout);
    parse_output(reader)
}

fn parse_output<R: BufRead>(reader: R) -> Result<Vec<DayOutput>, String> {
    let day_re = Regex::new(r"Day\s+(\d+)").unwrap();
    let part1_re = Regex::new(r"Part\s+1:\s+(.+)").unwrap();
    let part2_re = Regex::new(r"Part\s+2:\s+(.+)").unwrap();

    let mut results = Vec::new();
    let mut current_day = 0;
    let mut part1 = None;
    let mut part2 = None;

    for line in reader.lines().filter_map(Result::ok) {
        match (
            day_re.captures(&line),
            part1_re.captures(&line),
            part2_re.captures(&line),
        ) {
            (Some(cap), _, _) => {
                if current_day > 0 {
                    results.push((current_day, part1.take(), part2.take()));
                }
                current_day = cap[1]
                    .parse()
                    .map_err(|e| format!("Failed to parse day: {e}"))?;
            }
            (_, Some(cap), _) => part1 = Some(cap[1].to_string()),
            (_, _, Some(cap)) => part2 = Some(cap[1].to_string()),
            _ => continue,
        }
    }

    if current_day > 0 {
        results.push((current_day, part1, part2));
    }

    if results.is_empty() {
        Err("Failed to find valid day".into())
    } else {
        Ok(results)
    }
}
