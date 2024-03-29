/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CashbackObject : Provides information about an instant reimbursement in the form of cashback. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashbackObject {
    /// A brief description of why this cashback was paid. 
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "amount")]
    pub amount: Box<crate::models::MoneyObject>,
}

impl CashbackObject {
    /// Provides information about an instant reimbursement in the form of cashback. 
    pub fn new(description: String, amount: crate::models::MoneyObject) -> CashbackObject {
        CashbackObject {
            description,
            amount: Box::new(amount),
        }
    }
}


