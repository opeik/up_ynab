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
pub struct PayeesResponseData {
    #[serde(rename = "payees")]
    pub payees: Vec<crate::models::Payee>,
    /// The knowledge of the server
    #[serde(rename = "server_knowledge")]
    pub server_knowledge: i64,
}

impl PayeesResponseData {
    pub fn new(payees: Vec<crate::models::Payee>, server_knowledge: i64) -> PayeesResponseData {
        PayeesResponseData {
            payees,
            server_knowledge,
        }
    }
}


