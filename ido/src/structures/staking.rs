/*
This file is temporary
It should be on staking module instead
 */
// use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{
    Balance, Timestamp
};

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct UserStakingInfo {
    pub tier: StakingTier,
    pub staked: Balance,
    pub un_staked: Balance,
    pub staked_at: Timestamp,
    pub lock_day_count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum StakingTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}
