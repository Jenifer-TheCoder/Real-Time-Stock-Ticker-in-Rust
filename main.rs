use reqwest::Error;
use serde::Deserialize;
use prettytable::{Table, Row, Cell};
use prettytable::row;


#[derive(Debug, Deserialize)]
struct GlobalQuoteResponse {
    #[serde(rename = "Global Quote")]
    global_quote: GlobalQuote,
}
#[derive(Debug, Deserialize)]
struct GlobalQuote {
    // #[serde(rename = "01. symbol")]
    // symbol: String,
    #[serde(rename = "05. price")]
    price: String,
}
#[derive(Debug, Deserialize)]
struct Overview {
    #[serde(rename = "Symbol")]
    symbol: String,
    #[serde(rename = "Industry")]
    industry: String,
    #[serde(rename = "PERatio")]
    pe_ratio: String,
    #[serde(rename = "ReturnOnEquityTTM")]
    return_on_equity: String,
    #[serde(rename = "ReturnOnAssetsTTM")]
    return_on_assets: String,
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let symbol = "GOOGL";
    let api_key = std::env::var("API_KEY").expect("API_KEY environment variable not found");
    let global_quote_url = format!(
        "https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol={}&apikey={}",
        symbol, api_key
    );
    let overview_url = format!(
        "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
        symbol, api_key
    );
    let global_quote_response = reqwest::get(&global_quote_url).await?.json::<GlobalQuoteResponse>().await?;
    let overview_response = reqwest::get(&overview_url).await?.json::<Overview>().await?;
    // Create a table
    let mut table = Table::new();
    // Add headers
    table.add_row(row!["Stock Name", "Industry", "Current Price", "PE RATIO", "ROE", "ROA"]);
    // Add data rows
    //let symbol = global_quote_response.symbol.unwrap_or_else(|| "N/A".to_string()); // Using "N/A" if symbol is missing
    let price = if global_quote_response.global_quote.price.is_empty() {
        "N/A".to_string()
    } else {
        global_quote_response.global_quote.price
    };
    
    table.add_row(Row::new(vec![
        Cell::new(&overview_response.symbol),
        Cell::new(&overview_response.industry),
        Cell::new(&price),
        Cell::new(&overview_response.pe_ratio),
        Cell::new(&overview_response.return_on_equity),
        Cell::new(&overview_response.return_on_assets),
    ]));
    // Print the table
    table.printstd();
    Ok(())
}

