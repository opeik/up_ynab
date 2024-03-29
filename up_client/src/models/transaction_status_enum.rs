/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// TransactionStatusEnum : Specifies which stage of processing a transaction is currently at. Currently returned values are `HELD` and `SETTLED`. When a transaction is held, its account’s `availableBalance` is affected. When a transaction is settled, its account’s `currentBalance` is affected. 

/// Specifies which stage of processing a transaction is currently at. Currently returned values are `HELD` and `SETTLED`. When a transaction is held, its account’s `availableBalance` is affected. When a transaction is settled, its account’s `currentBalance` is affected. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionStatusEnum {
    #[serde(rename = "HELD")]
    Held,
    #[serde(rename = "SETTLED")]
    Settled,

}

impl ToString for TransactionStatusEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Held => String::from("HELD"),
            Self::Settled => String::from("SETTLED"),
        }
    }
}

impl Default for TransactionStatusEnum {
    fn default() -> TransactionStatusEnum {
        Self::Held
    }
}




