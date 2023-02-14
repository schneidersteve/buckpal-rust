use crate::{
    inbound_ports::{SendMoneyCommand, SendMoneyUseCase},
    outbound_ports::{AccountLock, LoadAccountPort, UpdateAccountStatePort},
};

use async_trait::async_trait;
use chrono::{Days, Local};
use domain::vo::money::Money;
use std::{ops::Sub, sync::Arc};

// #[singleton]
#[derive(Debug)]
pub struct SendMoneyUseCaseImpl {
    load_account_port: Arc<dyn LoadAccountPort>,
    account_lock: Box<dyn AccountLock>,
    update_account_state_port: Arc<dyn UpdateAccountStatePort>,
    money_transfer_properties: MoneyTransferProperties,
}

impl SendMoneyUseCaseImpl {
    // #[inject]
    pub fn new(
        load_account_port: Arc<dyn LoadAccountPort>,
        account_lock: Box<dyn AccountLock>,
        update_account_state_port: Arc<dyn UpdateAccountStatePort>,
        money_transfer_properties: MoneyTransferProperties,
    ) -> Self {
        Self {
            load_account_port,
            account_lock,
            update_account_state_port,
            money_transfer_properties,
        }
    }

    fn check_threshold(&self, command: &SendMoneyCommand) {
        if command
            .money
            .is_greater_than(&self.money_transfer_properties.maximum_transfer_threshold)
        {
            panic!("Threshold Exceeded Exception");
        }
    }
}

unsafe impl Send for SendMoneyUseCaseImpl {}
unsafe impl Sync for SendMoneyUseCaseImpl {}

#[async_trait]
impl SendMoneyUseCase for SendMoneyUseCaseImpl {
    async fn send_money(&self, command: SendMoneyCommand) -> bool {
        self.check_threshold(&command);

        let baseline_date = Local::now().naive_local().sub(Days::new(10));

        let mut source_account = self
            .load_account_port
            .load_account(command.source_account_id, baseline_date)
            .await;

        let mut target_account = self
            .load_account_port
            .load_account(command.target_account_id, baseline_date)
            .await;

        let source_account_id = source_account
            .get_id()
            .unwrap_or_else(|| panic!("expected source account ID not to be empty"));
        let target_account_id = target_account
            .get_id()
            .unwrap_or_else(|| panic!("expected target account ID not to be empty"));

        self.account_lock.lock_account(source_account_id.clone());
        if !source_account.withdraw(command.money.clone(), target_account_id.clone()) {
            self.account_lock.release_account(source_account_id);
            return false;
        }

        self.account_lock.lock_account(target_account_id.clone());
        if !target_account.deposit(command.money, source_account_id.clone()) {
            self.account_lock.release_account(source_account_id);
            self.account_lock.release_account(target_account_id);
            return false;
        }

        self.update_account_state_port
            .update_activities(source_account)
            .await;
        self.update_account_state_port
            .update_activities(target_account)
            .await;

        self.account_lock.release_account(source_account_id);
        self.account_lock.release_account(target_account_id);
        true
    }
}

// #[singleton]
#[derive(PartialEq, Hash, Debug)]
pub struct MoneyTransferProperties {
    maximum_transfer_threshold: Money,
}

impl MoneyTransferProperties {
    // Functions

    pub fn new(maximum_transfer_threshold: Option<Money>) -> Self {
        Self {
            maximum_transfer_threshold: maximum_transfer_threshold.unwrap_or(Money::of(1_000_000)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::outbound_ports::{MockAccountLock, MockLoadAccountPort, MockUpdateAccountStatePort};

    use super::*;
    use chrono::NaiveDateTime;
    use domain::ar::account::AccountId;
    use mockall::{
        predicate::{always, eq},
    };
    use mockall_double::double;

    #[double]
    use domain::ar::account::Account;

    // TODO Add with() parameter expectations
    #[async_std::test]
    async fn test_transaction_succeeds() {
        let mut load_account_port = MockLoadAccountPort::new();
        // Given a source account
        let source_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Account {
                let mut account = Account::new();
                account
                    .expect_get_id()
                    .returning(move || Some(account_id.clone()));

                // And source account withdrawal will succeed
                account.expect_withdraw().times(1).return_const(true);

                account
            };
        load_account_port
            .expect_load_account()
            .with(eq(AccountId(41)), always())
            .returning(move |account_id, baseline_date| {
                source_account_closure(account_id, baseline_date)
            });
        // And a target account
        let target_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Account {
                let mut account = Account::new();
                account
                    .expect_get_id()
                    .returning(move || Some(account_id.clone()));

                // And target account deposit will succeed
                account.expect_deposit().times(1).return_const(true);

                account
            };
        load_account_port
            .expect_load_account()
            .with(eq(AccountId(42)), always())
            .returning(move |account_id, baseline_date| {
                target_account_closure(account_id, baseline_date)
            });

        let mut account_lock = Box::new(MockAccountLock::new());
        // And source account is locked
        // And target account is locked
        account_lock.expect_lock_account().times(2).return_const(());

        // And source account is released
        // And target account is released
        account_lock
            .expect_release_account()
            .times(2)
            .return_const(());

        let mut update_account_state_port = MockUpdateAccountStatePort::new();
        // And accounts have been updated
        update_account_state_port
            .expect_update_activities()
            .times(2)
            .return_const(());

        // When money is send
        let command = SendMoneyCommand::new(AccountId(41), AccountId(42), Money::of(500));
        let send_money_use_case = SendMoneyUseCaseImpl::new(
            Arc::new(load_account_port),
            account_lock,
            Arc::new(update_account_state_port),
            MoneyTransferProperties::new(Some(Money::of(i128::MAX))),
        );
        let success = send_money_use_case.send_money(command).await;

        // Then send money succeeds
        assert!(success);
    }

    #[async_std::test]
    async fn test_given_withdrawal_fails_then_only_source_account_is_locked_and_released() {
        let mut load_account_port = MockLoadAccountPort::new();
        // Given a source account
        let source_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Account {
                let mut account = Account::new();
                account
                    .expect_get_id()
                    .returning(move || Some(account_id.clone()));

                // And source account withdrawal will fail
                account.expect_withdraw().return_const(false);

                account
            };
        load_account_port
            .expect_load_account()
            .with(eq(AccountId(41)), always())
            .returning(move |account_id, baseline_date| {
                source_account_closure(account_id, baseline_date)
            });
        // And a target account
        let target_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Account {
                let mut account = Account::new();
                account
                    .expect_get_id()
                    .returning(move || Some(account_id.clone()));

                // And target account deposit will succeed
                account.expect_deposit().return_const(true);

                account
            };
        load_account_port
            .expect_load_account()
            .with(eq(AccountId(42)), always())
            .returning(move |account_id, baseline_date| {
                target_account_closure(account_id, baseline_date)
            });

        let mut account_lock = Box::new(MockAccountLock::new());
        // And source account is locked
        // And target account is not locked
        account_lock.expect_lock_account().times(1).return_const(());

        // And source account is released
        account_lock
            .expect_release_account()
            .times(1)
            .return_const(());

        // When money is send
        let command = SendMoneyCommand::new(AccountId(41), AccountId(42), Money::of(300));
        let send_money_use_case = SendMoneyUseCaseImpl::new(
            Arc::new(load_account_port),
            account_lock,
            Arc::new(MockUpdateAccountStatePort::new()),
            MoneyTransferProperties::new(Some(Money::of(i128::MAX))),
        );
        let success = send_money_use_case.send_money(command).await;

        // Then send money succeeds
        assert!(!success);
    }
}
