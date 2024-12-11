# NEET Scorecard Scraper

This project is a Rust-based web scraping tool designed to retrieve NEET scorecard data programmatically. It sends POST requests to the official NEET website with various combinations of application numbers and birthdates, parses the HTML responses, and extracts relevant details such as the application number, candidate's name, rank, and total marks obtained.

## Features

- **Web Scraping**: Uses `reqwest` for HTTP requests and `scraper` for HTML parsing.
- **Asynchronous Execution**: Built with `tokio` for handling multiple requests concurrently.
- **Robust Parsing**: Extracts scorecard details using precise selectors and handles missing data gracefully.
- **Error Handling**: Implements basic error handling for network and parsing errors.

## Prerequisites

- **Rust**: Ensure you have Rust and Cargo installed. You can install Rust using [rustup](https://rustup.rs/).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- **Dependencies**: The following crates are used:
  - `reqwest`: For making HTTP requests.
  - `scraper`: For parsing HTML documents.
  - `tokio`: For asynchronous runtime support.

Install dependencies using Cargo:

```bash
cargo build
```

## Setup and Usage

1. **Clone the Repository**

   ```bash
   git clone https://github.com/ANIR1604/NeetScrapper.git
   cd NeetScrapper
   ```

2. **Build the Project**

   Compile the project and ensure dependencies are resolved:

   ```bash
   cargo build
   ```

3. **Run the Scraper**

   Execute the scraper with:

   ```bash
   cargo run
   ```

   The tool will iterate through application numbers and date ranges, logging parsed data to the console.

## Code Overview

### Main Components

1. **HTTP Requests**:
   - The `reqwest::Client` is used to send POST requests to the NEET scorecard endpoint.

2. **HTML Parsing**:
   - `scraper` is used to locate and extract specific data fields (e.g., application number, rank).

3. **Concurrency**:
   - Asynchronous functions and `tokio` are utilized to handle multiple requests simultaneously.

4. **Error Handling**:
   - The program gracefully handles network timeouts and parsing failures by skipping invalid responses.

### Key Functions

- `solve`: Sends a POST request with specific form data and parses the response.
- `parse_html`: Extracts scorecard data from HTML content.
- `main_loop`: Iterates through possible date combinations and gathers data for a given application number.
- `solve_all_applications`: Handles multiple application numbers sequentially.

### Data Structure

The `ParsedData` struct represents the extracted details:

```rust
struct ParsedData {
    application_number: String,
    candidate_name: String,
    all_india_rank: String,
    marks: String,
}
```

## Notes

- **Rate Limiting**: The tool does not currently handle rate limiting. Be cautious of sending too many requests to avoid being blocked.
- **Legal Considerations**: Ensure that your use of this scraper complies with the terms of service of the NEET website.

## Dependencies

The project uses the following dependencies:

```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
scraper = "0.14"
tokio = { version = "1", features = ["full"] }
```

## Disclaimer

This tool is intended for educational purposes only. The developers are not responsible for any misuse or legal issues arising from its use.

---

Happy scraping!


