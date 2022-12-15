use application::{
    no_op_account_lock::NoOpAccountLock,
    send_money_use_case::{MoneyTransferProperties, SendMoneyUseCaseImpl},
};
use domain::money::Money;
use persistence::{
    account_persistence_adapter::AccountPersistenceAdapter,
    account_repository::AccountRepositoryImpl, activity_repository::ActivityRepositoryImpl,
};
use salvo::prelude::*;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, migrate};
use std::sync::Arc;
use web::send_money_handler::{get_routes, set_dependencies};

#[tokio::main]
async fn main() {
    let db_pool = create_db_pool().await;
    migrate_database(db_pool.clone()).await;

    wire_dependencies(db_pool);

    println!("Server Running: http://127.0.0.1:8080");
    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .serve(get_routes())
        .await;
}

async fn create_db_pool() -> SqlitePool {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .unwrap()
}

async fn migrate_database(db_pool: SqlitePool) {
    migrate!("./migrations").run(&db_pool).await.unwrap();
}

fn wire_dependencies(db_pool: SqlitePool) {
    let account_repository = Box::new(AccountRepositoryImpl::new(db_pool.clone()));
    let activity_repository = Box::new(ActivityRepositoryImpl::new(db_pool.clone()));
    let account_persistence_adapter = Arc::new(AccountPersistenceAdapter::new(
        account_repository,
        activity_repository,
    ));

    let account_lock = Box::new(NoOpAccountLock {});

    let money_transfer_properties = MoneyTransferProperties::new(Some(Money::of(1_000)));

    let send_money_use_case = Box::new(SendMoneyUseCaseImpl::new(
        account_persistence_adapter.clone(),
        account_lock,
        account_persistence_adapter,
        money_transfer_properties,
    ));
    set_dependencies(send_money_use_case);
}
