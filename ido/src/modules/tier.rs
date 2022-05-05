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
pub type TierConfigs = UnorderedMap<Tier, TierInfo>;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TierInfo {
    // tier and point: Please see the staking tier on the staking contract
    pub(crate) ticket: u64,
    pub(crate) allocation: u64,
}

/// This is derivative data so plz do not store it in the storage
pub struct UserTierInfo {
    // tier and point: Please see the staking tier on the staking contract
    tier: Tier,
    point: Balance,
    ticket: u64,
    alloc: u64,
}

impl UserTierInfo {
    pub fn get_user_tier_info(account_id: AccountId, tier_configs: TierConfigs) -> UserTierInfo {
        // xKULA => tier => other info
        let (user_tier, user_point) = (Tier::Tier0, 0); // TODO: Get from staking contract get_user_staking_tier(account_id)
        // TODO: Get from staking contract get_user_staking_matched_tiers(user_point)
        let user_staking_matched_tiers: Vec<(Tier, u64, u64)> = vec![
            (Tier::Tier3, 5000, 5000),
            (Tier::Tier2, 1000, 4000),
            (Tier::Tier1, 100, 900),
            (Tier::Tier0, 0, 99),
        ];  

        let mut ticket: u64 = 0;
        let mut alloc: u64 = 0;

        // user point is spread to all the smaller range
        for (tier, min_tier_point, total_tier_point) in user_staking_matched_tiers {
            let _tier_config = tier_configs.get(&tier);
            if _tier_config.is_none() {
                continue;
            }

            let tier_info = _tier_config.unwrap();
            let match_count: u64 = total_tier_point / min_tier_point;
            ticket += match_count * tier_info.ticket;
            alloc += match_count * tier_info.allocation;
        }


        UserTierInfo {
            tier: user_tier,
            point: user_point,
            ticket,
            alloc,
        }
    }
}
