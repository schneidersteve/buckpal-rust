use std::ops::Sub;

use chrono::{Days, Local};
use domain::money::Money;

use crate::{
    inbound_ports::{SendMoneyCommand, SendMoneyUseCase},
    outbound_ports::{AccountLock, LoadAccountPort, UpdateAccountStatePort},
};

pub struct SendMoneyUseCaseImpl {
    load_account_port: Box<dyn LoadAccountPort>,
    account_lock: Box<dyn AccountLock>,
    update_account_state_port: Box<dyn UpdateAccountStatePort>,
    money_transfer_properties: MoneyTransferProperties,
}

impl SendMoneyUseCaseImpl {
    pub fn new(
        load_account_port: Box<dyn LoadAccountPort>,
        account_lock: Box<dyn AccountLock>,
        update_account_state_port: Box<dyn UpdateAccountStatePort>,
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

impl SendMoneyUseCase for SendMoneyUseCaseImpl {
    fn send_money(&self, command: SendMoneyCommand) -> bool {
        self.check_threshold(&command);

        let baseline_date = Local::now().naive_local().sub(Days::new(10));

        let mut source_account = self
            .load_account_port
            .load_account(command.source_account_id, baseline_date);

        let mut target_account = self
            .load_account_port
            .load_account(command.target_account_id, baseline_date);

        let source_account_id = source_account.get_id().unwrap();
        // .unwrap_or(panic!("expected source account ID not to be empty"));
        let target_account_id = source_account.get_id().unwrap();
        // .unwrap_or(panic!("expected target account ID not to be empty"));

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
            .update_activities(source_account);
        self.update_account_state_port
            .update_activities(target_account);

        self.account_lock.release_account(source_account_id);
        self.account_lock.release_account(target_account_id);
        true
    }
}

#[derive(PartialEq, Hash, Debug)]
pub struct MoneyTransferProperties {
    maximum_transfer_threshold: Money,
}

impl MoneyTransferProperties {
    // Functions

    pub fn new(mtt: Option<Money>) -> Self {
        Self {
            maximum_transfer_threshold: mtt.unwrap_or(Money::of(1_000_000)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;
    use domain::account::{Account, AccountId};
    use mockall::{
        mock,
        predicate::{always, eq},
    };

    mock! {
        #[derive(Debug)]
        AccountImpl {}
        impl Account for AccountImpl {
            fn get_id(&self) -> Option<AccountId>;
            fn calculate_balance(&self) -> Money;
            fn withdraw(&mut self, money: Money, target_account_id: AccountId) -> bool;
            fn deposit(&mut self, money: Money, source_account_id: AccountId) -> bool;
        }
    }

    mock! {
        LoadAccountPortImpl {}
        impl LoadAccountPort for LoadAccountPortImpl {
            fn load_account(&self, account_id: AccountId, baseline_date: NaiveDateTime) -> Box<dyn Account>;
        }
    }

    mock! {
        NoOpAccountLockImpl {}
        impl AccountLock for NoOpAccountLockImpl {
            fn lock_account(&self, account_id: AccountId);
            fn release_account(&self, account_id: AccountId);
        }
    }

    mock! {
        UpdateAccountStatePortImpl {}
        impl UpdateAccountStatePort for UpdateAccountStatePortImpl {
            fn update_activities(&self, account: Box<dyn Account>);
        }
    }

    // TODO Add with() parameter expectations
    #[test]
    fn test_transaction_succeeds() {
        let mut load_account_port = Box::new(MockLoadAccountPortImpl::new());
        // Given a source account
        let source_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Box<dyn Account> {
                let mut account = Box::new(MockAccountImpl::new());
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
            .times(1)
            .returning(move |account_id, baseline_date| {
                source_account_closure(account_id, baseline_date)
            });
        // And a target account
        let target_account_closure =
            |account_id: AccountId, _baseline_date: NaiveDateTime| -> Box<dyn Account> {
                let mut account = Box::new(MockAccountImpl::new());
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
            .times(1)
            .returning(move |account_id, baseline_date| {
                target_account_closure(account_id, baseline_date)
            });

        let mut account_lock = Box::new(MockNoOpAccountLockImpl::new());
        // And source account is locked
        // And target account is locked
        account_lock.expect_lock_account().times(2).return_const(());

        // And target account is released
        // And source account is released
        account_lock
            .expect_release_account()
            .times(2)
            .return_const(());

        let mut update_account_state_port = Box::new(MockUpdateAccountStatePortImpl::new());
        // And accounts have been updated
        update_account_state_port
            .expect_update_activities()
            .times(2)
            .return_const(());

        // When money is send
        let command = SendMoneyCommand::new(AccountId(41), AccountId(42), Money::of(500));
        let send_money_use_case = SendMoneyUseCaseImpl::new(
            load_account_port,
            account_lock,
            update_account_state_port,
            MoneyTransferProperties::new(Some(Money::of(i128::MAX))),
        );
        let success = send_money_use_case.send_money(command);

        // Then send money succeeds
        assert!(success);
    }

    #[test]
    fn test_given_withdrawal_fails_then_only_source_account_is_locked_and_released() {}
}
