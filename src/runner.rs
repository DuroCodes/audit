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
    let mut results = Vec::new();
    let mut current_day = 0;
    let mut part1 = None;
    let mut part2 = None;

    for line in reader.lines().filter_map(Result::ok) {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        match parts.as_slice() {
            ["Day", d] => {
                if current_day > 0 {
                    results.push((current_day, part1.take(), part2.take()));
                }
                current_day = d.parse().map_err(|e| format!("Failed to parse day: {e}"))?;
            }
            ["Part", "1:", actual] => part1 = Some(actual.to_string()),
            ["Part", "2:", actual] => part2 = Some(actual.to_string()),
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
