use log::{info, warn};
use simplelog::SimpleLogger;
use simplelog::{Config, LevelFilter};
use clap::Parser;
use std::io;

/// Sum a column of output from stdin
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Column number to sum, 0-indexed
    #[arg(short, long, default_value_t = 0)]
    column: usize,

    /// warnings to stdout
    #[arg(short, long, default_value_t = false)]
    warn: bool,

    /// debug to stdout
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    setup_logger(args.debug, args.warn);

    let mut sum = 0.;

    let lines = io::stdin().lines();
    for (i, some_line) in lines.enumerate() {
        let line = some_line.unwrap();
        info!("[{}] Line: {}", i, line);

        if let Some(extract) = extract_value_from_line(line, args.column, i) {
            info!("[{}] Extract: {}", i, extract);

            sum += extract;
        };
    }

    println!("{}", sum);
}

fn setup_logger(debug: bool, warn: bool) {
    let mut level = LevelFilter::Error;

    if debug  {
        level = LevelFilter::Debug;
    } else if warn {
        level = LevelFilter::Warn;
    }

    let _ = SimpleLogger::init(level, Config::default());
}

fn extract_value_from_line(line: String, position: usize, line_number: usize) -> Option<f32> {
    let cols: Vec<&str> = line.split_whitespace().collect();

    if let Some(element) = cols.get(position) {
        if let Ok(parsed) = element.parse::<f32>() {
            return Some(parsed)
        }

        warn!("[{}] Hard to parse: {}", line_number, element);
        return None;
    }

    warn!("[{}] Empty Column {}", line_number, position);
    return None;
}