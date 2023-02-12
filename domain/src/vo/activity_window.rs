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
