use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};
use std::time::Instant;
use chrono::Utc;
use rand::RngExt;

fn generate_test_log(log_size_bytes:usize) -> io::Result<()> {
    let file = File::create("test_log.json")?;
    // Setting the write buffer to 64 KB considering performance
    // factors like CPU cache and NVMe SSD writing operations
    let mut write_buffer = BufWriter::with_capacity(64 * 1024, file);

    // These will be the labels for the log file
    let levels = ["INFO", "WARN", "ERROR", "DEBUG"];
    let modules = ["auth_service", "payment_api", "db_pool", "cache_worker"];
    let messages = [
        "Connection timed out",
        "User login successful",
        "Cache eviction policy triggered",
        "Failed to process transaction ID"
    ];

    // The rand crate will be used to diversify the logs labels
    let mut rng = rand::rng();

    // This counter tracks the number of bytes written
    let mut bytes_written = 0;

    // Log total time generation is tracked starting from here
    let start_time = Instant::now();

    while bytes_written < log_size_bytes {
        // Formatting the next log line to be written
        let log_line = format!(
            "{{\"timestamp\":\"{}\",\"level\":\"{}\",\"module\":\"{}\",\"message\":\"{}\"}}\n",
            Utc::now().to_rfc3339(),
            levels[rng.random_range(0..levels.len())],
            modules[rng.random_range(0..modules.len())],
            messages[rng.random_range(0..messages.len())]
        );
        // Converting to a byte slice for writing
        let bytes = log_line.as_bytes();
        // Performing write operation and incrementing the counter
        bytes_written += write_buffer.write(bytes)?;
    }
    // Ensuring buffer contents are flushed properly
    write_buffer.flush()?;

    println!("Generated {} bytes log file in {:?}", bytes_written, start_time.elapsed());
    io::stdout().flush()?;

    Ok(())
}

pub(crate) fn generate(log_size: usize) -> Result<(),io::Error> {
    match log_size {
        // At least 1 KB
        0 => generate_test_log(1 * 1024),
        // 1 KB to 4 GB (1 KB * 4 * 1024 * 1024)
        size @ 1..=4_194_304 => generate_test_log(size * 1024),
        // Capping to 4 GB
        _ => generate_test_log(4_194_304 * 1024),
    }
}