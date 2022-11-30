use application::inbound_ports::{SendMoneyCommand, SendMoneyUseCase};
use domain::{account::AccountId, money::Money};
use once_cell::sync::OnceCell;
use salvo::prelude::*;

static SEND_MONEY_USE_CASE: OnceCell<Box<dyn SendMoneyUseCase>> = OnceCell::new();

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
pub async fn send_money(req: &mut Request, res: &mut Response) {
    let command = SendMoneyCommand::new(
        AccountId(req.param::<i64>("sourceAccountId").unwrap()),
        AccountId(req.param::<i64>("targetAccountId").unwrap()),
        Money::of(req.param::<i64>("amount").unwrap() as i128),
    );

    SEND_MONEY_USE_CASE.get().unwrap().send_money(command);

    res.set_status_code(StatusCode::OK);
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::account::AccountId;
    use mockall::{mock, predicate::eq};
    use salvo::test::TestClient;

    mock! {
        #[derive(Debug)]
        SendMoneyUseCaseImpl {}
        impl SendMoneyUseCase for SendMoneyUseCaseImpl {
            fn send_money(&self, command: SendMoneyCommand) -> bool;
        }
    }

    #[tokio::test]
    async fn test_send_money() {
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

        let status_code = TestClient::post("http://127.0.0.1:8080/accounts/send/41/42/500")
            .send(&service)
            .await
            .status_code()
            .unwrap();

        assert_eq!(StatusCode::OK, status_code);
    }
}
