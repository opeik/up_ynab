use std::pin::Pin;

use chrono::{DateTime, FixedOffset};
use color_eyre::eyre::{Context, Result};
use futures::{
    stream::{self},
    Stream,
};
use up_client::{
    apis::{accounts_api, configuration::Configuration, transactions_api, util, Error},
    models,
};

#[derive(Debug, Clone)]
pub struct Client {
    config: Configuration,
}

pub type AccountKind = up_client::models::AccountTypeEnum;
pub type OwnershipKind = up_client::models::OwnershipTypeEnum;
pub type TransactionState = up_client::models::TransactionStatusEnum;

const PAGE_SIZE: i32 = 100;

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(pattern = "owned", setter(into), build_fn(private))]
pub struct GetTransactionsParams<'a> {
    #[builder(private)]
    client: &'a Client,
    /// The transaction status for which to return records. This can be used to filter `HELD`
    /// transactions from those that are `SETTLED`.
    #[builder(default)]
    filter_status: Option<TransactionState>,
    /// The start date-time from which to return records, formatted according to rfc-3339. Not to
    /// be used for pagination purposes.
    #[builder(default)]
    filter_since: Option<DateTime<FixedOffset>>,
    /// The end date-time up to which to return records, formatted according to rfc-3339. Not to be
    /// used for pagination purposes.
    #[builder(default)]
    filter_until: Option<DateTime<FixedOffset>>,
    /// The category identifier for which to filter transactions. Both parent and child categories
    /// can be filtered through this parameter. Providing an invalid category identifier results in
    /// a `404` response.
    #[builder(default)]
    filter_category: Option<String>,
    /// A transaction tag to filter for which to return records. If the tag does not exist, zero
    /// records are returned and a success response is given.
    #[builder(default)]
    filter_tag: Option<String>,
}

#[derive(Debug, Clone, derive_builder::Builder)]
#[builder(pattern = "owned", setter(into), build_fn(private))]
pub struct GetAccountsParams<'a> {
    #[builder(private)]
    client: &'a Client,
    /// The type of account for which to return records. This can be used to filter Savers from
    /// spending accounts.
    #[builder(default)]
    filter_type: Option<AccountKind>,
    /// The account ownership structure for which to return records. This can be used to filter 2Up
    /// accounts from Up accounts.
    #[builder(default)]
    filter_ownership: Option<OwnershipKind>,
}

macro_rules! stream_pages_impl {
    ($name:ident, $page_fn:ident, $A:ty, $T:ty, $E:ty) => {
        fn $name(&self, args: $A) -> Pin<Box<impl Stream<Item = Result<$T, Error<$E>>> + '_>> {
            use tracing::debug;

            struct State {
                data: <Vec<$T> as IntoIterator>::IntoIter,
                next: Option<String>,
                count: usize,
                args: $A,
            }

            let state = State {
                data: Vec::new().into_iter(),
                next: None,
                count: 0,
                args,
            };

            let items = stream::try_unfold(state, move |mut state: State| async move {
                if let Some(x) = state.data.next() {
                    return Ok(Some((x, state)));
                } else if state.next.is_none() && state.count > 0 {
                    return Ok(None);
                }

                debug!("fetching page {}", state.count);
                let response = self
                    .$page_fn(state.next.as_deref(), state.args.clone())
                    .await?;
                state.data = response.data.into_iter();
                state.next = response.links.next;
                state.count += 1;
                Ok(state.data.next().map(|x| (x, state)))
            });

            Box::pin(items)
        }
    };
}

impl<'a> GetAccountsParams<'a> {
    fn into_api(self) -> accounts_api::AccountsGetParams {
        accounts_api::AccountsGetParams {
            page_size: Some(PAGE_SIZE),
            filter_type: self.filter_type,
            filter_ownership: self.filter_ownership,
        }
    }
}

impl<'a> GetTransactionsParams<'a> {
    fn into_api(self) -> transactions_api::TransactionsGetParams {
        transactions_api::TransactionsGetParams {
            page_size: Some(PAGE_SIZE),
            filter_status: self.filter_status,
            filter_since: self.filter_since.map(|x| x.to_rfc3339()),
            filter_until: self.filter_until.map(|x| x.to_rfc3339()),
            filter_category: self.filter_category,
            filter_tag: self.filter_tag,
        }
    }
}

type TransactionsResult =
    Result<models::TransactionResource, Error<transactions_api::TransactionsGetError>>;
impl<'a> GetTransactionsParamsBuilder<'a> {
    pub fn send(self) -> Result<Pin<Box<impl Stream<Item = TransactionsResult> + 'a>>> {
        let params = self.build().wrap_err("failed to build parameters")?;
        Ok(params.client.transactions_send(params.into_api()))
    }
}

type AccountsResult = Result<models::AccountResource, Error<accounts_api::AccountsGetError>>;
impl<'a> GetAccountsParamsBuilder<'a> {
    pub fn send(self) -> Result<Pin<Box<impl Stream<Item = AccountsResult> + 'a>>> {
        let params = self.build().wrap_err("failed to build parameters")?;
        Ok(params.client.accounts_send(params.into_api()))
    }
}

impl Client {
    pub fn new(api_token: &str) -> Self {
        Self {
            config: Configuration {
                bearer_access_token: Some(api_token.to_owned()),
                ..Default::default()
            },
        }
    }

    pub fn transactions(&self) -> GetTransactionsParamsBuilder<'_> {
        GetTransactionsParamsBuilder {
            client: Some(self),
            ..Default::default()
        }
    }

    pub fn accounts(&self) -> GetAccountsParamsBuilder<'_> {
        GetAccountsParamsBuilder {
            client: Some(self),
            ..Default::default()
        }
    }

    stream_pages_impl!(
        transactions_send,
        transactions_page,
        transactions_api::TransactionsGetParams,
        models::TransactionResource,
        transactions_api::TransactionsGetError
    );

    stream_pages_impl!(
        accounts_send,
        accounts_page,
        accounts_api::AccountsGetParams,
        models::AccountResource,
        accounts_api::AccountsGetError
    );

    async fn transactions_page(
        &self,
        page: Option<&str>,
        params: transactions_api::TransactionsGetParams,
    ) -> Result<models::ListTransactionsResponse, Error<transactions_api::TransactionsGetError>>
    {
        if let Some(page) = page {
            util::get_page::<
                models::ListTransactionsResponse,
                transactions_api::TransactionsGetError,
            >(&self.config, page)
            .await
        } else {
            transactions_api::transactions_get(&self.config, params).await
        }
    }

    async fn accounts_page(
        &self,
        page: Option<&str>,
        params: accounts_api::AccountsGetParams,
    ) -> Result<models::ListAccountsResponse, Error<accounts_api::AccountsGetError>> {
        if let Some(page) = page {
            util::get_page::<models::ListAccountsResponse, accounts_api::AccountsGetError>(
                &self.config,
                page,
            )
            .await
        } else {
            accounts_api::accounts_get(&self.config, params).await
        }
    }
}
