use chrono::NaiveDateTime;
use domain::account::{Account, AccountId};

pub trait LoadAccountPort {
    fn load_account(&self, account_id: AccountId, baseline_date: NaiveDateTime)
        -> Box<dyn Account>;
}

pub trait AccountLock {
    fn lock_account(&self, account_id: AccountId);
    fn release_account(&self, account_id: AccountId);
}

pub trait UpdateAccountStatePort {
    fn update_activities(&self, account: Box<dyn Account>);
}
