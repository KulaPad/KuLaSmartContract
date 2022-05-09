use crate::*;

pub type PointType = u64;

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
    pub min_point: PointType,
}

impl TierConfig {
    pub fn new(min_point: PointType) -> Self {
        Self { min_point }
    }
}

impl StakingContract {
    /// Get user point(xKula) amount by account id
    pub(crate) fn internal_get_user_point(&self, account_id: &AccountId) -> PointType {
        let account: Option<UpgradableAccount> = self.accounts.get(account_id);
        if account.is_some() {
            let acc: Account = Account::from(account.unwrap());
            acc.point
        } else {
            0
        }
    }

    /// Get user tier and point by account id
    pub(crate) fn internal_get_user_tier(&self, account_id: &AccountId) -> (Tier, PointType) {
        let point = self.internal_get_user_point(account_id);
        let mut configs = self.config.tier_configs.iter().map(|a| (*a.0, *a.1)).collect::<Vec<(Tier, TierConfig)>>();
        // Sort the list descending by min point
        configs.sort_by(|a, b| b.1.min_point.cmp(&a.1.min_point));
        let len = configs.len();

        for (tier, config) in configs {
            if point >= config.min_point {
                return (tier, point);
            }
        }
        
        (Tier::Tier0, point)
    }
}
