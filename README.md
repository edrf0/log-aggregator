# LogAggregator

A high-performance, CLI-based log parsing and filtering tool written in Rust. Designed for efficiency, it allows you to scan massive JSONL log files using literal keywords or complex Regex patterns.

## Features

* **Fast Filtering:** Scans raw text before deserializing JSON, significantly reducing CPU overhead.
* **Dual-Mode Matching:** Supports both simple substring searches and advanced Regex patterns.
* **Memory Efficient:** Uses buffered I/O (`BufReader`/`BufWriter`) to handle files much larger than available RAM.
* **Flexible Output:** Print results directly to your terminal or save them to a file.
* **Built-in Generator:** Includes a test log generator to help you benchmark performance.
