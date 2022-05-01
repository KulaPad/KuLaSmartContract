use crate::*;
use near_sdk::collections::UnorderedMap;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
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
pub type TierMinPointConfigs = UnorderedMap<Tier, u64>;

