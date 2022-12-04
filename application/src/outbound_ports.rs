use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain::account::{Account, AccountId};

#[async_trait]
pub trait LoadAccountPort: Send + Sync + std::fmt::Debug {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: NaiveDateTime,
    ) -> Box<dyn Account>;
}

pub trait AccountLock: std::fmt::Debug {
    fn lock_account(&self, account_id: AccountId);
    fn release_account(&self, account_id: AccountId);
}

#[async_trait]
pub trait UpdateAccountStatePort: Send + Sync + std::fmt::Debug {
    async fn update_activities(&self, account: Box<dyn Account>);
}
