/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CurrencyFormat : The currency format setting for the budget.  In some cases the format will not be available and will be specified as null.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrencyFormat {
    #[serde(rename = "iso_code")]
    pub iso_code: String,
    #[serde(rename = "example_format")]
    pub example_format: String,
    #[serde(rename = "decimal_digits")]
    pub decimal_digits: i32,
    #[serde(rename = "decimal_separator")]
    pub decimal_separator: String,
    #[serde(rename = "symbol_first")]
    pub symbol_first: bool,
    #[serde(rename = "group_separator")]
    pub group_separator: String,
    #[serde(rename = "currency_symbol")]
    pub currency_symbol: String,
    #[serde(rename = "display_symbol")]
    pub display_symbol: bool,
}

impl CurrencyFormat {
    /// The currency format setting for the budget.  In some cases the format will not be available and will be specified as null.
    pub fn new(iso_code: String, example_format: String, decimal_digits: i32, decimal_separator: String, symbol_first: bool, group_separator: String, currency_symbol: String, display_symbol: bool) -> CurrencyFormat {
        CurrencyFormat {
            iso_code,
            example_format,
            decimal_digits,
            decimal_separator,
            symbol_first,
            group_separator,
            currency_symbol,
            display_symbol,
        }
    }
}


