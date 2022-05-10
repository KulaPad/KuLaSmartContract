use crate::*;

pub type TierConfigsType = HashMap<Tier, TierConfig>;

#[derive(
    BorshSerialize,
    BorshDeserialize,
    Serialize,
    Deserialize,
    PartialEq,
    Debug,
    Clone,
    Eq,
    Hash,
    PartialOrd,
    Copy,
)]
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
    pub ticket: TicketNumber,
    pub allocation: AllocationNumber,
}

impl TierConfig {
    pub fn new(min_point: Balance, ticket: TicketNumber, allocation: AllocationNumber) -> Self {
        Self {
            min_point,
            ticket,
            allocation,
        }
    }

    pub fn get_default_tier_configs() -> TierConfigsType {
        TierConfig::get_default_tier_configs_multiple(TOKEN_DECIMAL)
    }

    pub fn get_default_tier_configs_multiple(digits: u8) -> TierConfigsType {
        let mut cfg = TierConfigsType::new();
        let multiple = u128::pow(10, digits as u32);

        cfg.insert(Tier::Tier0, TierConfig::new(0, 0, 0));
        cfg.insert(Tier::Tier1, TierConfig::new(100 * multiple, 1, 0));
        cfg.insert(Tier::Tier2, TierConfig::new(1_000 * multiple, 12, 0));
        cfg.insert(Tier::Tier3, TierConfig::new(5_000 * multiple, 100, 0));
        cfg.insert(Tier::Tier4, TierConfig::new(10_000 * multiple, 100, 1));

        cfg
    }
}

/// This is derivative data so plz do not store it in the storage
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct UserTierJson {
    // tier and point: Please see the staking tier on the staking contract
    tier: Tier,
    point: U128,
    ticket: u64,
    allocation: u64,
}

impl Default for UserTierJson {
    fn default() -> Self {
        Self {
            tier: Tier::default(),
            point: U128(0),
            ticket: 0,
            allocation: 0,
        }
    }
}
