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

#[derive(Serialize, Deserialize,Debug)]
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
    
    pub fn internal_register_whitelist(&mut self, account_id: AccountId,project_id: ProjectId) {
        let project = self.internal_get_project_or_panic(project_id);                       
        assert_eq!(project.status, ProjectStatus::Whitelist,"Project isn't on whitelist");
        assert!(project.is_in_whitelist_period(), "Project isn't on whitelist time");
        

        match project.whitelist_type {
            WhitelistType::None | WhitelistType::Ticket  =>{ 
                self.internal_add_account(&account_id, project_id);
            },
            WhitelistType::XToken(xtoken) => {
                self.internal_register_whitelist_fixed_xtoken_project(project_id, account_id, xtoken);
            },
        };
    }

    // Project Sale
    pub(crate) fn internal_commit(&mut self, project_id: ProjectId, account_id: &AccountId, amount: Balance) -> Balance{
        
        let project = self.internal_get_project_or_panic(project_id);                         
                
        assert!(self.is_whitelist(project_id,account_id.to_string()),"Account does not register whitelisting this project");
        project.assert_sale_period();
                  
        match project.sale_type {
            SaleType::Shared { 
                min_allocation_per_user, 
                max_allocation_per_user
            } => {
                    self.internal_commit_shared_project(min_allocation_per_user, 
                                                                    max_allocation_per_user,
                                                                    project_id,
                                                                    account_id,
                                                                    amount)
                },
            SaleType::Lottery { 
                allocation_per_ticket, 
                total_tickets, 
                win_ticket_ids } => {
                    self.internal_commit_lottery_project(allocation_per_ticket,
                                                            total_tickets,
                                                            win_ticket_ids,
                                                            project_id,
                                                            account_id,
                                                            amount,
                                                            )
                }
        }
    }
    
    pub(crate) fn internal_commit_shared_project(&mut self,
        min_allocation: u128, 
        max_allocation: u128,
        project_id: ProjectId, 
        account_id: &AccountId, 
        deposit: u128)-> Balance {
            // Update Project struct
            let mut project_account_unordered_map = self.internal_get_accounts_by_project_or_panic(project_id);
            let mut project_account = self.internal_get_account_by_project_or_panic(project_id,&account_id);
            let mut project = self.internal_get_project_or_panic(project_id);
            let mut account_sale = project_account.sale_data.unwrap_or(
                AccountSale{
                    committed_amount: 0,
                    sale_data: AccountSaleData::Shared
                }
            );
                    
            assert!( (account_sale.committed_amount + deposit) >= min_allocation 
                && (account_sale.committed_amount + deposit) <= max_allocation,
                "Total deposit amount must be between min_allocation and max_allocation");
    
            account_sale = AccountSale{
                committed_amount: account_sale.committed_amount + deposit,
                sale_data: AccountSaleData::Shared
            };
                    
            // Update Project account
            project_account.sale_data = Some(account_sale);
    
            // Update Project total_fund_committed
            project.total_fund_committed += deposit;
    
            // Insert new value into accounts_by_project
            project_account_unordered_map.insert(&account_id,&project_account);
            self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
            self.projects.insert(&project_id, &project);
                    
            deposit              
    }
    
    
    pub(crate) fn internal_commit_lottery_project(&mut self,
        allocation_per_ticket: u128,
        total_tickets: u64,
        project_ticket_win_ids: Option<Vec<u64>>,
        project_id: ProjectId, 
        account_id: &AccountId,
        deposit: u128,
    ) -> Balance{
            let mut tickets_by_project = self.internal_get_tickets_by_project_or_panic(project_id);
            let mut project_account_unordered_map = self.internal_get_accounts_by_project_or_panic(project_id);
            let mut project_account = self.internal_get_account_by_project_or_panic(project_id,&account_id);
            let mut project = self.internal_get_project_or_panic(project_id);
            let account_sale = project_account.sale_data.unwrap_or( AccountSale{
                committed_amount: 0,
                sale_data: AccountSaleData::Lottery(
                    LotteryAccountSaleData{
                        eligible_tickets: 0,
                        deposit_tickets: 0,
                        ticket_ids: vec![],
                        win_ticket_ids: vec![]
                    })
            });
                    
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
    
                        for i in total_tickets..(total_tickets+ tickets_num as u64) {
                            ticket_ids.push(i);
                            tickets_by_project.insert(&i,&account_id);
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
                                
                        // Update Project account sale_data
                        project_account.sale_data = Some(account_sale);
                        // Update Project total_fund_committed
                        project.total_fund_committed += tickets_num*allocation_per_ticket;
                        // Update project total_tickets
                        project.sale_type = SaleType::Lottery { 
                            allocation_per_ticket: allocation_per_ticket, 
                            total_tickets: total_tickets + tickets_num as u64, 
                            win_ticket_ids: project_ticket_win_ids
                        };
    
                        // Insert new value into accounts_by_project and tickets_by_project
                        project_account_unordered_map.insert(&account_id,&project_account);
                        self.accounts_by_project.insert(&project_id,&project_account_unordered_map);
                        self.tickets_by_project.insert(&project_id,&tickets_by_project);
                        self.projects.insert(&project_id, &project);
    
                        (tickets_num * allocation_per_ticket) as u128
                    }
                        
                _ => panic!("Invalid sale_data")
            }
    }

    // Project Distribution
    pub(crate) fn internal_distribute_token_to_users(&mut self, project_id: ProjectId) {
        // Get project account token sales
        let project = self.internal_get_project_or_panic(project_id);
        let project_sale_type = &project.sale_type;
        //  Traverse all account_id in ProjectAccountUnorderedMap
        let accounts_by_project_1 = self.internal_get_accounts_by_project_or_panic(project_id);
        let mut accounts_by_project_2 = self.internal_get_accounts_by_project_or_panic(project_id);
        match project_sale_type {
            SaleType::Shared { 
                min_allocation_per_user: _,
                max_allocation_per_user: _ 
            } => {             
                for (account_id, project_account) in accounts_by_project_1.iter(){
                    
                    let account_sale = project_account.sale_data.unwrap_or(
                        AccountSale{
                        committed_amount: 0,
                        sale_data: AccountSaleData::Shared
                    });   
                        
                    if account_sale.committed_amount != 0 {
                        //  Create AccountDistribution data for all account
                        let account_distribution = AccountDistribution {
                            unlocked_amount: (account_sale.committed_amount / project.total_fund_committed) * project.token_raised_amount,
                            locked_amount: 0,
                            claimed_amount: 0
                        };
                        let new_project_account = ProjectAccount {
                            sale_data: Some(account_sale),
                            distribution_data: Some(account_distribution)
                        };
                        accounts_by_project_2.insert(&account_id, &new_project_account);
                    }
                }
            },

            SaleType::Lottery { 
                allocation_per_ticket, 
                total_tickets, 
                win_ticket_ids
            } =>{
                let tickets_by_project = self.internal_get_tickets_by_project_or_panic(project_id);
                // TODO: Random win_ticket and save to win_ticket_ids
                // Blocktimestamp of near is nanoseconds so 4 last numbers of timestamp will be randomly
                // Ex: if the 4 last numbers is 1234. We do 123 * 234 * 1234 = 35516988. 
                // So we have 35, 51, 69, 88 is 4 numbers of last 2digit in user ticket.
                // Check through all user ticket by tickets_by_project.
                // Ex: If total tickets is 12055. 
                //     But there're only 1_000_000(token_raised_amount) / 1_000 (allocation_per_ticket) = 1000 win tickets
                // 
                for (account_id, project_account) in accounts_by_project_1.iter(){
                    //  Create AccountDistribution data for all account
                    let account_sale = project_account.sale_data.unwrap_or(
                        AccountSale{
                        committed_amount: 0,
                        sale_data: AccountSaleData::Lottery(
                            LotteryAccountSaleData{
                                eligible_tickets: 0,
                                deposit_tickets: 0,
                                ticket_ids: vec![],
                                win_ticket_ids: vec![]
                            })
                    });

                    if account_sale.committed_amount != 0 {
                        //  TODO: Create AccountDistribution data for all account
                    }
                }

                self.projects.insert(&project_id, &project);
            }
        }
        
        //  Insert into accounts_by_project
        self.accounts_by_project.insert(&project_id, &accounts_by_project_2);
    }

}