use near_sdk::{AccountId, MockedBlockchain, PromiseResult, VMContext};
use near_sdk::{Balance, BlockHeight, EpochHeight, Timestamp};

const STAKING_TOKEN_ID: &str = "token-kulapad.testnet";
const UNIX_TIME_ONE_SECOND_IN_NANOSECONDS: Timestamp = 1_000_000_000;
const SECONDS_A_MINUTE: u8 = 60;
const MINUTES_AN_HOUR: u8 = 60;
const HOURS_A_DAY: u8 = 24;

fn get_timestamp(days: u16, hours: u8, minutes: u8, seconds: u8) -> Timestamp {
    let mut timestamp: Timestamp = 0;
    timestamp = days as u64 * HOURS_A_DAY as u64 + hours as u64;
    timestamp = timestamp * MINUTES_AN_HOUR as u64 + minutes as u64;
    timestamp = timestamp * SECONDS_A_MINUTE as u64 + seconds as u64;
    timestamp * UNIX_TIME_ONE_SECOND_IN_NANOSECONDS
}

pub fn increase_timestamp(timestamp: &Timestamp, days: u16, hours: u8, minutes: u8, seconds: u8) -> Timestamp {
    timestamp + get_timestamp(days, hours, minutes, seconds)
}

pub fn decrease_timestamp(timestamp: &Timestamp, days: u16, hours: u8, minutes: u8, seconds: u8) -> Timestamp {
    timestamp - get_timestamp(days, hours, minutes, seconds)
}

#[test]
fn test_timestamp() {
    // 2022-02-22 22:22:22 - 1645543342000000000
    // 2022-01-31 21:21:21 - 1643638881000000000
    let timestamp_1: Timestamp = 1645543342_000000000; // 2022-02-22 22:22:22
    let timestamp_2: Timestamp = 1643638881_000000000; // 2022-01-31 21:21:21
    let timestamp_3: Timestamp = 1645543403_000000000; // 2022-02-22 22:23:23

    // increase t1 1s to t3
    assert_eq!(increase_timestamp(&timestamp_1, 0, 0, 0, 61), timestamp_3);
    assert_eq!(increase_timestamp(&timestamp_1, 0, 0, 1, 1), timestamp_3);
    assert_eq!(timestamp_1, increase_timestamp(&timestamp_2, 22, 1, 1, 1));
    assert_eq!(timestamp_2, decrease_timestamp(&timestamp_1, 22, 1, 1, 1));
}

pub fn staking() -> AccountId {
    "staking".to_string()
}

pub fn alice() -> AccountId {
    "alice".to_string()
}
pub fn bob() -> AccountId {
    "bob".to_string()
}

pub fn owner() -> AccountId {
    "owner".to_string()
}

pub fn account_a() -> AccountId {
    "account-a.tesnet".to_string()
}

pub fn account_b() -> AccountId {
    "account-b.tesnet".to_string()
}

pub fn account_c() -> AccountId {
    "account-c.tesnet".to_string()
}

pub fn ft_token_id() -> AccountId {
    STAKING_TOKEN_ID.to_string()
}

pub fn ntoy(near_amount: Balance) -> Balance {
    near_amount * 10u128.pow(24)
}

/// Rounds to nearest
pub fn yton(yocto_amount: Balance) -> Balance {
    (yocto_amount + (5 * 10u128.pow(23))) / 10u128.pow(24)
}

#[macro_export]
macro_rules! assert_eq_in_near {
    ($a:expr, $b:expr) => {
        assert_eq!(yton($a), yton($b))
    };
    ($a:expr, $b:expr, $c:expr) => {
        assert_eq!(yton($a), yton($b), $c)
    };
}

pub struct VMContextBuilder {
    context: VMContext,
}

impl VMContextBuilder {
    pub fn new() -> Self {
        Self {
            context: VMContext {
                current_account_id: "".to_string(),
                signer_account_id: "".to_string(),
                signer_account_pk: vec![0, 1, 2],
                predecessor_account_id: "".to_string(),
                input: vec![],
                epoch_height: 0,
                block_index: 0,
                // Current time: 29/4/2022 - 9h33'
                block_timestamp: 1651199572000000000,
                account_balance: 0,
                account_locked_balance: 0,
                storage_usage: 10u64.pow(6),
                attached_deposit: 0,
                prepaid_gas: 10u64.pow(18),
                random_seed: vec![0, 1, 2],
                is_view: false,
                output_data_receivers: vec![],
            },
        }
    }

    pub fn current_account_id(mut self, account_id: AccountId) -> Self {
        self.context.current_account_id = account_id;
        self
    }

    #[allow(dead_code)]
    pub fn signer_account_id(mut self, account_id: AccountId) -> Self {
        self.context.signer_account_id = account_id;
        self
    }

    pub fn predecessor_account_id(mut self, account_id: AccountId) -> Self {
        self.context.predecessor_account_id = account_id;
        self
    }

    #[allow(dead_code)]
    pub fn block_index(mut self, block_index: BlockHeight) -> Self {
        self.context.block_index = block_index;
        self
    }

    pub fn epoch_height(mut self, epoch_height: EpochHeight) -> Self {
        self.context.epoch_height = epoch_height;
        self
    }

    pub fn attached_deposit(mut self, amount: Balance) -> Self {
        self.context.attached_deposit = amount;
        self
    }

    pub fn account_balance(mut self, amount: Balance) -> Self {
        self.context.account_balance = amount;
        self
    }

    pub fn account_locked_balance(mut self, amount: Balance) -> Self {
        self.context.account_locked_balance = amount;
        self
    }

    pub fn finish(self) -> VMContext {
        self.context
    }

    pub fn block_timestamp(mut self, timestamp: Timestamp) -> Self {
        self.context.block_timestamp = timestamp;
        self
    }
}

pub fn testing_env_with_promise_results(context: VMContext, promise_result: PromiseResult) {
    let storage = near_sdk::env::take_blockchain_interface()
        .unwrap()
        .as_mut_mocked_blockchain()
        .unwrap()
        .take_storage();

    near_sdk::env::set_blockchain_interface(Box::new(MockedBlockchain::new(
        context,
        Default::default(),
        Default::default(),
        vec![promise_result],
        storage,
        Default::default(),
        None
    )));
}

