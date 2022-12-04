use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[async_trait]
pub trait AccountRepository: Send + Sync + std::fmt::Debug {
    async fn find_by_id(&self, id: i64) -> Option<AccountEntity>;
}

#[derive(Debug)]
struct AccountRepositoryImpl {
    db_pool: SqlitePool,
}

impl AccountRepositoryImpl {
    // #[inject]
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {
    async fn find_by_id(&self, id: i64) -> Option<AccountEntity> {
        let row = sqlx::query("SELECT id FROM account_entity WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await;
        if let Ok(row) = row {
            return Some(AccountEntity {
                id: row.try_get("id").unwrap(),
            });
        }
        None
    }
}

#[derive(PartialEq, Hash, Debug)]
pub struct AccountEntity {
    pub id: Option<i64>,
}
