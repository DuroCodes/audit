mod config;
mod runner;
mod validator;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(
        short,
        long,
        default_value = "audit.toml",
        help = "Path to config file (default: audit.toml)"
    )]
    config: String,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let config = match config::Config::from_file(&cli.config) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Config Error: {e}");
            std::process::exit(1);
        }
    };

    let result = match (cli.command.is_empty(), atty::is(atty::Stream::Stdin)) {
        (true, true) => Err("No input provided. Use either a pipe or provide a command.".into()),
        (true, false) => runner::from_pipe(),
        (false, _) => runner::from_command(&cli.command),
    };

    match result {
        Ok(outputs) => validator::validate_parts(&outputs, &config.solutions),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
