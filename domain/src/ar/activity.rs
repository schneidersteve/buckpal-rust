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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{testdata::default_activity, vo::activity_window::ActivityWindow};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_calculates_start_timestamp() {
        let window = ActivityWindow::new(vec![
            default_activity().with_timestamp(start_date()).build(),
            default_activity().with_timestamp(in_between_date()).build(),
            default_activity().with_timestamp(end_date()).build(),
        ]);
        assert_eq!(start_date(), window.get_start_timestamp());
    }

    #[test]
    fn test_calculates_end_timestamp() {
        let window = ActivityWindow::new(vec![
            default_activity().with_timestamp(start_date()).build(),
            default_activity().with_timestamp(in_between_date()).build(),
            default_activity().with_timestamp(end_date()).build(),
        ]);
        assert_eq!(end_date(), window.get_end_timestamp());
    }

    #[test]
    fn test_calculates_balance() {
        let account1 = AccountId(1);
        let account2 = AccountId(2);
        let window = ActivityWindow::new(vec![
            default_activity()
                .with_source_account(account1.clone())
                .with_target_account(account2.clone())
                .with_money(Money::of(999))
                .build(),
            default_activity()
                .with_source_account(account1.clone())
                .with_target_account(account2.clone())
                .with_money(Money::of(1))
                .build(),
            default_activity()
                .with_source_account(account2.clone())
                .with_target_account(account1.clone())
                .with_money(Money::of(500))
                .build(),
        ]);
        assert_eq!(Money::of(-500), window.calculate_balance(&account1));
        assert_eq!(Money::of(500), window.calculate_balance(&account2));
    }

    fn start_date() -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2019, 8, 3).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    }

    fn in_between_date() -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2019, 8, 4).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    }

    fn end_date() -> NaiveDateTime {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2019, 8, 5).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        )
    }
}
