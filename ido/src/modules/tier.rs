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


/// Map<Tier, min_point_to_achieve_this_tier>
pub type TierConfigs = UnorderedMap<StakingTier, TierInfo>;


#[derive(BorshSerialize, BorshDeserialize)]
pub struct TierInfo {
    // tier and point: Please see the staking tier on the staking contract
    pub(crate) ticket: u64,
    pub(crate) allocation: u64,
}

/// This is derivative data so plz do not store it in the storage
pub struct UserTierInfo {
    // tier and point: Please see the staking tier on the staking contract
    tier: StakingTier,
    point: Balance,
    ticket: u64,
    alloc: u64,
}

impl UserTierInfo {
    pub fn get_user_tier_info(account_id: AccountId, tier_configs: TierConfigs) -> UserTierInfo {
        // xKULA => tier => other info
        let (user_tier, user_point) = (StakingTier::Tier0, U128(0)); // TODO: Get from staking contract get_user_staking_tier(account_id)
        // TODO: Get from staking contract get_user_staking_matched_tiers(user_point)
        let user_staking_matched_tiers: Vec<(StakingTier, U64, U64)> = vec![
            (StakingTier::Tier3, U64(5000), U64(5000)),
            (StakingTier::Tier2, U64(1000), U64(4000)),
            (StakingTier::Tier1, U64(100), U64(900)),
            (StakingTier::Tier0, U64(0), U64(99)),
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
            let match_count: U64 = total_tier_point / min_tier_point;
            ticket += match_count * tier_info.ticket;
            alloc += match_count * tier_info.allocation;
        }


        UserTierInfo {
            tier: user_tier,
            point: Balance(user_point),
            ticket,
            alloc,
        }
    }
}
