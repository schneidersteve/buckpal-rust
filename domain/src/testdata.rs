use chrono::{Local, NaiveDateTime};

use crate::{vo::{money::Money, activity_window::ActivityWindow}, ar::{account::{AccountId, Account, AccountImpl}, activity::{ActivityId, Activity}}};

pub fn default_account() -> AccountBuilder {
    AccountBuilder::new()
        .with_account_id(AccountId(42))
        .with_baseline_balance(Money::of(999))
        .with_activity_window(ActivityWindow::new(vec![]))
}

pub struct AccountBuilder {
    account_id: Option<AccountId>,
    baseline_balance: Option<Money>,
    activity_window: Option<ActivityWindow>,
}

impl AccountBuilder {
    pub fn new() -> Self {
        Self {
            account_id: None,
            baseline_balance: None,
            activity_window: None,
        }
    }

    pub fn with_account_id(mut self, account_id: AccountId) -> Self {
        self.account_id = Some(account_id);
        self
    }

    pub fn with_baseline_balance(mut self, baseline_balance: Money) -> Self {
        self.baseline_balance = Some(baseline_balance);
        self
    }

    pub fn with_activity_window(mut self, activity_window: ActivityWindow) -> Self {
        self.activity_window = Some(activity_window);
        self
    }

    pub fn build(self) -> Box<impl Account> {
        Box::new(AccountImpl::with_id(
            self.account_id.unwrap(),
            self.baseline_balance.unwrap(),
            self.activity_window.unwrap(),
        ))
    }
}

pub fn default_activity() -> ActivityBuilder {
    ActivityBuilder::new()
        .with_owner_account(AccountId(42))
        .with_source_account(AccountId(42))
        .with_target_account(AccountId(41))
        .with_timestamp(Local::now().naive_local())
        .with_money(Money::of(999))
}

pub struct ActivityBuilder {
    id: Option<ActivityId>,
    owner_account_id: Option<AccountId>,
    source_account_id: Option<AccountId>,
    target_account_id: Option<AccountId>,
    timestamp: Option<NaiveDateTime>,
    money: Option<Money>,
}

impl ActivityBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            owner_account_id: None,
            source_account_id: None,
            target_account_id: None,
            timestamp: None,
            money: None,
        }
    }

    pub fn with_id(mut self, account_id: Option<ActivityId>) -> Self {
        self.id = account_id;
        self
    }

    pub fn with_owner_account(mut self, account_id: AccountId) -> Self {
        self.owner_account_id = Some(account_id);
        self
    }

    pub fn with_source_account(mut self, account_id: AccountId) -> Self {
        self.source_account_id = Some(account_id);
        self
    }

    pub fn with_target_account(mut self, account_id: AccountId) -> Self {
        self.target_account_id = Some(account_id);
        self
    }

    pub fn with_timestamp(mut self, timestamp: NaiveDateTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    pub fn with_money(mut self, money: Money) -> Self {
        self.money = Some(money);
        self
    }

    pub fn build(self) -> Activity {
        Activity::with_id(
            self.id,
            self.owner_account_id.unwrap(),
            self.source_account_id.unwrap(),
            self.target_account_id.unwrap(),
            self.timestamp.unwrap(),
            self.money.unwrap(),
        )
    }
}
