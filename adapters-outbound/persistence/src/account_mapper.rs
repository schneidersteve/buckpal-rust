use crate::{account_repository::AccountEntity, activity_repository::ActivityEntity};
use domain::{
    ar::{
        account::{AccountId, Account},
        activity::{Activity, ActivityId},
    },
    vo::{activity_window::ActivityWindow, money::Money},
};

pub fn map_to_account(
    account: AccountEntity,
    activities: Vec<ActivityEntity>,
    withdrawal_balance: i128,
    deposit_balance: i128,
) -> Account {
    let baseline_balance =
        Money::substract(&Money::of(deposit_balance), &Money::of(withdrawal_balance));
    Account::with_id(
        AccountId(account.id.unwrap()),
        baseline_balance,
        map_to_activity_window(activities),
    )
}

fn map_to_activity_window(activities: Vec<ActivityEntity>) -> ActivityWindow {
    ActivityWindow::new(
        activities
            .iter()
            .map(|ae| {
                Activity::with_id(
                    Some(ActivityId(ae.id.unwrap())),
                    AccountId(ae.owner_account_id),
                    AccountId(ae.source_account_id),
                    AccountId(ae.target_account_id),
                    ae.timestamp,
                    Money::of(ae.amount as i128),
                )
            })
            .collect(),
    )
}

pub fn map_to_activity_entity(activity: &Activity) -> ActivityEntity {
    let amount = activity.money.amount.to_string().parse::<i128>().unwrap() as i64;
    let mut id = None;
    if let Some(aid) = &activity.id {
        id = Some(aid.0);
    }
    ActivityEntity {
        id,
        timestamp: activity.timestamp,
        owner_account_id: activity.owner_account_id.0,
        source_account_id: activity.source_account_id.0,
        target_account_id: activity.target_account_id.0,
        amount,
    }
}
