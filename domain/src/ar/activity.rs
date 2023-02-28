// use crate::{account::AccountId, money::Money};
use super::account::AccountId;
use crate::vo::money::Money;
use chrono::NaiveDateTime;

#[derive(PartialEq, Hash, Debug)]
pub struct ActivityId(pub i64);

/**
 * A money transfer activity between [Account]s
 */
#[derive(PartialEq, Hash, Debug)]
pub struct Activity {
    pub id: Option<ActivityId>,
    pub owner_account_id: AccountId,
    pub source_account_id: AccountId,
    pub target_account_id: AccountId,
    pub timestamp: NaiveDateTime,
    pub money: Money,
}

impl Activity {
    /// # Arguments
    ///
    /// * `owner_account_id` - The account that owns this activity.
    /// * `source_account_id` - The debited account.
    /// * `target_account_id` - The credited account.
    /// * `timestamp` - The timestamp of the activity.
    /// * `money` - The money that was transferred between the accounts.
    pub fn new(
        owner_account_id: AccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        timestamp: NaiveDateTime,
        money: Money,
    ) -> Self {
        Self::with_id(
            None,
            owner_account_id,
            source_account_id,
            target_account_id,
            timestamp,
            money,
        )
    }

    pub fn with_id(
        id: Option<ActivityId>,
        owner_account_id: AccountId,
        source_account_id: AccountId,
        target_account_id: AccountId,
        timestamp: NaiveDateTime,
        money: Money,
    ) -> Self {
        Self {
            id,
            owner_account_id,
            source_account_id,
            target_account_id,
            timestamp,
            money,
        }
    }
}
