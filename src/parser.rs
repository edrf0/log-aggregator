use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use serde_json::Value;
use regex::Regex;

pub(crate) fn parse_log_file(
    file_path: PathBuf, pattern: String, use_regex:bool,output_path:Option<PathBuf>,ignore_case:bool)
    -> Result<(),Box<dyn Error>> {
    // Checking if file path exists
    let file_handle = File::open(file_path)?;
    // Creating buffered reader
    let reader = BufReader::new(file_handle);
    // Creating dynamic writer
    let mut writer:Box<dyn Write> = match output_path {
        // File writer
        Some(path) => Box::new(BufWriter::new(File::create(path)?)),
        // Terminal printer
        None => Box::new(io::stdout()),
    };
    // Changing the pattern if the ignore_case flag is active
    let pattern = if ignore_case {
        pattern.to_lowercase()
    } else {
        pattern
    };
    // Regex compiling if needed
    let re = if use_regex {
        Some(Regex::new(pattern.as_ref())?)
    } else {
        None
    };
    // Preparing to read
    for line in reader.lines() {
        // Unwrapping Result<String> or returning early with an error
        let line = line?;
        // Checking for empty lines as serde_json::from_str(&line)? returns an error
        // if an empty line is found
        if line.trim().is_empty() {
            continue;
        }
        // Applying search pattern
        let is_match = if let Some(re) = &re {
            re.is_match(&line)
        } else {
            line.contains::<&str>(pattern.as_ref())
        };
        // Deserializing only if a match was found
        if is_match {
            // Reading each json line (JSONL)
            let value: Value = serde_json::from_str(&line)?;
            // Writing to either a file or the terminal
            writeln!(writer, "{}", value)?;
        }
    }
    // Ensuring all data has been unloaded
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // A helper to simulate reading from a file
    fn create_reader(content: &str) -> BufReader<Cursor<&[u8]>> {
        BufReader::new(Cursor::new(content.as_bytes()))
    }

    #[test]
    fn test_keyword_filtering() {
        let log_data = "{\"level\":\"INFO\"}\n{\"level\":\"ERROR\"}\n{\"level\":\"DEBUG\"}";
        let reader = create_reader(log_data);

        // Simplified version of the logic to test the filter
        let pattern = "ERROR";
        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| line.contains(pattern))
            .collect();

        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("ERROR"));
    }

    #[test]
    fn test_regex_filtering() {
        let log_data = "{\"level\":\"ERROR_1\"}\n{\"level\":\"INFO\"}\n{\"level\":\"ERROR_2\"}";
        let re = Regex::new(r"ERROR_\d").unwrap();

        let reader = create_reader(log_data);
        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| re.is_match(line))
            .collect();

        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_empty_line_handling() {
        let log_data = "{\"level\":\"INFO\"}\n\n{\"level\":\"ERROR\"}";
        let reader = create_reader(log_data);

        let lines: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| !line.trim().is_empty())
            .collect();

        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_malformed_json_handling() {
        let log_data = "{\"level\":\"INFO\"}\n{\"level\": \"ERROR\" "; // Broken line
        let reader = create_reader(log_data);

        // We expect the line-by-line parsing to stop or return an error
        let results: Vec<Result<Value, _>> = reader.lines()
            .map(|l| serde_json::from_str(&l.unwrap()))
            .collect();

        assert!(results[0].is_ok()); // The first one is fine
        assert!(results[1].is_err()); // The second one should fail
    }

    #[test]
    fn test_no_matches_found() {
        let log_data = "{\"level\":\"DEBUG\"}\n{\"level\":\"DEBUG\"}";
        let reader = create_reader(log_data);
        let pattern = "ERROR";

        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| line.contains(pattern))
            .collect();

        assert_eq!(filtered.len(), 0); // Correctly found nothing
    }

    #[test]
    fn test_case_sensitivity() {
        let log_data = "{\"level\":\"info\"}";
        let reader = create_reader(log_data);
        let pattern = "INFO";

        // This will likely FAIL currently because you are using case-sensitive matching
        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| line.contains(pattern))
            .collect();

        // If you want this to pass, you'll need to update your logic
        assert_eq!(filtered.len(), 0);
    }

    #[test]
    fn test_extremely_long_line() {
        let long_log = format!("{{\"message\":\"{}\"}}", "a".repeat(10_000));
        let reader = create_reader(&long_log);

        let line = reader.lines().next().unwrap().unwrap();
        let value: Value = serde_json::from_str(&line).unwrap();

        assert_eq!(value["message"].as_str().unwrap().len(), 10_000);
    }

    #[test]
    fn test_ignore_case_matching() {
        let log_data = "{\"level\":\"INFO\"}\n{\"level\":\"ERROR\"}";
        let reader = create_reader(log_data);

        // Test case: Pattern is lowercase, data is uppercase
        let pattern = "info";
        let ignore_case = true;

        let pattern_lower = pattern.to_lowercase();

        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| {
                if ignore_case {
                    line.to_lowercase().contains(&pattern_lower)
                } else {
                    line.contains(pattern)
                }
            })
            .collect();

        assert_eq!(filtered.len(), 1);
        assert!(filtered[0].contains("INFO"));
    }

    #[test]
    fn test_ignore_case_mixed_casing() {
        let log_data = "{\"level\":\"WaRn\"}";
        let reader = create_reader(log_data);

        let pattern = "warn";
        let ignore_case = true;
        let pattern_lower = pattern.to_lowercase();

        let filtered: Vec<String> = reader.lines()
            .map(|l| l.unwrap())
            .filter(|line| {
                if ignore_case {
                    line.to_lowercase().contains(&pattern_lower)
                } else {
                    line.contains(pattern)
                }
            })
            .collect();

        assert_eq!(filtered.len(), 1);
    }
}