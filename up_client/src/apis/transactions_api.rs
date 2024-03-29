/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can
 * request past transactions or set up webhooks to receive real-time events when new transactions
 * hit your account. It’s new, it’s exciting and it’s just the beginning.
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;
use serde::Deserialize;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models::TransactionStatusEnum};

/// struct for passing parameters to the method [`accounts_account_id_transactions_get`]
#[derive(Clone, Debug)]
pub struct AccountsAccountIdTransactionsGetParams {
    /// The unique identifier for the account.
    pub account_id: String,
    /// The number of records to return in each page.
    pub page_size: Option<i32>,
    /// The transaction status for which to return records. This can be used to filter `HELD`
    /// transactions from those that are `SETTLED`.
    pub filter_status: Option<TransactionStatusEnum>,
    /// The start date-time from which to return records, formatted according to rfc-3339. Not to
    /// be used for pagination purposes.
    pub filter_since: Option<String>,
    /// The end date-time up to which to return records, formatted according to rfc-3339. Not to be
    /// used for pagination purposes.
    pub filter_until: Option<String>,
    /// The category identifier for which to filter transactions. Both parent and child categories
    /// can be filtered through this parameter. Providing an invalid category identifier results in
    /// a `404` response.
    pub filter_category: Option<String>,
    /// A transaction tag to filter for which to return records. If the tag does not exist, zero
    /// records are returned and a success response is given.
    pub filter_tag: Option<String>,
}

/// struct for passing parameters to the method [`transactions_get`]
#[derive(Clone, Debug)]
pub struct TransactionsGetParams {
    /// The number of records to return in each page.
    pub page_size: Option<i32>,
    /// The transaction status for which to return records. This can be used to filter `HELD`
    /// transactions from those that are `SETTLED`.
    pub filter_status: Option<TransactionStatusEnum>,
    /// The start date-time from which to return records, formatted according to rfc-3339. Not to
    /// be used for pagination purposes.
    pub filter_since: Option<String>,
    /// The end date-time up to which to return records, formatted according to rfc-3339. Not to be
    /// used for pagination purposes.
    pub filter_until: Option<String>,
    /// The category identifier for which to filter transactions. Both parent and child categories
    /// can be filtered through this parameter. Providing an invalid category identifier results in
    /// a `404` response.
    pub filter_category: Option<String>,
    /// A transaction tag to filter for which to return records. If the tag does not exist, zero
    /// records are returned and a success response is given.
    pub filter_tag: Option<String>,
}

/// struct for passing parameters to the method [`transactions_id_get`]
#[derive(Clone, Debug)]
pub struct TransactionsIdGetParams {
    /// The unique identifier for the transaction.
    pub id: String,
}

/// struct for typed errors of method [`accounts_account_id_transactions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountsAccountIdTransactionsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsIdGetError {
    UnknownValue(serde_json::Value),
}

/// Retrieve a list of all transactions for a specific account. The returned list is
/// [paginated](#pagination) and can be scrolled by following the `next` and `prev` links where
/// present. To narrow the results to a specific date range pass one or both of `filter[since]` and
/// `filter[until]` in the query string. These filter parameters **should not** be used for
/// pagination. Results are ordered newest first to oldest last.
pub async fn accounts_account_id_transactions_get(
    configuration: &configuration::Configuration,
    params: AccountsAccountIdTransactionsGetParams,
) -> Result<crate::models::ListTransactionsResponse, Error<AccountsAccountIdTransactionsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let page_left_square_bracket_size_right_square_bracket = params.page_size;
    let filter_left_square_bracket_status_right_square_bracket = params.filter_status;
    let filter_left_square_bracket_since_right_square_bracket = params.filter_since;
    let filter_left_square_bracket_until_right_square_bracket = params.filter_until;
    let filter_left_square_bracket_category_right_square_bracket = params.filter_category;
    let filter_left_square_bracket_tag_right_square_bracket = params.filter_tag;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/accounts/{accountId}/transactions",
        local_var_configuration.base_path,
        accountId = crate::apis::urlencode(account_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_left_square_bracket_size_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("page[size]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_status_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[status]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_since_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[since]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_until_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[until]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_category_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[category]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_tag_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[tag]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AccountsAccountIdTransactionsGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a list of all transactions across all accounts for the currently authenticated user.
/// The returned list is [paginated](#pagination) and can be scrolled by following the `next` and
/// `prev` links where present. To narrow the results to a specific date range pass one or both of
/// `filter[since]` and `filter[until]` in the query string. These filter parameters **should not**
/// be used for pagination. Results are ordered newest first to oldest last.
pub async fn transactions_get(
    configuration: &configuration::Configuration,
    params: TransactionsGetParams,
) -> Result<crate::models::ListTransactionsResponse, Error<TransactionsGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_left_square_bracket_size_right_square_bracket = params.page_size;
    let filter_left_square_bracket_status_right_square_bracket = params.filter_status;
    let filter_left_square_bracket_since_right_square_bracket = params.filter_since;
    let filter_left_square_bracket_until_right_square_bracket = params.filter_until;
    let filter_left_square_bracket_category_right_square_bracket = params.filter_category;
    let filter_left_square_bracket_tag_right_square_bracket = params.filter_tag;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transactions", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_left_square_bracket_size_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("page[size]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_status_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[status]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_since_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[since]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_until_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[until]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_category_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[category]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_left_square_bracket_tag_right_square_bracket {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter[tag]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TransactionsGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a specific transaction by providing its unique identifier.
pub async fn transactions_id_get(
    configuration: &configuration::Configuration,
    params: TransactionsIdGetParams,
) -> Result<crate::models::GetTransactionResponse, Error<TransactionsIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/transactions/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TransactionsIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
