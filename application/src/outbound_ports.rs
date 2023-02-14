use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain::ar::account::AccountId;
use mockall_double::double;

#[cfg(test)]
use mockall::automock;

#[double]
use domain::ar::account::Account;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait LoadAccountPort: Send + Sync + std::fmt::Debug {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: NaiveDateTime,
    ) -> Account;
}

#[cfg_attr(test, automock)]
pub trait AccountLock: std::fmt::Debug {
    fn lock_account(&self, account_id: AccountId);
    fn release_account(&self, account_id: AccountId);
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UpdateAccountStatePort: Send + Sync + std::fmt::Debug {
    async fn update_activities(&self, account: Account);
}
