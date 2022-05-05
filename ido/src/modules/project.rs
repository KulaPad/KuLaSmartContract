use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectStatus {
    Preparation,
    Whitelist,
    Sales,
    Distribution,
}

impl Default for ProjectStatus {
    fn default() -> Self { ProjectStatus::Preparation }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub enum WhitelistType {
    None,
    XToken(Balance),
    Ticket
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub enum SaleType {
    Shared {
        min_allocation_per_user: Balance,
        max_allocation_per_user: Balance,
    },
    Lottery {
        allocation_per_ticket: Balance,
        total_tickets: TicketNumber,
        win_ticket_ids: Option<Vec<TicketNumber>>,
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub enum DistributionType {
    Unlocked,
    Vested
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Rate {
    numberator: u64,
    denominator: u64,
}

impl Rate {
    pub(crate) fn new(numberator: u64, denominator: u64) -> Self {
        Self {
            numberator,
            denominator,
        }
    }

    pub(crate) fn multiply(&self, amount: u128) -> u128 {
        amount * self.numberator as u128 / self.denominator as u128
    }

    pub(crate) fn devided_by(&self, amount: u128) -> u128 {
        amount * self.denominator as u128 / self.numberator as u128
    }

    pub(crate) fn get_rate(&self) -> f64 {
        self.numberator as f64 / self.denominator as f64
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct Project {
    pub owner_id: AccountId,

    pub whitelist_start_date: Timestamp,
    pub whitelist_end_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,

    /// your.near
    pub token_contract_id: AccountId,
    /// The amount of tokens to be sold in this campaign like: 30.000.000 TOKEN
    pub token_raised_amount: Balance,
    /// The price of a token like: 0.01 (NEAR) / 1 TOKEN => numberator: 1, denominator: 100
    pub token_sale_rate: Rate,

    /// None
    pub fund_contract_id: AccountId,
    /// The total fund that users deposited to buy token
    pub total_fund_committed: Balance,

    pub status: ProjectStatus,
    pub whitelist_type: WhitelistType,
    pub sale_type: SaleType,
    pub distribution_type: DistributionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInput {
    pub owner_id: AccountId,

    pub whitelist_start_date: Timestamp,
    pub whitelist_end_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,

    pub token_contract_id: AccountId,
    pub token_raised_amount: U128,
    pub token_sale_rate_numberator: u64,
    pub token_sale_rate_denominator: u64,

    pub fund_contract_id: AccountId,
    
    pub whitelist_type: WhitelistType,
    pub sale_type: SaleType,
    pub distribution_type: DistributionType,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectJson {
    pub id: ProjectId,
    pub whitelist_start_date: Timestamp,
    pub whitelist_end_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,

    pub token_contract_id: AccountId,
    pub token_raised_amount: U128,
    pub token_sale_rate: f64,

    pub fund_contract_id: AccountId,
    pub total_fund_committed: U128,
    pub hard_cap: U128,
    pub whitelist_accounts: u64,

    pub status: ProjectStatus,
    pub whitelist_type: WhitelistType,
    pub sale_type: SaleType,
    pub distribution_type: DistributionType,
}

// Project functions
impl Project {
    pub fn from(project: ProjectInput) -> Project {
        Self {
            owner_id: project.owner_id.clone(),
            whitelist_start_date: project.whitelist_start_date,
            whitelist_end_date: project.whitelist_end_date,
            sale_start_date: project.sale_start_date,
            sale_end_date: project.sale_end_date,
            token_contract_id: project.token_contract_id.clone(),
            token_raised_amount: project.token_raised_amount.0,
            token_sale_rate: Rate {
                numberator: project.token_sale_rate_numberator,
                denominator: project.token_sale_rate_denominator,
            },
            fund_contract_id: project.fund_contract_id.clone(),
            total_fund_committed: 0,
            status: ProjectStatus::Preparation,
            whitelist_type: project.whitelist_type,
            sale_type: project.sale_type,
            distribution_type: project.distribution_type,
        }
    }

    pub(crate) fn assert_whitelist_period(&self) {
        assert!(self.is_in_whitelist_period(), "Project isn't in whitelist period.");
    }

    pub(crate) fn assert_sale_period(&self) {
        assert!(self.is_in_sale_period(), "Project isn't in sale period.");
    }
    
    pub(crate) fn is_in_whitelist_period(&self) -> bool {
        let current_time = get_current_time();
        self.status == ProjectStatus::Whitelist && self.whitelist_start_date <= current_time && current_time <= self.whitelist_end_date
    }

    pub(crate) fn is_in_sale_period(&self) -> bool {
        let current_time = get_current_time();
        self.status == ProjectStatus::Sales && self.sale_start_date <= current_time && current_time <= self.sale_end_date
    }

    pub(crate) fn is_in_distribution_period(&self) -> bool {
        let current_time = get_current_time();
        self.status == ProjectStatus::Distribution && self.sale_end_date <= current_time
    }

    pub(crate) fn get_hard_cap(&self) -> Balance {
        self.token_sale_rate.multiply(self.token_raised_amount)
    }
}

// Internal functions
impl IDOContract {
    // Assert & Get or panic functions
    pub(crate) fn assert_project_exist(&self, project_id: ProjectId) {
        assert!(self.internal_has_project(project_id), "Project does not exist.");
    }

    // Projects

    pub(crate) fn internal_get_project_or_panic(&self, project_id: ProjectId) -> Project {
        self.projects.get(&project_id).expect("Project does not exist.")
    }

    // Accounts by Project

    pub(crate) fn internal_get_accounts_by_project_or_panic(&self, project_id: ProjectId) -> ProjectAccountUnorderedMap {
        self.accounts_by_project.get(&project_id).expect("Project account tickets do not exist.")
    }

    pub(crate) fn internal_get_account_by_project_or_panic(&self, project_id: ProjectId, account_id: &AccountId) -> ProjectAccount {
        self.internal_get_accounts_by_project_or_panic(project_id).get(&account_id).expect("The account doesn't belong to the project.")
    }

    pub(crate) fn internal_get_account_by_project(&self, project_id: ProjectId, account_id: &AccountId) -> Option<ProjectAccount> {
        self.internal_get_accounts_by_project_or_panic(project_id).get(&account_id)
    }
  
    // Projects by Account
  
    pub(crate) fn internal_get_projects_by_account_or_default(&self, account_id: &AccountId)
        -> ProjectIdUnorderedSet {
            self.projects_by_account
                .get(&account_id)
                .unwrap_or_else(|| {
                    UnorderedSet::new(
                        get_storage_key(StorageKey::ProjectsByAccountInnerKey{
                            account_id_hash: hash_account_id(&account_id)
                        })
                    )
                })
    }

    // Tickets by Project

    pub(crate) fn internal_get_tickets_by_project_or_panic(&self, project_id: ProjectId) -> TicketAndAccountLookupMap {
        self.tickets_by_project.get(&project_id).expect("Project tickets do not exist.")
    }

    // Create and modify project

    pub(crate) fn internal_create_project(&mut self, project: Project) -> ProjectId{
        // Get next Id
        let project_id = self.projects.len() + 1;

        // Insert the project
        self.projects.insert(&project_id, &project);

        // Insert this project to related variables, this should be done by each status
        self.accounts_by_project.insert(&project_id, &UnorderedMap::new(get_storage_key(StorageKey::AccountsByProjectInnerKey(project_id)))); 
        self.tickets_by_project.insert(&project_id, &LookupMap::new(get_storage_key(StorageKey::TicketsByProjectInnerKey(project_id)))); 
        
        project_id
    }

    pub(crate) fn internal_change_project_status(&mut self, project_id: ProjectId) {
        let mut project = self.internal_get_project_or_panic(project_id);
        let current_time = get_current_time();
        match project.status {
            ProjectStatus::Preparation => {
                assert!(project.whitelist_start_date <= current_time && current_time <= project.whitelist_end_date, "Cannot change project's status to Whitelist");
                project.status = ProjectStatus::Whitelist;
            },
            ProjectStatus::Whitelist => {
                assert!(project.whitelist_end_date <= current_time,
                        "{}", format!("The whitelist period (End: {} - Current: {}) is not end.",
                        project.whitelist_end_date, current_time));
                project.status = ProjectStatus::Sales;
            }
            ProjectStatus::Sales => {
                assert!(project.sale_end_date < current_time, "Cannot change project's status to Distribution.");
                project.status = ProjectStatus::Distribution;
                self.internal_distribute_token_to_users(project_id);
            }
            _ => panic!("Unable to change project status.")
        }

        // Update project
        self.projects.insert(&project_id, &project);
    }

    // Project Json

    pub(crate) fn internal_get_project(&self, project_id: ProjectId, project: Option<Project>) -> Option<ProjectJson> {
        if let Some(project) = project {
            let whitelist_accounts = self.internal_get_accounts_by_project_or_panic(project_id).len();

            Some(ProjectJson {
                id: project_id,
                
                whitelist_start_date: project.whitelist_start_date,
                whitelist_end_date: project.whitelist_end_date,
                sale_start_date: project.sale_start_date,
                sale_end_date: project.sale_end_date,
                
                token_contract_id: project.token_contract_id.clone(),
                token_raised_amount: U128(project.token_raised_amount),
                token_sale_rate: project.token_sale_rate.get_rate(),
                
                fund_contract_id: project.fund_contract_id.clone(),
                total_fund_committed: U128::from(project.total_fund_committed),
                hard_cap: U128::from(project.get_hard_cap()),
                whitelist_accounts: whitelist_accounts,

                status: project.status,
                whitelist_type: project.whitelist_type,
                sale_type: project.sale_type,
                distribution_type: project.distribution_type,
            })
        } else {
            None
        }
    }

    pub(crate) fn internal_has_project(&self, project_id: ProjectId) -> bool{
        self.projects.get(&project_id).is_some()
    }
    
    // Project Whitelist
    // If project has no whitelist type or the whitelist type is ticket, 
    // user can register whitelist without any requires.
    // If project's whitelist type is fixed Xtoken, 
    // need to check user has enough xtoken or not.
    pub(crate) fn internal_register_whitelist(&mut self, project_id: ProjectId) {
        let project = self.internal_get_project_or_panic(project_id);                         
        assert_eq!(project.status, ProjectStatus::Whitelist,"Project isn't on whitelist");
        assert!(project.is_in_whitelist_period(), "Project isn't on whitelist time");
        let account_id = env::signer_account_id();

        match project.whitelist_type {
            WhitelistType::None =>{ 
                self.internal_add_account(&account_id, project_id);
            },
            WhitelistType::XToken(xtoken) => {
                self.internal_register_fixed_xtoken_project(project_id, account_id, xtoken);
            },
            WhitelistType::Ticket => {
                self.internal_add_account(&account_id, project_id);
            }
        }
    }

    pub(crate) fn internal_add_account(&mut self, account_id: &AccountId, project_id: ProjectId){

        let mut projects_by_account = self.internal_get_projects_by_account_or_default(account_id);

        assert!(!projects_by_account.contains(&project_id),"Already register whitelist this project");
        projects_by_account.insert(&project_id);
        self.projects_by_account.insert(account_id, &projects_by_account);

        // Insert into accounts_by_project -> Use unwrap because of making sure that it has been inserted when project created.
        let mut accounts_in_project = self.accounts_by_project.get(&project_id).unwrap();
        accounts_in_project.insert(account_id, &ProjectAccount::default());
        self.accounts_by_project.insert(&project_id, &accounts_in_project);
    }

    // Project Distribution
    pub(crate) fn internal_distribute_token_to_users(&mut self, project_id: ProjectId) {
        // Get project account token sales
        let project = self.internal_get_project_or_panic(project_id);
        
    }

}

#[near_bindgen]
impl IDOContract{
        // Project Sale
        #[payable]
        pub(crate) fn internal_sale_commit(&mut self, project_id: ProjectId) {
        
            let account_id = env::signer_account_id();
            let project = self.internal_get_project_or_panic(project_id);                         
            let deposit = env::attached_deposit();

            project.assert_sale_period();
            assert!(self.is_whitelist(project_id,None),"Account do not register whitelisting this project");
              
            match project.sale_type {
                SaleType::Shared { 
                    min_allocation_per_user, 
                    max_allocation_per_user} => {
                        self.internal_sale_commit_shared_project(min_allocation_per_user, 
                                                                max_allocation_per_user,
                                                                project_id,
                                                                account_id,
                                                                deposit);
                    },
                SaleType::Lottery { 
                    allocation_per_ticket, 
                    total_tickets: _, 
                    win_ticket_ids: _ } => {
                        self.internal_sale_commit_lottery_project(allocation_per_ticket,
                                                                project_id,
                                                                account_id,
                                                                deposit);
                    }
            }
        }

        #[payable]
        pub fn internal_sale_commit_shared_project(&mut self,
            min_allocation: u128, 
            max_allocation: u128,
            project_id: ProjectId, 
            account_id: AccountId, 
            deposit: u128) {
                // Update Project struct
                let mut project_account_unordered_map = self.internal_get_accounts_by_project_or_panic(project_id);
                let mut project_account = self.internal_get_account_by_project_or_panic(project_id,&account_id);
                let mut project = self.internal_get_project_or_panic(project_id);
                let account_sale = project_account.sale_data.unwrap_or(
                    AccountSale{
                        committed_amount: 0,
                        sale_data: AccountSaleData::Shared
                    }
                );
                
                assert!( (account_sale.committed_amount + deposit) > min_allocation 
                    && (account_sale.committed_amount + deposit) < max_allocation,
                    "Total deposit amount must be between min_allocation and max_allocation");

                let account_sale = AccountSale{
                    committed_amount: account_sale.committed_amount + deposit,
                    sale_data: AccountSaleData::Shared
                };
                
                // Update Project account
                project_account = ProjectAccount{
                    sale_data: Some(account_sale),
                    distribution_data: None
                };

                // Update Project total_fund_committed
                project.total_fund_committed += deposit;

                // Insert new value into accounts_by_project
                project_account_unordered_map.insert(&account_id,&project_account);
                self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
                self.projects.insert(&project_id, &project);
                
        }

        #[payable]
        pub fn internal_sale_commit_lottery_project(&mut self,
            allocation_per_ticket: u128,
            project_id: ProjectId, 
            account_id: AccountId,
            deposit: u128){
                let mut project_account_unordered_map = self.internal_get_accounts_by_project_or_panic(project_id);
                let mut project_account = self.internal_get_account_by_project_or_panic(project_id,&account_id);
                let mut project = self.internal_get_project_or_panic(project_id);
                let account_sale = project_account.sale_data.unwrap();
                
                match account_sale.sale_data{ 
                    AccountSaleData::Lottery(
                        LotteryAccountSaleData{
                            eligible_tickets,
                            deposit_tickets,
                            ticket_ids,
                            win_ticket_ids
                        }) => {
                            let tickets_num = (deposit / allocation_per_ticket) as u128;
                            let mut ticket_ids = ticket_ids.clone();
                            assert!(tickets_num>0, "Must deposit at least {} for exchange a ticket",allocation_per_ticket);
                            assert!((tickets_num as u64 + deposit_tickets)<=eligible_tickets,"Eligible tickets not enough");

                            if deposit > (tickets_num * allocation_per_ticket){
                                //  Transfer back change deposit
                                Promise::new(account_id.clone()).transfer(deposit - tickets_num * allocation_per_ticket);
                            };

                            let project_total_tickets = (project.total_fund_committed / allocation_per_ticket) as u64;
                            for i in project_total_tickets..(project_total_tickets+ tickets_num as u64) {
                                ticket_ids.push(i);
                            }

                            let lottery_account_sale_data = AccountSaleData::Lottery(
                                LotteryAccountSaleData{
                                    eligible_tickets: eligible_tickets,
                                    deposit_tickets: (deposit_tickets + tickets_num as u64),
                                    ticket_ids: ticket_ids,
                                    win_ticket_ids: win_ticket_ids
                                }  
                            );

                            let account_sale = AccountSale{
                                committed_amount: account_sale.committed_amount + (tickets_num*allocation_per_ticket),
                                sale_data: lottery_account_sale_data
                            };
                            
                            // Update Project account
                            project_account = ProjectAccount{
                                sale_data: Some(account_sale),
                                distribution_data: None
                            };

                            // Update Project total_fund_committed
                            project.total_fund_committed += tickets_num*allocation_per_ticket;

                            // Insert new value into accounts_by_project
                            project_account_unordered_map.insert(&account_id,&project_account);
                            self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
                            self.projects.insert(&project_id, &project);
                        }
                    
                    _ => panic!("Invalid sale_data")
                }
        }
}