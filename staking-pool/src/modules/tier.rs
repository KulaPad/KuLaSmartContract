use crate::*;

pub type TierConfigsType = HashMap<Tier, TierConfig>;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug, Clone, Eq, Hash, PartialOrd, Copy)]
#[serde(crate = "near_sdk::serde")]
pub enum Tier {
    Tier0,
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Default for Tier {
    fn default() -> Self {
        Tier::Tier0
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct TierConfig {
    pub min_point: Balance,
}

impl TierConfig {
    pub fn new(min_point: Balance) -> Self {
        Self { min_point }
    }

    pub fn get_default_tier_configs() -> TierConfigsType {
        TierConfig::get_default_tier_configs_multiple(DEFAULT_TOKEN_DECIMAL)
    }

    pub fn get_default_tier_configs_multiple(digits: u8) -> TierConfigsType{
        let mut cfg = TierConfigsType::new();
        let digits: u128 = digits as u128;

        cfg.insert(Tier::Tier0, TierConfig::new(0));
        cfg.insert(Tier::Tier1, TierConfig::new(100 * digits));
        cfg.insert(Tier::Tier2, TierConfig::new(1_000 * digits));
        cfg.insert(Tier::Tier3, TierConfig::new(5_000 * digits));
        cfg.insert(Tier::Tier4, TierConfig::new(10_000 * digits));

        cfg
    }
}

impl StakingContract {
    pub(crate) fn internal_get_tier(&self, point: Balance) -> Tier {
        let mut configs = self.config.tier_configs.iter().map(|a| (*a.0, *a.1)).collect::<Vec<(Tier, TierConfig)>>();
        // Sort the list descending by min point
        configs.sort_by(|a, b| b.1.min_point.cmp(&a.1.min_point));

        for (tier, config) in configs {
            if point >= config.min_point {
                return tier;
            }
        }
        
        Tier::Tier0
    }
}