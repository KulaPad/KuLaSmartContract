use crate::*;
pub use std::collections::HashMap;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, PartialOrd)]
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
// #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Clone,Debug)]
// #[serde(crate = "near_sdk::serde")]
// pub struct TierMinPointConfigs {
//     pub tier: Tier, 
//     pub min_point: u64
// }

pub type TierMinPointConfigs =  HashMap<Tier, u64>;

