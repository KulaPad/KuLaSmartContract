use crate::*;

pub(crate) const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

pub(crate) fn get_storage_key(key: StorageKey) -> Vec<u8> {
    key.try_to_vec().unwrap()
}

pub(crate) fn get_token_raised_amount(human_amount: u64, token_decimal: u8) -> Balance {
    human_amount as u128 * u128::pow(10, token_decimal as u32) 
}

pub(crate) fn get_token_sale_rate(system_amount: u128, numberator: u64, denominator: u64) -> Balance {
    system_amount * numberator as u128 / denominator as u128
}

pub(crate) fn get_token_raised_human_amount(amount: u128, token_decimal: u8) -> u128 {
    amount / u128::pow(10, token_decimal as u32) 
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash{
    let mut hash = CryptoHash::default();
    // Hash account ID rồi return chính nó
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));

    hash
}

pub(crate) fn get_current_time() -> Timestamp {
    env::block_timestamp()
}

pub(crate) fn panic_project_not_exist() {
    panic!("Project does not exist.");
}

pub(crate) fn assert_project_whitelist_period(project: &ProjectInfo) {
    assert!(project.is_in_whitelist_period(), "Project isn't in whitelist period.");
}

impl IDOContract{
    pub(crate) fn assert_project_exist(&self, project_id: ProjectId) {
        assert!(self.internal_has_project(project_id), "Project does not exist.");
    }

    // Function use for testing_buytoken
    //#[private]
    pub fn create_default_account_token_sales(&mut self, project_id: ProjectId, account_id: &AccountId){
        let default_account_token_sales = AccountTokenSales{
            funding_amount: 0,
            token_unlocked_amount:0,
            token_locked_amount:0,
            token_withdrawal_amount:0,
        };

        self.assert_project_exist(project_id);

        let mut account_token_sales = self.project_account_token_sales.get(&project_id).unwrap();
        account_token_sales.insert(&account_id, &default_account_token_sales);
        self.project_account_token_sales.insert(&project_id, &account_token_sales);
    }

    pub fn get_project_info(&self, project_id: &ProjectId)-> ProjectInfo{

        let project_info = self.projects.get(&project_id)
                                                .expect("No project found");
        project_info
    }

    pub fn unwrap_account_project(&self,account_id: &AccountId)
            -> UnorderedSet<ProjectId> {
        let account_projects = self.account_projects
                                    .get(&account_id)
                                    .unwrap_or_else(|| {
                                        UnorderedSet::new(
                                            get_storage_key(StorageKey::AccountProjectKeyInnerKey{
                                                account_id_hash: hash_account_id(&account_id)
                                            })
                                        )
                                    });
        account_projects
    }

    pub fn unwrap_project_account_token_sales(&self, project_id: ProjectId) 
            -> UnorderedMap<AccountId, AccountTokenSales> {
        self.assert_project_exist(project_id);
        let project_account_token_sales = self.project_account_token_sales.get(&project_id).unwrap();
        project_account_token_sales
    }

    pub fn unwrap_project_account_ticket(&self, project_id: ProjectId, account_id: &AccountId) -> AccountTickets{
        let project_account_tickets = self.project_account_tickets.get(&project_id)
                                                        .expect("No project found");
        let account_tickets = project_account_tickets.get(account_id)
                                                        .expect("Account didn't join whitelist");
        account_tickets
    }
}