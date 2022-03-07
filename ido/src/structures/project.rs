use crate::*;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum SaleType {
    FullUnlocked,
    Vested
}

#[derive(BorshSerialize, BorshDeserialize,Serialize, Deserialize,PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProjectStatus {
    Preparation,
    Whitelist,
    Sales,
    Distribution,
    Rejected
}


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

pub struct ProjectInfo {
    /// yourproject.near
    pub owner_id: AccountId,
    /// YOUR
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub introduction: String,
    pub categories: Vec<String>,
    pub whitelist_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    /// your.near
    pub token_contract_id: AccountId,
    /// YOUR
    pub token_symbol: String,
    /// 30.000.000
    pub token_raised_amount: Balance,
    /// 0.01 (NEAR)
    pub token_sale_rate: Balance,
    /// None
    pub fund_contract_id: Option<AccountId>,
    /// NEAR
    pub fund_symbol: String,

    pub sale_type: SaleType,
    pub configuration: ProjectConfiguration,
    pub current_ticket_id: TicketId,
    pub status: ProjectStatus,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectConfiguration {
    pub max_staking_tickets_per_user: u16,
    pub max_win_tickets_per_user: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonProject {
    pub id: ProjectId,
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub introduction: String,
    pub categories: Vec<String>,
    pub whitelist_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    pub token_symbol: String,
    pub token_raised_amount: Balance,
    pub token_sale_rate: Balance,
    pub fund_symbol: String,
    pub sale_type: SaleType,
    pub hard_cap: Balance,
    pub status: ProjectStatus,
    pub whitelist_accounts: u16,
    pub configuration: ProjectConfiguration,
}

#[near_bindgen]
impl IDOContract{

    

    pub fn create_project(&mut self, project_info: ProjectInfo) {
        // Get next Id
        let project_id = self.projects.len() + 1;
        

        // Insert the project
        self.projects.insert(&project_id, &project_info);

    }

    pub fn join_new_whitelist_project(&mut self,project_id:ProjectId, ticket_deposit: TicketsAmount) {

        // Require deposit 1 yoctoNEAR for user's private  
        assert_one_yocto_near();

        assert!(project_id <= self.projects.len(),"Project ID not exists");

        let mut project_info = self.projects.get(&project_id).unwrap();
        assert!(project_info.status == ProjectStatus::Whitelist,"Project is not whitelisted");

        let account_id = env::predecessor_account_id();

        let mut owner_tickets = self.staking_tickets_amount_per_owner_id.get(&account_id).unwrap_or(0);
        assert!(owner_tickets >= ticket_deposit,"Staking ticket of owner id must be greater than tickets amount deposit" );
        
        // Deposit some staking tickets when join project => Subtitute them with ticket_deposit
        owner_tickets -= ticket_deposit;
        self.staking_tickets_amount_per_owner_id.insert(&account_id,&owner_tickets);
        
        let mut projects_joined = self.account_projects.get(&account_id)
                                                        .unwrap_or_else(|| {
                                                            UnorderedSet::new(StorageKey::AccountProjectInnerKey{
                                                                account_id_hash: hash_account_id(&account_id)
                                            }.try_to_vec().unwrap())
                                                        });
        // Add project to owner
        projects_joined.insert(&project_id);

        
        self.account_projects.insert(&account_id,&projects_joined);

        for _ in 0..ticket_deposit{
            self.mint_staking_ticket(project_id,account_id.clone());
        };

        // Change current_ticket_id of project_info
        project_info.current_ticket_id += ticket_deposit;
        self.projects.insert(&project_id,&project_info);

        // TODO:Unit test
    }

    pub fn check_joined_whitelist(&self, account_id:AccountId, project_id: ProjectId)-> bool {
        let current_ticket_id = self.project_tickets.get(&project_id)
                                                                    .unwrap_or_else(||{
                                                                        panic!("No project found");
                                                                    }).len();
        for i in 0..current_ticket_id{
            let project_and_ticket_id = format!("{}.{}",project_id,i+1);
            let ticket = self.ticket_info.get(&project_and_ticket_id).unwrap();
            if ticket.account_id == account_id {
                return true;
            }
        }

        false

        // TODO: Unit test
    }

    pub fn get_project(&self, project_id: ProjectId) -> Option<JsonProject> {
        let project = self.projects.get(&project_id);

        if let Some(project) = project {
            Some(JsonProject {
                id: project_id,
                name: project.name,
                logo_url: project.logo_url,
                description: project.description,
                introduction: project.introduction,
                categories: project.categories,
                whitelist_date: project.whitelist_date,
                sale_start_date: project.sale_start_date,
                sale_end_date: project.sale_end_date,
                token_symbol: project.token_symbol,
                fund_symbol: project.fund_symbol,
                token_raised_amount: project.token_raised_amount,
                token_sale_rate: project.token_sale_rate,
                hard_cap: project.token_raised_amount * project.token_sale_rate,
                sale_type: project.sale_type,
                status: project.status,
                configuration: project.configuration,
                whitelist_accounts: 0,
            })
        } else {
            None
        }
    }

    pub fn get_projects(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonProject>{
        let projects_keys = self.projects.keys_as_vector();

        let start = from_index.unwrap_or(0);
        projects_keys.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|project_id| self.get_project(project_id).unwrap())
            .collect()


        
        // TODO: Unit test
    }

    pub fn get_projects_for_account(&self,account_id:AccountId, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonProject>{
        let account_projects = self.account_projects.get(&account_id);
        
        let projects = if let Some(account_projects) = account_projects{
            account_projects
        } else {
            return vec![];
        };

        let start = from_index.unwrap_or(0);

        projects.iter()
            .skip(start as usize)
            .take(limit.unwrap_or(0) as usize)
            .map(|project_id| self.get_project(project_id).unwrap())
            .collect()

        //  TODO: Unit test
    }
    

}