use domain::{account::AccountId, money::Money};

pub trait SendMoneyUseCase : Send + Sync + std::fmt::Debug {
    fn send_money(&self, command: SendMoneyCommand) -> bool;
}

#[derive(PartialEq, Hash, Debug)]
pub struct SendMoneyCommand {
    pub source_account_id: AccountId,
    pub target_account_id: AccountId,
    pub money: Money,
}

impl SendMoneyCommand {
    // Functions

    pub fn new(source_account_id: AccountId, target_account_id: AccountId, money: Money) -> Self {
        Self {
            source_account_id,
            target_account_id,
            money,
        }
    }
}
