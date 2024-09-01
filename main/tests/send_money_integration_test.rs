#[cfg(test)]
mod tests {
    use application::{
        no_op_account_lock::NoOpAccountLock,
        outbound_ports::LoadAccountPort,
        send_money_use_case::{MoneyTransferProperties, SendMoneyUseCaseImpl},
    };
    use chrono::Local;
    use domain::{ar::account::AccountId, vo::money::Money};
    use env_logger::WriteStyle;
    use log::LevelFilter;
    use persistence::{
        account_persistence_adapter::AccountPersistenceAdapter,
        account_repository::AccountRepositoryImpl, activity_repository::ActivityRepositoryImpl,
    };
    use reqwest::StatusCode;
    use salvo::{prelude::TcpListener, Listener, Server};
    use sqlx::{migrate, sqlite::SqlitePoolOptions, SqlitePool};
    use std::sync::Arc;
    use tokio::sync::oneshot;
    use rest::send_money_handler::{get_routes, set_dependencies};

    // #[tokio::test]
    // async fn test_send_money() {
    //     // Setup
    //     let db_pool = create_db_pool().await;
    //     migrate_database(db_pool.clone()).await;

    //     env_logger::Builder::new()
    //         .filter(
    //             Some("adapters_outbound_persistence::account_persistence_adapter"),
    //             LevelFilter::Debug,
    //         )
    //         .write_style(WriteStyle::Never)
    //         // .is_test(true)
    //         .init();

    //     let load_account_port = wire_dependencies(db_pool);

    //     let (tx, rx) = oneshot::channel();
    //     let acceptor = TcpListener::new("127.0.0.1:8080").bind().await;
    //     let server = Server::new(acceptor);
    //     server.try_serve(
    //         get_routes()
    //     );
    //     tokio::task::spawn(server);

    //     // Given initial source account balance
    //     let source_account_id = AccountId(1);
    //     let source_account = load_account_port
    //         .load_account(source_account_id.clone(), Local::now().naive_local())
    //         .await;
    //     let initial_source_balance = source_account.calculate_balance();

    //     // And initial target account balance
    //     let target_account_id = AccountId(2);
    //     let target_account = load_account_port
    //         .load_account(target_account_id.clone(), Local::now().naive_local())
    //         .await;
    //     let initial_target_balance = target_account.calculate_balance();

    //     // When money is send
    //     let money = Money::of(500);
    //     let resp = reqwest::Client::new()
    //         .post(format!(
    //             "http://localhost:8080/accounts/send/{}/{}/{}",
    //             source_account_id.0, target_account_id.0, money.amount
    //         ))
    //         .send()
    //         .await
    //         .unwrap();

    //     // Then http status is OK
    //     assert_eq!(StatusCode::OK, resp.status());

    //     // And source account balance is correct
    //     let source_account = load_account_port
    //         .load_account(source_account_id, Local::now().naive_local())
    //         .await;
    //     assert_eq!(
    //         source_account.calculate_balance(),
    //         initial_source_balance.minus(&money)
    //     );

    //     // And target account balance is correct
    //     let target_account = load_account_port
    //         .load_account(target_account_id, Local::now().naive_local())
    //         .await;
    //     assert_eq!(
    //         target_account.calculate_balance(),
    //         initial_target_balance.plus(&money)
    //     );

    //     // Shutdown
    //     let _ = tx.send(());
    // }

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

    fn wire_dependencies(db_pool: SqlitePool) -> Arc<dyn LoadAccountPort> {
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
            account_persistence_adapter.clone(),
            money_transfer_properties,
        ));
        set_dependencies(send_money_use_case);

        account_persistence_adapter
    }
}
