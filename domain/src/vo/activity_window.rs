use super::money::Money;
use crate::ar::{account::AccountId, activity::Activity};
use chrono::NaiveDateTime;

/**
 * A window of account activities.
 */
#[derive(Debug)]
pub struct ActivityWindow {
    pub activities: Vec<Activity>,
}

// Associated Functions
impl ActivityWindow {
    /// # Arguments
    ///
    /// * `activities` - The list of account activities within this window.
    pub fn new(activities: Vec<Activity>) -> Self {
        Self { activities }
    }
}

// Methods
impl ActivityWindow {
    /**
     * The timestamp of the first activity within this window.
     */
    pub fn get_start_timestamp(&self) -> NaiveDateTime {
        self.activities.iter().map(|a| a.timestamp).min().unwrap()
    }

    /**
     * The timestamp of the last activity within this window.
     * @return
     */
    pub fn get_end_timestamp(&self) -> NaiveDateTime {
        self.activities.iter().map(|a| a.timestamp).max().unwrap()
    }

    /**
     * Calculates the balance by summing up the values of all activities within this window.
     */
    pub fn calculate_balance(&self, account_id: &AccountId) -> Money {
        let deposit_balance = self
            .activities
            .iter()
            .filter(|a| &a.target_account_id == account_id)
            .map(|a| &a.money)
            .fold(Money::of(0), |acc, money| Money::add(&acc, money));
        let withdrawal_balance = self
            .activities
            .iter()
            .filter(|a| &a.source_account_id == account_id)
            .map(|a| &a.money)
            .fold(Money::of(0), |acc, money| Money::add(&acc, money));
        Money::add(&deposit_balance, &withdrawal_balance.negate())
    }

    pub fn add_activity(&mut self, activity: Activity) {
        self.activities.push(activity);
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
