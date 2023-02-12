use super::activity::Activity;
use crate::vo::{activity_window::ActivityWindow, money::Money};
use chrono::Local;

#[derive(Clone, PartialEq, Hash, Debug)]
pub struct AccountId(pub i64);

/**
 * An account that holds a certain amount of money. An [Account] object only
 * contains a window of the latest account activities. The total balance of the account is
 * the sum of a baseline balance that was valid before the first activity in the
 * window and the sum of the activity values.
 */
pub trait Account: Send + Sync {
    fn get_id(&self) -> Option<AccountId>;
    fn calculate_balance(&self) -> Money;
    fn withdraw(&mut self, money: Money, target_account_id: AccountId) -> bool;
    fn deposit(&mut self, money: Money, source_account_id: AccountId) -> bool;
    fn get_activity_window(&self) -> &ActivityWindow;
}

#[derive(Debug)]
pub struct AccountImpl {
    id: Option<AccountId>,
    baseline_balance: Money,
    pub activity_window: ActivityWindow,
}

// Associated Functions
impl AccountImpl {
    /// # Arguments
    ///
    /// * `id` - The unique ID of the account.
    /// * `baseline_balance` - The baseline balance of the account. This was the balance of the account before the first activity in the activityWindow.
    /// * `activity_window` - The window of latest activities on this account.
    fn new(
        id: Option<AccountId>,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> impl Account {
        Self {
            id,
            baseline_balance,
            activity_window,
        }
    }

    pub fn without_id(baseline_balance: Money, activity_window: ActivityWindow) -> impl Account {
        AccountImpl::new(None, baseline_balance, activity_window)
    }

    pub fn with_id(
        account_id: AccountId,
        baseline_balance: Money,
        activity_window: ActivityWindow,
    ) -> impl Account {
        AccountImpl::new(Some(account_id), baseline_balance, activity_window)
    }
}

// Methods
impl AccountImpl {
    #[allow(unused)]
    fn may_withdraw(&self, money: &Money) -> bool {
        Money::add(&self.calculate_balance(), &money.negate()).is_positive_or_zero()
    }
}

impl Account for AccountImpl {
    fn get_id(&self) -> Option<AccountId> {
        self.id.clone()
    }

    /**
     * Calculates the total balance of the account by adding the activity values to the baseline balance.
     */
    fn calculate_balance(&self) -> Money {
        Money::add(
            &self.baseline_balance,
            &self
                .activity_window
                .calculate_balance(&self.id.clone().unwrap()),
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
            self.id.clone().unwrap(),
            self.id.clone().unwrap(),
            target_account_id,
            Local::now().naive_local(),
            money,
        );
        self.activity_window.add_activity(withdrawal);
        true
    }

    /**
     * Tries to deposit a certain amount of money to this account.
     * If sucessful, creates a new activity with a positive value.
     * @return true if the deposit was successful, false if not.
     */
    fn deposit(&mut self, money: Money, source_account_id: AccountId) -> bool {
        let deposit = Activity::new(
            self.id.clone().unwrap(),
            source_account_id,
            self.id.clone().unwrap(),
            Local::now().naive_local(),
            money,
        );
        self.activity_window.add_activity(deposit);
        true
    }

    fn get_activity_window(&self) -> &ActivityWindow {
        &self.activity_window
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testdata::{default_account, default_activity};
    use std::any::Any;

    #[test]
    fn test_calculates_balance() {
        let account_id = AccountId(1);
        let account = default_account()
            .with_account_id(account_id.clone())
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id.clone())
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
        let account = default_account()
            .with_account_id(account_id.clone())
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id.clone())
                    .with_money(Money::of(999))
                    .build(),
                default_activity()
                    .with_target_account(account_id)
                    .with_money(Money::of(1))
                    .build(),
            ]))
            .build();
        let mut any = account as Box<dyn Any>;
        let account_impl = any.downcast_mut::<AccountImpl>().unwrap();
        let success = account_impl.withdraw(Money::of(555), AccountId(99));
        assert!(success);
        assert_eq!(3, account_impl.activity_window.activities.len());
        assert_eq!(Money::of(1000), account_impl.calculate_balance());
    }

    #[test]
    fn test_withdrawal_failure() {
        let account_id = AccountId(1);
        let mut account = default_account()
            .with_account_id(account_id.clone())
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id.clone())
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
        // assert_eq!(2, account.activity_window.activities.len());
        assert_eq!(Money::of(1555), account.calculate_balance());
    }

    #[test]
    fn test_deposit_success() {
        let account_id = AccountId(1);
        let mut account = default_account()
            .with_account_id(account_id.clone())
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![
                default_activity()
                    .with_target_account(account_id.clone())
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
        // assert_eq!(3, account.activity_window.activities.len());
        assert_eq!(Money::of(2000), account.calculate_balance());
    }
}
