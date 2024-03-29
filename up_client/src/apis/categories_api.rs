/*
 * Up API
 *
 * The Up API gives you programmatic access to your balances and transaction data. You can request past transactions or set up webhooks to receive real-time events when new transactions hit your account. It’s new, it’s exciting and it’s just the beginning. 
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`categories_get`]
#[derive(Clone, Debug)]
pub struct CategoriesGetParams {
    /// The unique identifier of a parent category for which to return only its children. Providing an invalid category identifier results in a `404` response. 
    pub filter_left_square_bracket_parent_right_square_bracket: Option<String>
}

/// struct for passing parameters to the method [`categories_id_get`]
#[derive(Clone, Debug)]
pub struct CategoriesIdGetParams {
    /// The unique identifier for the category. 
    pub id: String
}

/// struct for passing parameters to the method [`transactions_transaction_id_relationships_category_patch`]
#[derive(Clone, Debug)]
pub struct TransactionsTransactionIdRelationshipsCategoryPatchParams {
    /// The unique identifier for the transaction. 
    pub transaction_id: String,
    pub update_transaction_category_request: Option<crate::models::UpdateTransactionCategoryRequest>
}


/// struct for typed errors of method [`categories_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CategoriesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`categories_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CategoriesIdGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transactions_transaction_id_relationships_category_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionsTransactionIdRelationshipsCategoryPatchError {
    UnknownValue(serde_json::Value),
}


/// Retrieve a list of all categories and their ancestry. The returned list is not paginated. 
pub async fn categories_get(configuration: &configuration::Configuration, params: CategoriesGetParams) -> Result<crate::models::ListCategoriesResponse, Error<CategoriesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let filter_left_square_bracket_parent_right_square_bracket = params.filter_left_square_bracket_parent_right_square_bracket;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/categories", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter_left_square_bracket_parent_right_square_bracket {
        local_var_req_builder = local_var_req_builder.query(&[("filter[parent]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<CategoriesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a specific category by providing its unique identifier. 
pub async fn categories_id_get(configuration: &configuration::Configuration, params: CategoriesIdGetParams) -> Result<crate::models::GetCategoryResponse, Error<CategoriesIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/categories/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        let local_var_entity: Option<CategoriesIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates the category associated with a transaction. Only transactions for which `isCategorizable` is set to true support this operation. The `id` is taken from the list exposed on `/categories` and cannot be one of the top-level (parent) categories. To de-categorize a transaction, set the entire `data` key to `null`. An HTTP `204` is returned on success. The associated category, along with its request URL is also exposed via the `category` relationship on the transaction resource returned from `/transactions/{id}`. 
pub async fn transactions_transaction_id_relationships_category_patch(configuration: &configuration::Configuration, params: TransactionsTransactionIdRelationshipsCategoryPatchParams) -> Result<(), Error<TransactionsTransactionIdRelationshipsCategoryPatchError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let transaction_id = params.transaction_id;
    let update_transaction_category_request = params.update_transaction_category_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/transactions/{transactionId}/relationships/category", local_var_configuration.base_path, transactionId=crate::apis::urlencode(transaction_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&update_transaction_category_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<TransactionsTransactionIdRelationshipsCategoryPatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

