use crate::{
    account_mapper, account_repository::AccountRepository, activity_repository::ActivityRepository,
};
use application::outbound_ports::{LoadAccountPort, UpdateAccountStatePort};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use domain::account::{Account, AccountId};
use log::debug;

// #[singleton]
#[derive(Debug)]
pub struct AccountPersistenceAdapter {
    account_repository: Box<dyn AccountRepository>,
    activity_repository: Box<dyn ActivityRepository>,
}

impl AccountPersistenceAdapter {
    // #[inject]
    pub fn new(
        account_repository: Box<dyn AccountRepository>,
        activity_repository: Box<dyn ActivityRepository>,
    ) -> Self {
        Self {
            account_repository,
            activity_repository,
        }
    }
}

#[async_trait]
impl LoadAccountPort for AccountPersistenceAdapter {
    async fn load_account(
        &self,
        account_id: AccountId,
        baseline_date: NaiveDateTime,
    ) -> Box<dyn Account> {
        let account = self
            .account_repository
            .find_by_id(account_id.0)
            .await
            .unwrap_or_else(|| panic!("EntityNotFoundException"));
        debug!("find_by_id(id = {:?}) = {:?}", account_id, account);

        let activities = self
            .activity_repository
            .find_by_owner_since(account_id.0, baseline_date)
            .await;
        debug!(
            "find_by_owner_since(owner_account_id = {:?}, timestamp = {}) = {:?}",
            account_id, baseline_date, activities
        );

        let withdrawal_balance = self
            .activity_repository
            .get_withdrawal_balance_until(account_id.0, baseline_date)
            .await
            .unwrap_or(0);
        debug!(
            "get_withdrawal_balance_until(account_id = {:?}, until = {}) = {:?}",
            account_id, baseline_date, withdrawal_balance
        );

        let deposit_balance = self
            .activity_repository
            .get_deposit_balance_until(account_id.0, baseline_date)
            .await
            .unwrap_or(0);
        debug!(
            "get_deposit_balance_until(account_id = {:?}, until = {}) = {:?}",
            account_id, baseline_date, deposit_balance
        );

        account_mapper::map_to_account(account, activities, withdrawal_balance, deposit_balance)
    }
}

#[async_trait]
impl UpdateAccountStatePort for AccountPersistenceAdapter {
    async fn update_activities(&self, account: Box<dyn Account>) {
        for activity in &account.get_activity_window().activities {
            if activity.id.is_none() {
                let ae = account_mapper::map_to_activity_entity(activity);
                debug!("save(activity_entity = {:?}", ae);
                self.activity_repository.save(ae).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{account_repository::AccountEntity, activity_repository::ActivityEntity};
    use chrono::{NaiveDate, NaiveTime};
    use domain::account::AccountImpl;
    use domain::{
        activity_window::ActivityWindow,
        money::Money,
        testdata::{default_account, default_activity},
    };
    use mockall::{mock, predicate::eq};

    mock! {
        #[derive(Debug)]
        AccountRepositoryImpl {}
        #[async_trait]
        impl AccountRepository for AccountRepositoryImpl {
            async fn find_by_id(&self, id: i64) -> Option<AccountEntity>;
        }
    }

    mock! {
        #[derive(Debug)]
        ActivityRepositoryImpl {}
        #[async_trait]
        impl ActivityRepository for ActivityRepositoryImpl {
            async fn find_by_owner_since(
                &self,
                owner_account_id: i64,
                timestamp: NaiveDateTime,
            ) -> Vec<ActivityEntity>;
            async fn get_deposit_balance_until(
                &self,
                account_id: i64,
                until: NaiveDateTime,
            ) -> Option<i128>;
            async fn get_withdrawal_balance_until(
                &self,
                account_id: i64,
                until: NaiveDateTime,
            ) -> Option<i128>;
            async fn save(&self, activity_entity: ActivityEntity);
        }
    }

    #[tokio::test]
    async fn test_loads_account() {
        // Given
        let account_id = AccountId(1);
        let baseline_date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2018, 8, 10).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let mut account_repository = Box::new(MockAccountRepositoryImpl::new());
        account_repository
            .expect_find_by_id()
            .with(eq(account_id.0))
            .returning(|id| Some(AccountEntity { id: Some(id) }));

        let mut activity_repository = Box::new(MockActivityRepositoryImpl::new());
        activity_repository
            .expect_find_by_owner_since()
            .with(eq(account_id.0), eq(baseline_date))
            .returning(|_owner_account_id, _timestamp| {
                vec![
                    ActivityEntity {
                        id: Some(5),
                        timestamp: NaiveDateTime::new(
                            NaiveDate::from_ymd_opt(2019, 8, 9).unwrap(),
                            NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                        ),
                        owner_account_id: 1,
                        source_account_id: 1,
                        target_account_id: 2,
                        amount: 1000,
                    },
                    ActivityEntity {
                        id: Some(7),
                        timestamp: NaiveDateTime::new(
                            NaiveDate::from_ymd_opt(2019, 8, 9).unwrap(),
                            NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
                        ),
                        owner_account_id: 1,
                        source_account_id: 2,
                        target_account_id: 1,
                        amount: 1000,
                    },
                ]
            });
        activity_repository
            .expect_get_withdrawal_balance_until()
            .with(eq(account_id.0), eq(baseline_date))
            .return_const(500);
        activity_repository
            .expect_get_deposit_balance_until()
            .with(eq(account_id.0), eq(baseline_date))
            .return_const(1000);

        // When
        let adapter_under_test =
            AccountPersistenceAdapter::new(account_repository, activity_repository);
        let account = adapter_under_test
            .load_account(account_id, baseline_date)
            .await;

        // Then
        let account_impl =
            unsafe { &*(&account as *const Box<dyn Account> as *const Box<AccountImpl>) };
        assert_eq!(2, account_impl.activity_window.activities.len());
        assert_eq!(Money::of(500), account.calculate_balance());
    }

    #[tokio::test]
    async fn test_updates_activities() {
        // Given
        let account = default_account()
            .with_baseline_balance(Money::of(555))
            .with_activity_window(ActivityWindow::new(vec![default_activity()
                .with_id(None)
                .with_money(Money::of(1))
                .build()]))
            .build();

        let account_repository = Box::new(MockAccountRepositoryImpl::new());
        let mut activity_repository = Box::new(MockActivityRepositoryImpl::new());
        activity_repository.expect_save().times(1).return_const(());

        // When
        let adapter_under_test =
            AccountPersistenceAdapter::new(account_repository, activity_repository);
        adapter_under_test.update_activities(account).await;
    }
}
