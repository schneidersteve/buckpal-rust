use chrono::Local;

use crate::{
    activity_window::{Activity, ActivityWindow},
    money::Money,
};

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct AccountId(pub i64);

pub struct Account {
    id: Option<AccountId>,
    baseline_balance: Money,
    activity_window: ActivityWindow,
}

/**
 * An account that holds a certain amount of money. An [Account] object only
 * contains a window of the latest account activities. The total balance of the account is
 * the sum of a baseline balance that was valid before the first activity in the
 * window and the sum of the activity values.
 */
impl Account {
    // Functions

    /// # Arguments
    ///
    /// * `id` - The unique ID of the account.
    /// * `baseline_balance` - The baseline balance of the account. This was the balance of the account before the first activity in the activityWindow.
    /// * `activity_window` - The window of latest activities on this account.
    fn new(
        id: Option<AccountId>,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Self {
        Self {
            id,
            baseline_balance,
            activity_window,
        }
    }

    pub fn without_id(baseline_balance: Money, activity_window: ActivityWindow) -> Account {
        Account::new(None, baseline_balance, activity_window)
    }

    pub fn with_id(
        account_id: AccountId,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> Account {
        Account::new(Some(account_id), baseline_balance, activity_window)
    }

    // Methods

    fn get_id(&self) -> Option<AccountId> {
        // Return a copy
        self.id
    }

    fn calculate_balance(&self) -> Money {
        Money::add(
            &self.baseline_balance,
            &self.activity_window.calculate_balance(&self.id.unwrap()),
        )
    }

    /**
     * Tries to withdraw a certain amount of money from this account.
     * If successful, creates a new activity with a negative value.
     * @return true if the withdrawal was successful, false if not.
     */
    fn withdraw(&mut self, money: Money, target_account_id: AccountId) -> bool {
        if !self.may_withdraw(&money) {
            return false;
        }
        let withdrawal = Activity::new(
            self.id.unwrap(),
            self.id.unwrap(),
            target_account_id,
            Local::now().naive_local(),
            money,
        );
        self.activity_window.add_activity(withdrawal);
        true
    }

    fn deposit(&mut self, money: Money, source_account_id: AccountId) -> bool {
        let deposit = Activity::new(
            self.id.unwrap(),
            source_account_id,
            self.id.unwrap(),
            Local::now().naive_local(),
            money,
        );
        self.activity_window.add_activity(deposit);
        true
    }

    #[allow(unused)]
    fn may_withdraw(&self, money: &Money) -> bool {
        Money::add(&self.calculate_balance(), &money.negate()).is_positive_or_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::{default_account, default_activity};

    #[test]
    fn test_calculates_balance() {
        let account_id = AccountId(1);
        let account = default_account()
            .with_account_id(account_id)
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(999))
                    .build(),
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(1))
                    .build(),
            ]))
            .build();
        let balance = account.calculate_balance();
        assert_eq!(Money::of(1555), balance);
    }

    #[test]
    fn test_withdrawal_succeeds() {
        let account_id = AccountId(1);
        let mut account = default_account()
            .with_account_id(account_id)
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(999))
                    .build(),
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(1))
                    .build(),
            ]))
            .build();
        let success = account.withdraw(Money::of(555), AccountId(99));
        assert!(success);
        assert_eq!(3, account.activity_window.activities.len());
        assert_eq!(Money::of(1000), account.calculate_balance());
    }

    #[test]
    fn test_withdrawal_failure() {
        let account_id = AccountId(1);
        let mut account = default_account()
            .with_account_id(account_id)
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(999))
                    .build(),
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(1))
                    .build(),
            ]))
            .build();
        let success = account.withdraw(Money::of(1556), AccountId(99));
        assert!(!success);
        assert_eq!(2, account.activity_window.activities.len());
        assert_eq!(Money::of(1555), account.calculate_balance());
    }

    #[test]
    fn test_deposit_success() {
        let account_id = AccountId(1);
        let mut account = default_account()
            .with_account_id(account_id)
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(999))
                    .build(),
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(1))
                    .build(),
            ]))
            .build();
        let success = account.deposit(Money::of(445), AccountId(99));
        assert!(success);
        assert_eq!(3, account.activity_window.activities.len());
        assert_eq!(Money::of(2000), account.calculate_balance());
    }
}
