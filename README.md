# Audit

<div align="center">
  <img src="./logo.png" width="400" />
</div>

Audit is a CLI tool that helps you validate and refactor your [advent of code](https://adventofcode.com/) solutions. Supports any language, and can be used as a pipe.

> [!WARNING]
> Audit doesn't download inputs or check your answers on advent of code; it's used to refactor your code once you've came up with an initial solution.

## Installation

You can use `cargo install audit-cli` to install the CLI tool on your system, with the `aud` alias (given you have Rust installed)

## Usage

1. Create a configuration file in your project (default: `.audit.toml`)
2. Run `aud` with your advent of code program, such as `aud python day1.py`

> [!TIP]
> You can also pass `aud` as a pipe, such as `python day1.py | aud`

### Configuration

You can specify a custom configuration path with the `-c/--config` flag, such as `python day1.py | aud -c audit.toml`

```toml
"$schema" = "https://raw.githubusercontent.com/durocodes/audit/main/schema.json" # <- optional editor support

[[solutions]]
day = 1
part1 = "123"
part2 = "456"

[[solutions]]
day = 2
part1 = "789"
part2 = "012"

# ...
```

## Example

You can find a minimal example in the `example` directory, which contains a Python "solution" and a configuration file.

The example can be run with `python runner.py | aud` (make sure you're in the `example` directory)
