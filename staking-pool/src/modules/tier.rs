use crate::*;
use near_sdk::collections::UnorderedMap;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum Tier {
    Tier0,
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Default for Tier {
    fn default() -> Self { Tier::Tier0 }
}

/// Map<Tier, min_point_to_achieve_this_tier>
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct TierMinPointConfig {
    pub tier: Tier, 
    pub min_point: u64
}

