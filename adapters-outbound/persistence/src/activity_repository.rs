use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::SqlitePool;

#[async_trait]
pub trait ActivityRepository: Send + Sync + std::fmt::Debug {
    async fn find_by_owner_account_id_equals_and_timestamp_greater_than_equals(
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

#[derive(Debug)]
struct ActivityRepositoryImpl {
    db_pool: SqlitePool,
}

impl ActivityRepositoryImpl {
    // #[inject]
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl ActivityRepository for ActivityRepositoryImpl {
    async fn find_by_owner_account_id_equals_and_timestamp_greater_than_equals(
        &self,
        owner_account_id: i64,
        timestamp: NaiveDateTime,
    ) -> Vec<ActivityEntity> {
        todo!()
    }

    async fn get_deposit_balance_until(
        &self,
        account_id: i64,
        until: NaiveDateTime,
    ) -> Option<i128> {
        todo!()
    }

    async fn get_withdrawal_balance_until(
        &self,
        account_id: i64,
        until: NaiveDateTime,
    ) -> Option<i128> {
        todo!()
    }

    async fn save(&self, activity_entity: ActivityEntity) {
        todo!()
    }
}

#[derive(PartialEq, Hash, Debug)]
pub struct ActivityEntity {
    pub id: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub owner_account_id: i64,
    pub source_account_id: i64,
    pub target_account_id: i64,
    pub amount: i128,
}
