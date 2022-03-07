/*
This file is temporary
It should be on staking module instead
 */
use crate::*;

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct UserStakingInfo {
    pub tier: StakingTier,
    pub staked: Balance,
    pub un_staked: Balance,
    pub staked_at: Timestamp,
    pub lock_day_count: u32,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub enum StakingTier {
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TierInfo {
    pub locked_amount: u64,
    pub no_of_tickets: UnorderedMap<u16, u16>,
    pub no_of_allocations: UnorderedMap<u16, u8>,
}

impl TierInfo {
    fn new(tier: StakingTier, locked_amount: u64) -> Self {
        Self {
            locked_amount,
            no_of_tickets: UnorderedMap::new(get_storage_key(StorageKey::TierTicketInnerKey(format!("{:?}", tier)))),
            no_of_allocations: UnorderedMap::new(get_storage_key(StorageKey::TierAllocationInnerKey(format!("{:?}", tier)))),
        }
    }
}

pub(crate) fn initialize_tiers(token_decimal: u8) -> UnorderedMap<StakingTier, TierInfo> {
    let mut tiers = UnorderedMap::new(get_storage_key(StorageKey::TierKey));
    
    let mut tier1 = TierInfo::new(StakingTier::Tier1, 200 * token_decimal as u64);
    let mut tier2 = TierInfo::new(StakingTier::Tier2, 1000 * token_decimal as u64);
    let mut tier3 = TierInfo::new(StakingTier::Tier3, 5000 * token_decimal as u64);
    let mut tier4 = TierInfo::new(StakingTier::Tier4, 10000 * token_decimal as u64);

    tier1.no_of_tickets.insert(&7, &1);
    tier1.no_of_tickets.insert(&14, &2);
    tier1.no_of_tickets.insert(&30, &4);
    tier1.no_of_tickets.insert(&90, &8);
    tier1.no_of_tickets.insert(&180, &12);
    tier1.no_of_tickets.insert(&365, &20);

    tier2.no_of_tickets.insert(&7, &6);
    tier2.no_of_tickets.insert(&14, &12);
    tier2.no_of_tickets.insert(&30, &24);
    tier2.no_of_tickets.insert(&90, &48);
    tier2.no_of_tickets.insert(&180, &72);
    tier2.no_of_tickets.insert(&365, &120);

    tier3.no_of_tickets.insert(&7, &35);
    tier3.no_of_tickets.insert(&14, &70);
    tier3.no_of_tickets.insert(&30, &140);
    tier3.no_of_tickets.insert(&90, &280);
    tier3.no_of_tickets.insert(&180, &420);
    tier3.no_of_tickets.insert(&365, &700);

    tier4.no_of_tickets.insert(&7, &35);
    tier4.no_of_tickets.insert(&14, &70);
    tier4.no_of_tickets.insert(&30, &140);
    tier4.no_of_tickets.insert(&90, &280);
    tier4.no_of_tickets.insert(&180, &420);
    tier4.no_of_tickets.insert(&365, &700);

    tier4.no_of_allocations.insert(&7, &1);
    tier4.no_of_allocations.insert(&30, &2);
    tier4.no_of_allocations.insert(&180, &3);
    
    tiers.insert(&StakingTier::Tier1, &tier1);
    tiers.insert(&StakingTier::Tier2, &tier2);
    tiers.insert(&StakingTier::Tier3, &tier3);
    tiers.insert(&StakingTier::Tier4, &tier4);

    tiers
}