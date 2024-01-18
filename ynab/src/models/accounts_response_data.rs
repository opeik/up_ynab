/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.ynab.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountsResponseData {
    #[serde(rename = "accounts")]
    pub accounts: Vec<crate::models::Account>,
    /// The knowledge of the server
    #[serde(rename = "server_knowledge")]
    pub server_knowledge: i64,
}

impl AccountsResponseData {
    pub fn new(accounts: Vec<crate::models::Account>, server_knowledge: i64) -> AccountsResponseData {
        AccountsResponseData {
            accounts,
            server_knowledge,
        }
    }
}


