use crate::outbound_ports::AccountLock;
use domain::account::AccountId;

pub struct NoOpAccountLock {}

impl AccountLock for NoOpAccountLock {
    fn lock_account(&self, _account_id: AccountId) {
        // do nothing
    }

    fn release_account(&self, _account_id: AccountId) {
        // do nothing
    }
}
