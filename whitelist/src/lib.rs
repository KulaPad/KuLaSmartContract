use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{init, env, near_bindgen, PanicOnDefault};

use near_sdk::{
    AccountId, log,
    collections::{UnorderedMap, LookupMap, LazyOption},
};
use near_sdk::env::log;


pub type CampaignId = u32;

impl Default for WhiteList {
    fn default() -> Self {
        Self {
            campaign_addresses: UnorderedMap::new(b"b".to_vec()),
        }
    }
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct WhiteList {
    campaign_addresses: UnorderedMap<CampaignId, LookupMap<AccountId, bool>>,
}

#[near_bindgen]
impl WhiteList {
    #[private]
    pub fn add(&mut self, campaign_id: CampaignId, address: AccountId) {
        // log!("Closed @{} with {}", account_id, balance);
        let mut addresses = self.get_or_create_addresses(campaign_id);

        // TODO: Ask: Best performance to update a value hash map in a hashmap
        // How can NEAR update only the changed key instead of update the whole map
        // Because if you read the source code of UnOrderedMap => It's worst
        addresses.insert(&address, &true);
        self.campaign_addresses.insert(&campaign_id, &addresses);
    }

    /// Rm a address to the whitelist
    #[private]
    pub fn remove(&mut self, campaign_id: CampaignId, address: AccountId) {
        let mut addresses = self.get_or_create_addresses(campaign_id);
        addresses.remove(&address);
        self.campaign_addresses.insert(&campaign_id, &addresses);
    }

    pub fn register(&mut self, campaign_id: CampaignId) {
        let account_id = env::signer_account_id();
        let mut addresses = self.get_or_create_addresses(campaign_id);

        addresses.insert(&account_id, &true);
        self.campaign_addresses.insert(&campaign_id, &addresses);
    }

    pub fn is_whitelisted(&mut self, campaign_id: CampaignId) -> bool {
        let addresses = self.get_or_create_addresses(campaign_id);
        let account_id = env::signer_account_id();

        println!("{:?}", self.campaign_addresses.len());

        addresses.contains_key(&account_id)
    }

    /// Remove a campaign from the whitelist
    pub fn remove_campaign(&mut self, campaign_id: CampaignId) {
        assert!(
            self.campaign_addresses.get(&campaign_id).is_some(),
            "Campaign has not been tracked yet"
        );

        self.campaign_addresses.remove(&campaign_id);
    }
}


impl WhiteList {
    /// Add a campaign to the whitelist
    ///
    /// Ask: how *Map key_prefix affect?
    fn add_campaign(&mut self, campaign_id: CampaignId) -> LookupMap<AccountId, bool> {
        assert!(
            // TODO: NEAR-sdk not support check HashMap.contains_key as Rust does, need to make feature request
            // Get might get worst performance than just check the key exist because we need to get the value and deserialize it
            self.campaign_addresses.get(&campaign_id).is_none(),
            "Campaign has already been tracked"
        );

        let addresses = LookupMap::new((String::from("WhiteList_") + &campaign_id.to_string()).as_bytes());
        self.campaign_addresses.insert(&campaign_id, &addresses);

        addresses
    }

    /// Get campaign whitelist addresses
    /// If not exist, create it
    fn get_or_create_addresses(&mut self, campaign_id: CampaignId) -> LookupMap<AccountId, bool> {
        let addresses = self.campaign_addresses.get(&campaign_id);
        if addresses.is_none() {
            self.add_campaign(campaign_id)
        } else {
            addresses.unwrap()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn is_whitelist_after_register() {
        // Basic set up for a unit test
        let predecessor = "alice.testnet".to_string();
        let input = vec![];
        testing_env!(get_context(input, false, predecessor));

        let mut contract = WhiteList::default();
        let campaign_id = 123;
        contract.register(campaign_id);
        assert_eq!(contract.is_whitelisted(campaign_id), true, "Must be whitelisted after registration");
    }

    #[test]
    fn can_remove_campaign() {
        // Basic set up for a unit test
        let predecessor = "alice.testnet".to_string();
        let input = vec![];
        testing_env!(get_context(input, false, predecessor));

        let mut contract = WhiteList::default();
        let campaign_id = 123;
        contract.register(campaign_id);
        contract.register(456);
        contract.register(666);
        contract.register(888);
        println!("==> campaign_addresses keys = {}", contract.campaign_addresses.len());
        assert_eq!(contract.is_whitelisted(campaign_id), true, "Must be whitelisted after registration");


        contract.remove_campaign(campaign_id);
        println!("==> campaign_addresses keys = {}", contract.campaign_addresses.len());
        assert_eq!(contract.is_whitelisted(campaign_id), false, "Must be not whitelisted after removed");
    }
}
