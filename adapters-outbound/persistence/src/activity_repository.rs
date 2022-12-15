use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{FromRow, Row, SqlitePool};

#[async_trait]
pub trait ActivityRepository: Send + Sync + std::fmt::Debug {
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

// #[singleton]
#[derive(Debug)]
pub struct ActivityRepositoryImpl {
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
    async fn find_by_owner_since(
        &self,
        owner_account_id: i64,
        timestamp: NaiveDateTime,
    ) -> Vec<ActivityEntity> {
        let rows = sqlx::query_as::<_, ActivityEntity>(
            "
            SELECT * FROM activity_entity
            WHERE owner_account_id = ?
            AND timestamp >= ?
            ",
        )
        .bind(owner_account_id)
        .bind(timestamp)
        .fetch_all(&self.db_pool)
        .await;
        if let Ok(rows) = rows {
            return rows;
        }
        vec![]
    }

    async fn get_deposit_balance_until(
        &self,
        account_id: i64,
        until: NaiveDateTime,
    ) -> Option<i128> {
        let row = sqlx::query(
            "
            SELECT SUM(amount) FROM activity_entity
            WHERE target_account_id = ?
            AND owner_account_id = ?
            AND timestamp < ?
            ",
        )
        .bind(account_id)
        .bind(account_id)
        .bind(until)
        .fetch_one(&self.db_pool)
        .await;
        if let Ok(row) = row {
            let amount: i64 = row.try_get("SUM(amount)").unwrap();
            return Some(amount as i128);
        }
        None
    }

    async fn get_withdrawal_balance_until(
        &self,
        account_id: i64,
        until: NaiveDateTime,
    ) -> Option<i128> {
        let row = sqlx::query(
            "
            SELECT SUM(amount) FROM activity_entity 
            WHERE source_account_id = ?
            AND owner_account_id = ? 
            AND timestamp < ?
            ",
        )
        .bind(account_id)
        .bind(account_id)
        .bind(until)
        .fetch_one(&self.db_pool)
        .await;
        if let Ok(row) = row {
            let amount: i64 = row.try_get("SUM(amount)").unwrap();
            return Some(amount as i128);
        }
        None
    }

    async fn save(&self, activity_entity: ActivityEntity) {
        sqlx::query(
            "
            INSERT INTO activity_entity (timestamp, owner_account_id, source_account_id, target_account_id, amount)
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(activity_entity.timestamp)
        .bind(activity_entity.owner_account_id)
        .bind(activity_entity.source_account_id)
        .bind(activity_entity.target_account_id)
        .bind(activity_entity.amount)
        .execute(&self.db_pool)
        .await
        .unwrap();
    }
}

#[derive(FromRow, PartialEq, Hash, Debug)]
pub struct ActivityEntity {
    pub id: Option<i64>,
    pub timestamp: NaiveDateTime,
    pub owner_account_id: i64,
    pub source_account_id: i64,
    pub target_account_id: i64,
    pub amount: i64,
}
