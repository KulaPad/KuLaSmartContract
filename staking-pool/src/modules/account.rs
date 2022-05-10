use near_sdk::Timestamp;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradableAccount {
    Default(Account),
    Current(Account),
}

impl From<UpgradableAccount> for Account {
    fn from(account: UpgradableAccount) -> Self {
        match account {
            UpgradableAccount::Default(account) => account,
            UpgradableAccount::Current(account) => account,
        }
    }
}

impl From<Account> for UpgradableAccount {
    fn from(account: Account) -> Self {
        UpgradableAccount::Current(account)
    }
}

#[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Account {
    pub locked_balance: Balance,
    pub locked_timestamp: Timestamp,
    pub locked_days: DayType,
    pub staked_balance: Balance,
    pub pre_stake_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub unstaked_balance: Balance,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch_height: EpochHeight,
    pub point: Balance,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub locked_balance: U128,
    pub unlocked_timestamp: Timestamp,
    pub staked_balance: U128,
    pub unstaked_balance: U128,
    pub reward: U128,
    pub can_withdraw: bool,
    pub start_unstake_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
    pub current_epoch: EpochHeight,
    pub tier: Tier,
    pub point: U128,
}

impl Account {
    pub fn get_available_unlocked_amount(&self) -> Balance {
        self.staked_balance - self.locked_balance
    }

    pub fn get_unlocked_timestamp(&self) -> Timestamp {
        self.locked_timestamp + self.locked_days as u64 * ONE_DAY_IN_NANOSECOND
    }

    pub fn get_remaining_locked_days(&self, current_timestamp: Timestamp) -> DayType {
        if self.get_unlocked_timestamp() <= current_timestamp {
            return 0;
        }

        ((self.get_unlocked_timestamp() - current_timestamp) % ONE_DAY_IN_NANOSECOND + 1) as DayType
    }

    pub fn calculate_point(
        &mut self,
        amount: Balance,
        locked_days: DayType,
        min_locking_days: DayType,
        max_locking_days: DayType,
        current_block_timestamp: Timestamp,
    ) {
        // Is the first lock? Required amount > 0 and locked_time >= 1 day in nanosecond
        if self.locked_balance == 0 {
            assert!(amount > 0, "ERR_AMOUNT_MUST_GREATER_THAN_ZERO");
            assert!(
                amount <= self.get_available_unlocked_amount(),
                "ERR_AMOUNT_MUST_LESS_THAN_BALANCE"
            );
            assert!(
                locked_days >= min_locking_days && locked_days <= max_locking_days,
                "ERR_LOCKED_DAYS_MUST_BETWEEN_MIN_AND_MAX"
            );

            self.point = Self::get_point(amount, Self::get_nanoseconds_from_days(locked_days));
            self.locked_balance = amount;
            self.locked_timestamp = current_block_timestamp;
            self.locked_days = locked_days;
        } else if amount > 0 {
            // Is increasing locked amount?
            assert!(
                locked_days == 0,
                "ERR_INCREASING_LOCKED_AMOUNT_THEN_DAYS_MUST_BE_ZERO"
            );
            assert!(
                amount <= self.get_available_unlocked_amount(),
                "ERR_AMOUNT_MUST_LESS_THAN_BALANCE"
            );
            assert!(
                self.get_unlocked_timestamp() > current_block_timestamp,
                "ERR_LOCK_TIME_EXPIRED"
            );

            self.point += Self::get_point(
                amount,
                self.get_unlocked_timestamp() - current_block_timestamp,
            );
            self.locked_balance += amount;
        } else if locked_days > 0 {
            // Is extending locked days?
            assert!(
                amount == 0,
                "ERR_EXTENDING_LOCKED_TIME_THEN_AMOUNT_MUST_BE_ZERO"
            );

            // The locked time is end but user has not unlocked token. -> Extend locked days & keep current point.
            if self.get_unlocked_timestamp() < current_block_timestamp {
                assert!(
                    locked_days >= min_locking_days && locked_days <= max_locking_days,
                    "ERR_LOCKED_DAYS_MUST_BETWEEN_MIN_AND_MAX"
                );

                self.point += Self::get_point(
                    self.locked_balance,
                    Self::get_nanoseconds_from_days(locked_days),
                );
                // When locked time is end, locked_days should be restarted.
                self.locked_days = locked_days;
            } else {
                // The locked time is on-going.
                let max_locking_days =
                    max_locking_days - self.get_remaining_locked_days(current_block_timestamp);
                assert!(
                    locked_days >= min_locking_days && locked_days <= max_locking_days,
                    "ERR_LOCKED_DAYS_MUST_BETWEEN_MIN_AND_MAX"
                );

                self.point += Self::get_point(
                    self.locked_balance,
                    Self::get_nanoseconds_from_days(locked_days),
                );
                // When locked time is on-going, locked_days should be accumulated.
                self.locked_days += locked_days;
            }
        } else {
            panic!("ERR_MUST_BE_SPECIFIC_TIME_OR_AMOUNT");
        }
    }

    /// The locked_time should be 1 day = 8460000000000000 nanoseconds
    pub fn get_point(amount: Balance, locked_time: Timestamp) -> Balance {
        let one_year_in_nanosecond: u64 = POINT_100_PERCENT_IN_NANOSECOND;
        amount * locked_time as u128 / one_year_in_nanosecond as u128
    }

    pub fn get_nanoseconds_from_days(days: DayType) -> Timestamp {
        days as u64 * ONE_DAY_IN_NANOSECOND
    }
}

#[cfg(all(test))]
mod tests {
    use super::*;
    const TOKEN_DECIMAL: u128 = DEFAULT_TOKEN_DECIMAL as u128;
    const MIN_LOCKING_DAYS: DayType = 1;
    const MAX_LOCKING_DAYS: DayType = 360;

    fn get_account(
        staked_balance: Balance,
        locked_balance: Balance,
        locked_days: DayType,
        locked_timestamp: Timestamp,
        point: Balance,
    ) -> Account {
        Account {
            locked_balance,
            locked_timestamp,
            locked_days,
            staked_balance,
            pre_stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: 0,
            unstaked_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch_height: 0,
            point,
        }
    }

    fn get_balance_with_decimal(balance: Balance) -> Balance {
        balance * Balance::pow(10, TOKEN_DECIMAL as u32)
    }

    #[test]
    #[should_panic(expected = "ERR_AMOUNT_MUST_GREATER_THAN_ZERO")]
    fn test_account_calculate_point_error_1() {
        let mut account = get_account(0, 0, 0, 0, 0);
        account.calculate_point(0, 0, 0, 0, 0);
    }

    fn test_account_calculate_point(
        current_block_timestamp: Timestamp,
        current_staked_amount: Balance,
        current_locked_amount: Balance,
        current_locked_days: DayType,
        current_locked_timestamp: Timestamp,
        current_point: Balance,
        locked_amount: Balance,
        locked_days: DayType,
        expected_locked_amount: Balance,
        expected_locked_days: DayType,
        expected_locked_timestamp: Timestamp,
        expected_point: Balance,
    ) {
        let mut account = get_account(
            current_staked_amount,
            current_locked_amount,
            current_locked_days,
            current_locked_timestamp,
            current_point,
        );

        account.calculate_point(
            locked_amount,
            locked_days,
            MIN_LOCKING_DAYS,
            MAX_LOCKING_DAYS,
            current_block_timestamp,
        );

        // Asserts locked_balance, locked_days, locked_timestamp, point
        assert_eq!(expected_locked_amount, account.locked_balance, "locked_amount");
        assert_eq!(expected_locked_days, account.locked_days, "locked_days");
        assert_eq!(expected_locked_timestamp, account.locked_timestamp, "locked_timestamp");
        assert_eq!(expected_point, account.point, "point");
    }

    #[test]
    fn test_account_calculate_point_for_lock() {
        // Current timestamp: 1652151705_000_000_000 - 2022-05-10 3:01:45 (GMT)
        let current_block_timestamp: Timestamp = 1652151705_000_000_000;

        // Staked: 100, locked: 0, locked days: 0, locked timestamp: , point: 0
        let current_staked_amount = get_balance_with_decimal(100);
        let current_locked_amount = get_balance_with_decimal(0);
        let current_locked_days = 0;
        let current_locked_timestamp = 0;
        let current_point = get_balance_with_decimal(0);

        // Lock 100 TOKEN for 36 days
        // * Amount: 100_00_000_000
        // * Locked days: 36 days
        // * Min locked days: 1 day
        // * Max locked days: 360 days
        let locked_amount = get_balance_with_decimal(100);
        let locked_days = 36;

        // Expected result
        let expected_locked_amount = locked_amount;
        let expected_locked_days = locked_days;
        let expected_locked_timestamp = current_block_timestamp;
        let expected_point = get_balance_with_decimal(10);

        test_account_calculate_point(
            current_block_timestamp,
            current_staked_amount,
            current_locked_amount,
            current_locked_days,
            current_locked_timestamp,
            current_point,
            locked_amount,
            locked_days,
            expected_locked_amount,
            expected_locked_days,
            expected_locked_timestamp,
            expected_point,
        );
    }

    #[test]
    fn test_account_calculate_point_for_extending_locked_time() {
        // Current timestamp: 1652151705_000_000_000 - 2022-05-10 3:01:45 (GMT)
        let current_block_timestamp: u64 = 1652151705_000_000_000;

        // Staked: 100, locked: 100, locked days: 36, locked timestamp: 16 days ago (remaining: 20 days), point: 10
        let current_staked_amount = get_balance_with_decimal(100);
        let current_locked_amount = get_balance_with_decimal(100);
        let current_locked_days = 36;
        let current_locked_timestamp = current_block_timestamp - ONE_DAY_IN_NANOSECOND * 16;
        let current_point = get_balance_with_decimal(10);

        // Extend locked time for 100 TOKEN for 36 days
        // * Amount: 0
        // * Locked days: 36 days (more)
        // * Min locked days: 1 day
        // * Max locked days: 360 days
        let locked_amount = get_balance_with_decimal(0);
        let locked_days = 36;

        // Expected result
        let expected_locked_amount = current_locked_amount;
        let expected_locked_days = current_locked_days + locked_days;
        let expected_locked_timestamp = current_locked_timestamp;
        let expected_point = current_point + get_balance_with_decimal(10);

        test_account_calculate_point(
            current_block_timestamp,
            current_staked_amount,
            current_locked_amount,
            current_locked_days,
            current_locked_timestamp,
            current_point,
            locked_amount,
            locked_days,
            expected_locked_amount,
            expected_locked_days,
            expected_locked_timestamp,
            expected_point,
        );
    }

    #[test]
    fn test_account_calculate_point_for_increasing_locked_amount() {
        // Current timestamp: 1652151705_000_000_000 - 2022-05-10 3:01:45 (GMT)
        let current_block_timestamp: u64 = 1652151705_000_000_000;

        // Staked: 100, locked: 100, locked days: 36, locked timestamp: 16 days ago (remaining: 20 days), point: 10
        let current_staked_amount = get_balance_with_decimal(200);
        let current_locked_amount = get_balance_with_decimal(100);
        let current_locked_days = 36;
        let current_locked_timestamp = current_block_timestamp - ONE_DAY_IN_NANOSECOND * 16;
        let current_point = get_balance_with_decimal(10);

        // Increase locked amount for 100 TOKEN more
        // * Amount: 100
        // * Locked days: remaining
        // * Min locked days: 1 day
        // * Max locked days: 360 days
        let locked_amount = get_balance_with_decimal(100);
        let locked_days = 0;

        // Expected result
        let expected_locked_amount = current_locked_amount + locked_amount;
        let expected_locked_days = current_locked_days;
        let expected_locked_timestamp = current_locked_timestamp;
        let expected_point = current_point + get_balance_with_decimal(100) * 20 / 360;

        test_account_calculate_point(
            current_block_timestamp,
            current_staked_amount,
            current_locked_amount,
            current_locked_days,
            current_locked_timestamp,
            current_point,
            locked_amount,
            locked_days,
            expected_locked_amount,
            expected_locked_days,
            expected_locked_timestamp,
            expected_point,
        );
    }
}
