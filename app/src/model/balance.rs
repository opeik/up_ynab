use std::{collections::BTreeMap, fmt, fs::File, path::Path};

use color_eyre::eyre::{ContextCompat, Result};
use indoc::{formatdoc, writedoc};
use itertools::Itertools;
use money2::Money;

use crate::model::{transaction, Account, Transaction};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Balance<'a> {
    pub values: BTreeMap<Account, Money>,
    pub transaction: &'a Transaction,
}

#[must_use]
pub fn running_total(transactions: &[Transaction]) -> Vec<Balance> {
    let mut balances = Vec::<Balance>::new();
    let transactions = transactions
        .iter()
        .sorted_by(|a: &&Transaction, b| Ord::cmp(&a.timestamp, &b.timestamp));

    for transaction in transactions {
        let last_balance = balances.last().cloned().unwrap_or(Balance {
            values: BTreeMap::new(),
            transaction,
        });
        let mut new_values = last_balance.values;

        let to = match &transaction.kind {
            transaction::Kind::External { to, from_name: _ } => to,
            transaction::Kind::Internal { to, from } => {
                new_values
                    .entry(from.clone())
                    .and_modify(|x| *x -= transaction.amount)
                    .or_insert(transaction.amount);
                to
            }
        };

        new_values
            .entry(to.clone())
            .and_modify(|x| *x += transaction.amount)
            .or_insert(transaction.amount);

        let new_balance = Balance {
            values: new_values.clone(),
            transaction,
        };

        balances.push(new_balance);
    }

    balances
}

pub fn write_csv<P: AsRef<Path>>(balances: &[Balance], path: P) -> Result<()> {
    let accounts = balances
        .last()
        .cloned()
        .wrap_err("missing account")?
        .values
        .into_keys()
        .sorted_by(|a, b| Ord::cmp(&a.name, &b.name))
        .collect::<Vec<_>>();
    let accounts_str = accounts.iter().map(|x| x.name.clone()).collect::<Vec<_>>();
    let headers = ["time", "id", "amount", "msg", "kind", "to", "from"]
        .into_iter()
        .map(ToOwned::to_owned)
        .chain(accounts_str)
        .collect::<Vec<_>>();

    let mut wtr = csv::Writer::from_writer(File::create(path.as_ref())?);
    wtr.write_record(headers)?;

    for balance in balances {
        let time = Some(balance.transaction.timestamp.to_rfc3339());
        let id = Some(balance.transaction.id.clone());
        let amount = Some(balance.transaction.amount.to_string());
        let msg = balance.transaction.msg.clone();
        let kind = Some(
            match &balance.transaction.kind {
                transaction::Kind::External {
                    to: _,
                    from_name: _,
                } => "external",
                transaction::Kind::Internal { to: _, from: _ } => "internal",
            }
            .to_owned(),
        );

        let to = Some(balance.transaction.to().name.clone());
        let from = Some(balance.transaction.from_name().to_owned());

        let account_balances = &accounts
            .iter()
            .map(|k| balance.values.get(k).map(|x| x.amount.to_string()))
            .collect::<Vec<_>>();

        let row = [time, id, amount, msg, kind, to, from]
            .into_iter()
            .chain(account_balances.clone())
            .map(|x| match x {
                None => String::new(),
                Some(x) => x,
            })
            .collect::<Vec<_>>();
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    Ok(())
}

impl<'a> fmt::Display for Balance<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accounts = self
            .values
            .iter()
            .map(|(k, v)| format!(" • {}: {}", k.name, v.amount))
            .join("\n");

        let kind = match &self.transaction.kind {
            transaction::Kind::External {
                to: _,
                from_name: _,
            } => "expense",
            transaction::Kind::Internal { to: _, from: _ } => "transfer",
        };

        let time = self.transaction.timestamp.to_rfc3339();
        let amount = self.transaction.amount;
        let to = self.transaction.to_name();
        let from = self.transaction.from_name();

        let transaction = if let Some(x) = self.transaction.msg.as_deref() {
            formatdoc! {"
                • amount: {amount}
                • kind: {kind}
                • msg: {x}
                • {to} →  {from}"
            }
        } else {
            formatdoc! {"
                • amount: {amount}
                • kind: {kind}
                • {to} →  {from}"
            }
        };

        writedoc! {
            f,
            "
            Balance at {time}:
            Transaction:
            {transaction}
            Accounts:
            {accounts}"
        }
    }
}

#[cfg(test)]
mod test {
    use std::{fs, str::FromStr};

    use fallible_iterator::{FallibleIterator, IteratorExt};
    use money2::Currency;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    use super::*;
    use crate::model::{Account, UpTransaction};

    fn spending_account() -> Result<Account> {
        Ok(Account {
            name: "Spending".to_owned(),
            up_id: "2be1c9de-7a89-4e8f-8077-f535150b588d".to_owned(),
            ynab_id: Uuid::from_str("f6ca888b-327a-45d0-9775-830abdaa3a04")?,
            ynab_transfer_id: Uuid::from_str("89ddd9ef-2510-4b42-a889-e7a68cae291c")?,
        })
    }

    fn home_account() -> Result<Account> {
        Ok(Account {
            name: "Home".to_owned(),
            up_id: "328160b1-d7bc-41ee-9d7b-c7da4f2484b0".to_owned(),
            ynab_id: Uuid::from_str("2b00a77e-9b3c-4277-9c6c-6944f7696705")?,
            ynab_transfer_id: Uuid::from_str("f9b0b92f-70f7-4015-b885-4e5807a78a44")?,
        })
    }

    fn accounts() -> Result<Vec<Account>> {
        Ok(Vec::from([home_account()?, spending_account()?]))
    }

    fn transactions_from_file<P: AsRef<Path>>(
        path: P,
        accounts: &[Account],
    ) -> Result<Vec<Transaction>> {
        let payload = fs::read_to_string(path)?;
        let up_transactions = serde_json::from_str::<Vec<UpTransaction>>(&payload)?;
        let transactions = up_transactions
            .into_iter()
            .map(|x| x.to_transaction(accounts))
            .transpose_into_fallible()
            .filter(|x| Ok(x.is_normalized()))
            .collect::<Vec<_>>()?;
        Ok(transactions)
    }

    #[test]
    fn up_round_up_balance() -> Result<()> {
        let accounts = accounts()?;
        let transactions = transactions_from_file("test/data/up_round_up_balance.json", &accounts)?;
        let actual = running_total(&transactions);
        let expected = Vec::from([
            Balance {
                values: BTreeMap::from([(
                    spending_account()?,
                    Money::new(50_00, 2, Currency::from_str("AUD")?),
                )]),
                transaction: &transactions[0],
            },
            Balance {
                values: BTreeMap::from([(
                    spending_account()?,
                    Money::new(20_00, 2, Currency::from_str("AUD")?),
                )]),
                transaction: &transactions[1],
            },
            Balance {
                values: BTreeMap::from([
                    (
                        spending_account()?,
                        Money::new(19_00, 2, Currency::from_str("AUD")?),
                    ),
                    (
                        home_account()?,
                        Money::new(1_00, 2, Currency::from_str("AUD")?),
                    ),
                ]),
                transaction: &transactions[2],
            },
        ]);

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn up_transfer_balance() -> Result<()> {
        let accounts = accounts()?;
        let transactions = transactions_from_file("test/data/up_transfer_balance.json", &accounts)?;

        let actual = running_total(&transactions);
        let expected = Vec::from([
            Balance {
                values: BTreeMap::from([(
                    spending_account()?,
                    Money::new(90000, 2, Currency::from_str("AUD")?),
                )]),
                transaction: &transactions[0],
            },
            Balance {
                values: BTreeMap::from([
                    (
                        spending_account()?,
                        Money::new(40000, 2, Currency::from_str("AUD")?),
                    ),
                    (
                        home_account()?,
                        Money::new(50000, 2, Currency::from_str("AUD")?),
                    ),
                ]),
                transaction: &transactions[1],
            },
        ]);

        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn up_balance_sussy_round_up() -> Result<()> {
        let accounts = accounts()?;
        let transactions = transactions_from_file("test/data/up_sussy_round_up.json", &accounts)?;

        let actual = running_total(&transactions);
        let expected = Vec::from([
            Balance {
                values: BTreeMap::from([(
                    spending_account()?,
                    Money::new(-33_50, 2, Currency::from_str("AUD")?),
                )]),
                transaction: &transactions[0],
            },
            Balance {
                values: BTreeMap::from([
                    (
                        spending_account()?,
                        Money::new(-34_00, 2, Currency::from_str("AUD")?),
                    ),
                    (
                        home_account()?,
                        Money::new(50, 2, Currency::from_str("AUD")?),
                    ),
                ]),
                transaction: &transactions[1],
            },
            Balance {
                values: BTreeMap::from([
                    (
                        spending_account()?,
                        Money::new(16600, 2, Currency::from_str("AUD")?),
                    ),
                    (
                        home_account()?,
                        Money::new(-19950, 2, Currency::from_str("AUD")?),
                    ),
                ]),
                transaction: &transactions[2],
            },
            Balance {
                values: BTreeMap::from([
                    (
                        spending_account()?,
                        Money::new(16500, 2, Currency::from_str("AUD")?),
                    ),
                    (
                        home_account()?,
                        Money::new(-19850, 2, Currency::from_str("AUD")?),
                    ),
                ]),
                transaction: &transactions[3],
            },
        ]);

        assert_eq!(expected, actual);
        Ok(())
    }
}
