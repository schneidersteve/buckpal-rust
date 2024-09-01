use std::sync::OnceLock;

use application::inbound_ports::{SendMoneyCommand, SendMoneyUseCase};
use domain::{ar::account::AccountId, vo::money::Money};
use salvo::prelude::*;

static SEND_MONEY_USE_CASE: OnceLock<Box<dyn SendMoneyUseCase>> = OnceLock::new();

pub fn set_dependencies(smuc: Box<dyn SendMoneyUseCase>) {
    SEND_MONEY_USE_CASE.set(smuc).unwrap();
}

// POST /accounts/send/<sourceAccountId>/<targetAccountId>/<amount>
pub fn get_routes() -> Router {
    Router::with_path("accounts").push(
        Router::with_path("send/<sourceAccountId:num>/<targetAccountId:num>/<amount:num>")
            .post(send_money),
    )
}

#[handler]
async fn send_money(req: &mut Request, res: &mut Response) {
    let command = SendMoneyCommand::new(
        AccountId(req.param::<i64>("sourceAccountId").unwrap()),
        AccountId(req.param::<i64>("targetAccountId").unwrap()),
        Money::of(req.param::<i64>("amount").unwrap() as i128),
    );

    SEND_MONEY_USE_CASE.get().unwrap().send_money(command).await;

    res.status_code(StatusCode::OK);
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::eq};
    use salvo::test::TestClient;

    mock! {
        #[derive(Debug)]
        SendMoneyUseCaseImpl {}
        #[async_trait]
        impl SendMoneyUseCase for SendMoneyUseCaseImpl {
            async fn send_money(&self, command: SendMoneyCommand) -> bool;
        }
    }

    #[tokio::test]
    async fn test_send_money() {
        // Given
        let mut smuc = Box::new(MockSendMoneyUseCaseImpl::new());
        smuc.expect_send_money()
            .times(12)
            .with(eq(SendMoneyCommand::new(
                AccountId(41),
                AccountId(42),
                Money::of(500),
            )))
            .return_const(true);
        super::set_dependencies(smuc);

        let service = Service::new(super::get_routes());

        // When
        let status_code = TestClient::post("http://127.0.0.1:8080/accounts/send/41/42/500")
            .send(&service)
            .await
            .status_code
            .unwrap();

        // Then
        assert_eq!(StatusCode::OK, status_code);
    }
}
