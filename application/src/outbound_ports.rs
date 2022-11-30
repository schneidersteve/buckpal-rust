use chrono::NaiveDateTime;
use domain::account::{Account, AccountId};

pub trait LoadAccountPort : std::fmt::Debug {
    fn load_account(&self, account_id: AccountId, baseline_date: NaiveDateTime)
        -> Box<dyn Account>;
}

pub trait AccountLock : std::fmt::Debug {
    fn lock_account(&self, account_id: AccountId);
    fn release_account(&self, account_id: AccountId);
}

pub trait UpdateAccountStatePort : std::fmt::Debug {
    fn update_activities(&self, account: Box<dyn Account>);
}
