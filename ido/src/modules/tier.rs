use crate::*;
use near_sdk::collections::UnorderedMap;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum StakingTier {
    Tier0,
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Default for StakingTier {
    fn default() -> Self { StakingTier::Tier0 }
}


#[derive(BorshSerialize, BorshDeserialize)]
pub struct TierInfo {
    pub locked_amount: u64,
    pub no_of_tickets: UnorderedMap<u16, u16>,
    pub no_of_allocations: UnorderedMap<u16, u8>,
}

impl TierInfo {
    pub(crate) fn new(tier: StakingTier, locked_amount: u64) -> Self {
        env::log(format!("Creating tier... -> Tier: {:?}", tier).as_bytes());

        Self {
            locked_amount,
            no_of_tickets: UnorderedMap::new(get_storage_key(StorageKey::TierTicketInnerKey(format!("{:?}", tier)))),
            no_of_allocations: UnorderedMap::new(get_storage_key(StorageKey::TierAllocationInnerKey(format!("{:?}", tier)))),
        }
    }

    pub(crate) fn get_no_of_tickets(&self, locked_days: u16) -> u16 {
        self.no_of_tickets
            .iter()
            .filter(|(days, _)| days <= &locked_days)
            .map(|(_, tickets)| tickets)
            .max()
            .unwrap_or(0)
    }

    pub(crate) fn get_no_of_allocations(&self, locked_days: u16) -> u8 {
        self.no_of_allocations
            .iter()
            .filter(|(days, _)| days <= &locked_days)
            .map(|(_, tickets)| tickets)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Serialize, Deserialize)]
/// Stores the information of an account
/// The tier is from Tier1 to Tier4, if the locked amount is less than Tier1, it would be Tier0
/// The no_of_staking_tickets is the number of staking tickets of current tier with a number of locked days.
/// The no_of_allocations is user for Tier4 only. This is the number of allocation that users can have a ido buy slot.
pub struct TierInfoJson {
    pub tier: StakingTier,
    pub locked_amount: U64,
    pub locked_days: u32,
    pub calculating_time: Timestamp,
    pub no_of_staking_tickets: TicketAmount,
    pub no_of_allocations: TicketAmount,
}

impl Default for TierInfoJson {
    fn default() -> Self {
        Self {
            tier: StakingTier::default(),
            locked_amount: U64::from(0),
            locked_days: 0,
            calculating_time: 0,
            no_of_staking_tickets: 0,
            no_of_allocations: 0,
        }
    }
}
