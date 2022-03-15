use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub enum SaleType {
    FullUnlocked,
    Vested
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ProjectStatus {
    Proposed,
    Approved,
    Rejected,
    Preparation,
    Whitelist,
    Sales,
    Distribution,
}

impl Default for ProjectStatus {
    fn default() -> Self { ProjectStatus::Preparation }
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
pub struct ProjectInfo {
    /// yourproject.near
    pub owner_id: AccountId,
    /// YOUR
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub introduction: String,
    pub categories: Vec<String>,
    pub whitelist_start_date: Timestamp,
    pub whitelist_end_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    /// your.near
    pub token_contract_id: AccountId,
    /// YOUR
    pub token_symbol: String,
    /// The amount of tokens to be sold in this campaign like: 30.000.000 TOKEN
    pub token_raised_amount: u64,
    /// The price of a token like: 0.01 (NEAR) / 1 TOKEN => numberator: 1, denominator: 100
    pub token_sale_rate: Rate,
    /// 8
    pub token_decimal: u8,
    /// The number of token that winner account can buy for one ticket like: 1.000 TOKEN / Ticket
    pub token_amount_per_sale_slot: u32,
    /// None
    pub fund_contract_id: Option<AccountId>,
    /// NEAR
    pub fund_symbol: String,
    /// The total fund that users deposited to buy token
    pub total_fund_received: Balance,
    pub sale_type: SaleType,
    pub configuration: ProjectConfiguration,
    pub status: ProjectStatus,
    /// Fixed allocations for account that is Tier 4
    pub total_allocations: TicketAmount,
    pub total_staking_tickets: TicketNumber,
    pub total_social_tickets: TicketNumber,
    pub total_referral_tickets: TicketNumber,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug)]
pub struct ProjectConfiguration {
    pub max_staking_tickets_per_user: u16,
    pub max_win_tickets_per_user: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectInfoJson {
    pub id: ProjectId,
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub introduction: String,
    pub categories: Vec<String>,
    pub whitelist_start_date: Timestamp,
    pub whitelist_end_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    pub token_symbol: String,
    pub token_raised_amount: u64,
    pub token_sale_rate: f64,
    pub token_decimal: u8,
    pub token_amount_per_sale_slot: u32,
    pub fund_symbol: String,
    pub total_fund_received: U128,
    pub sale_type: SaleType,
    pub hard_cap: u64,
    pub status: ProjectStatus,
    pub whitelist_accounts: u64,
    pub configuration: ProjectConfiguration,
    pub total_allocations: TicketAmount,
    pub total_staking_tickets: TicketNumber,
    pub total_social_tickets: TicketNumber,
    pub total_referral_tickets: TicketNumber,
}

impl ProjectInfo {
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

    pub(crate) fn get_total_sales_slots(&self) -> TicketAmount {
        (self.token_raised_amount / self.token_amount_per_sale_slot as u64) as u32
    }

    pub(crate) fn get_available_sales_slots(&self) -> TicketAmount {
        self.get_total_sales_slots() - self.total_allocations
    }

    pub(crate) fn get_sales_amount(&self, slots: u32) -> Balance {
        let amount_of_slot: u32 = self.token_amount_per_sale_slot;
        let unit_multiple: Balance = ONE_NEAR;

        // TODO: TBD
        // if self.fund_contract_id.is_some() {
        //     unit_multiple = self.fund_token_decimal;
        // }

        self.token_sale_rate.multiply(unit_multiple * amount_of_slot as u128 * slots as u128)
    }
}

#[near_bindgen]
impl IDOContract{
    pub fn create_project(&mut self, project_info: ProjectInfo) -> ProjectId{
        // Get next Id
        let project_id = self.projects.len() + 1;

        // Insert the project
        self.projects.insert(&project_id, &project_info);

        // Insert this project to related variables, this should be done by each status
        self.project_account_tickets.insert(&project_id, &UnorderedMap::new(get_storage_key(StorageKey::ProjectAccountTicketInnerKey(project_id)))); 
        self.project_account_token_sales.insert(&project_id, &UnorderedMap::new(get_storage_key(StorageKey::ProjectTokenSaleInnerKey(project_id)))); 
        self.project_tickets.insert(&project_id, &UnorderedMap::new(get_storage_key(StorageKey::ProjectTicketInnerKey(project_id)))); 
        
        project_id
    }

    pub fn get_project(&self, project_id: ProjectId) -> Option<ProjectInfoJson> {
        let project = self.projects.get(&project_id);

        self.internal_get_project(project_id, project)
    }
    
    pub fn get_projects(&self, status: Option<ProjectStatus>, from_index: Option<u64>, limit: Option<u64>) -> Vec<ProjectInfoJson>{
        self.projects
        .iter()
        .filter(|(_, project_info)| match &status { None => true, Some(s) => &project_info.status == s })
        .skip(from_index.unwrap_or(0) as usize)
        .take(limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize)
        .map(|(project_id, project_info)| self.internal_get_project(project_id.clone(), Some(project_info)).unwrap())
        .collect()
    }

    pub fn change_project_status(&mut self, project_id: ProjectId) {
        let mut project = self.get_project_or_panic(project_id);
        let current_time = get_current_time();
        match project.status {
            ProjectStatus::Preparation => {
                assert!(project.whitelist_start_date <= current_time && current_time <= project.whitelist_end_date, "Cannot change project's status to Whitelist");
                project.status = ProjectStatus::Whitelist;
            },
            ProjectStatus::Whitelist => {
                assert!(project.whitelist_end_date < current_time, "Cannot change project's status to Sale.");
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

    pub fn update_project_whitelist_date(&mut self, project_id: ProjectId, new_whitelist_start_date: Option<Timestamp>, new_whitelist_end_date: Option<Timestamp>) {
        assert_eq!(self.owner_id, env::signer_account_id(), "You are not allowed to update this project");

        let mut project_info = self.projects.get(&project_id).expect("No project found");

        let a_half_of_whitelist_period = (project_info.whitelist_end_date - project_info.whitelist_start_date) / 2;
        let current_time = env::block_timestamp();

        println!("update_project_whitelist_date: current_time: {}, a_half_of_period: {}", current_time, a_half_of_whitelist_period);

        project_info.whitelist_start_date = new_whitelist_start_date.unwrap_or(current_time - a_half_of_whitelist_period);
        project_info.whitelist_end_date = new_whitelist_end_date.unwrap_or(current_time + a_half_of_whitelist_period);

        self.projects.insert(&project_id, &project_info);

    }

    pub fn update_project_sales_date(&mut self, project_id: ProjectId) {
        assert_eq!(self.owner_id, env::signer_account_id(), "You are not allowed to update this project");

        let mut project_info = self.projects.get(&project_id).expect("No project found");

        let a_half_of_sales_period = (project_info.sale_end_date - project_info.sale_start_date) / 2;
        let current_time = env::block_timestamp();
        let time_to_change: i128 = current_time as i128 - project_info.sale_start_date as i128 - a_half_of_sales_period as i128;

        println!("Current time: {}", current_time);
        println!("Sales start: {}", project_info.sale_start_date);
        println!("Period: {}", a_half_of_sales_period);
        println!("To be change: {}", time_to_change);
        
        project_info.whitelist_start_date = (project_info.whitelist_start_date as i128 + time_to_change) as u64;
        project_info.whitelist_end_date = (project_info.whitelist_end_date as i128 + time_to_change) as u64;
        project_info.sale_start_date = (project_info.sale_start_date as i128 + time_to_change) as u64;
        project_info.sale_end_date = (project_info.sale_end_date as i128 + time_to_change) as u64;

        self.projects.insert(&project_id, &project_info);
    }

    pub fn update_project_sales_date_to_end(&mut self, project_id: ProjectId) {
        assert_eq!(self.owner_id, env::signer_account_id(), "You are not allowed to update this project");

        let mut project_info = self.projects.get(&project_id).expect("No project found");
        let current_timestamp = get_current_time();
        if project_info.sale_end_date <= current_timestamp {
            env::log("The sales end time is correct. No need to update.".as_bytes());
        }

        let time_period_after_sales_end = project_info.sale_end_date - current_timestamp + 1000;

        println!("Sales end date: {}, Current time: {}, Period: {}", project_info.sale_end_date, current_timestamp, time_period_after_sales_end);
        println!("Whitelist start date: {}", project_info.whitelist_start_date);
        println!("Whitelist end date: {}", project_info.whitelist_end_date);
        println!("Sales start date: {}", project_info.sale_start_date);
        println!("Sales end date: {}", project_info.sale_end_date);
        
        project_info.whitelist_start_date -= time_period_after_sales_end;
        project_info.whitelist_end_date -= time_period_after_sales_end;
        project_info.sale_start_date -= time_period_after_sales_end;
        project_info.sale_end_date -= time_period_after_sales_end;

        self.projects.insert(&project_id, &project_info);
    }

    pub fn update_project_status(&mut self, project_id: ProjectId, new_status: ProjectStatus) {
        assert_eq!(self.owner_id, env::signer_account_id(),"You are not allowed to update this project");

        let mut project_info = self.projects.get(&project_id).expect("No project found");
        project_info.status = new_status;

        self.projects.insert(&project_id, &project_info);
    }
}

impl IDOContract {
    pub(crate) fn get_project_or_panic(&self, project_id: ProjectId) -> ProjectInfo {
        self.projects.get(&project_id).expect("Project does not exist.")
    }

    pub(crate) fn get_project_account_ticket_or_panic(&self, project_id: ProjectId) -> AccountTicketsType {
        self.project_account_tickets.get(&project_id).expect("Project account tickets do not exist.")
    }

    pub(crate) fn get_project_account_token_sale_or_panic(&self, project_id: ProjectId) -> AccountTokenSalesType {
        self.project_account_token_sales.get(&project_id).expect("Project account token sales do not exist.")
    }

    pub(crate) fn get_project_ticket_or_panic(&self, project_id: ProjectId) -> ProjectTicketType {
        self.project_tickets.get(&project_id).expect("Project tickets do not exist.")
    }

    pub(crate) fn internal_get_project(&self, project_id: ProjectId, project_info: Option<ProjectInfo>) -> Option<ProjectInfoJson> {
        if let Some(project) = project_info {
            let whitelist_accounts = self.get_project_account_ticket_or_panic(project_id).len();

            Some(ProjectInfoJson {
                id: project_id,
                name: project.name,
                logo_url: project.logo_url,
                description: project.description,
                introduction: project.introduction,
                categories: project.categories,
                whitelist_start_date: project.whitelist_start_date,
                whitelist_end_date: project.whitelist_end_date,
                sale_start_date: project.sale_start_date,
                sale_end_date: project.sale_end_date,
                token_symbol: project.token_symbol,
                fund_symbol: project.fund_symbol,
                token_raised_amount: project.token_raised_amount,
                token_sale_rate: project.token_sale_rate.get_rate(),
                token_decimal: project.token_decimal,
                token_amount_per_sale_slot: project.token_amount_per_sale_slot,
                hard_cap: project.token_sale_rate.multiply(project.token_raised_amount as u128) as u64,
                sale_type: project.sale_type,
                total_fund_received: U128::from(project.total_fund_received),
                status: project.status,
                configuration: project.configuration,
                whitelist_accounts: whitelist_accounts,
                total_allocations: project.total_allocations,
                total_staking_tickets: project.total_staking_tickets,
                total_social_tickets: project.total_social_tickets,
                total_referral_tickets: project.total_referral_tickets,
            })
        } else {
            None
        }
    }

    pub(crate) fn internal_has_project(&self, project_id: ProjectId) -> bool{
        self.projects.get(&project_id).is_some()
    }
    
    pub(crate) fn internal_distribute_token_to_users(&mut self, project_id: ProjectId) {
        // Get project account token sales
        let project = self.get_project_or_panic(project_id);
        let accounts_for_token_sales = self.get_project_account_token_sale_or_panic(project_id);

        for (account_id, token_sales) in &accounts_for_token_sales.to_vec() {
            // Deposit near amount
            // Token price
            // Token amount
        }
        // Update token unlocked amount


    }
}