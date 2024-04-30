# Real-Time Stock Ticker in Rust

This project showcases a real-time stock ticker application built using Rust. It fetches stock data from the Alpha Vantage API and displays it in a formatted table using the `prettytable` crate.

## Features

- Retrieves stock data such as current price, industry, P/E ratio, ROE, and ROCE.
- Utilizes the Alpha Vantage API for real-time stock information.
- Demonstrates JSON deserialization and data processing in Rust.
- Displays data in a tabular format using the `prettytable` crate.

## Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/real-time-stock-ticker-rust.git
   ```

2. Navigate to the project directory:

   ```bash
   cd real-time-stock-ticker-rust
   ```

3. Install dependencies (ensure you have Rust and Cargo installed):

   ```bash
   cargo build
   ```

4. Set your Alpha Vantage API key:

   - Obtain an API key from [Alpha Vantage](https://www.alphavantage.co/).
   - Set the API key as an environment variable:

     ```bash
     export API_KEY="your-api-key"
     ```

5. Run the application:

   ```bash
   cargo run
   ```

## Dependencies

- `reqwest`: HTTP client for Rust.
- `serde`: Serialization and deserialization framework.
- `prettytable`: Table formatting library for CLI applications.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
