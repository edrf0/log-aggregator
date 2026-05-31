mod parser;
mod generator;

use std::error::Error;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Generate test log file
    #[arg(short, long, default_value_t = false)]
    generate: bool,
    /// Optional test log file size in KB
    #[arg(short, long)]
    logsize: Option<usize>,
    /// Optional output path for saving the parsed log
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// The path to the log file
    #[arg(long)]
    path: PathBuf,
    /// Optional search pattern
    #[arg(long)]
    pattern: Option<String>,
    /// Ignore case flag
    #[arg(short, long, default_value_t = false)]
    ignorecase: bool,
    /// Use regex for pattern matching
    #[arg(short, long, default_value_t = false)]
    regex: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = Args::parse();
    // Generate test log file if true
    if args.generate {
        // Checking if user inserted a size for the log
        if args.logsize.is_none() {
            // If the user did not then it will generate a 10 KB log file
            args.logsize = Some(10);
        }
        // Generating the log file
        match generator::generate(args.logsize.unwrap()) {
            Ok(()) => {
                println!("Successfully generated test_log.json in current directory.\n");
                io::stdout().flush()?;
            },
            Err(e) => {
                eprintln!("{}", e);
                io::stdout().flush()?;
            }
        }
        return Ok(());
    }
    // Checking for pattern presence before searching the log file
    if args.pattern.is_none() {
        println!("Please provide a pattern.\n");
        io::stdout().flush()?;
        return Ok(());
    }
    // Search for regex pattern on log file
    if let Err(e) =
        parser::parse_log_file(args.path,args.pattern.unwrap(),args.regex,args.output,args.ignorecase) {
        eprintln!("{}", e);
        io::stdout().flush()?;
    }
    Ok(())
}
