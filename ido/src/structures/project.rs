
use crate::*;
use crate::structures::account::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum DistributionType {
    FullUnlocked,
    Vested
}

#[derive(Serialize, Deserialize)]
pub enum ProjectStatus {
    Preparation,
    Whitelist,
    Sales,
    Distribution,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub owner_id: AccountId,
    pub name: String,
    pub logo_url: String,
    pub description: String,
    pub introduction: String,
    pub whitelist_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    pub token_symbol: String,
    pub token_contract_id: AccountId,
    pub fund_symbol: String,
    pub fund_contract_id: Option<AccountId>,
    pub token_raised_amount: Balance,
    pub fund_raised_amount: Balance,
    pub token_price: Balance,
    pub distribution_type: DistributionType,
    pub configuration: ProjectConfiguration,
    pub current_ticket_id: TicketId,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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
    pub whitelist_date: Timestamp,
    pub sale_start_date: Timestamp,
    pub sale_end_date: Timestamp,
    pub token_symbol: String,
    pub fund_symbol: String,
    pub fund_contract_id: Option<AccountId>,
    pub token_raised_amount: Balance,
    pub fund_raised_amount: Balance,
    pub token_price: Balance,
    pub distribution_type: DistributionType,
    pub configuration: ProjectConfiguration,
    pub whitelist_accounts: u16,
}

#[near_bindgen]
impl IDOContract{
    pub fn create_project(&mut self, project_info: ProjectInfo) {
        // Get next Id
        let project_id = self.projects.len() + 1;

        // Insert the project
        self.projects.insert(&project_id, &project_info);
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
                whitelist_date: project.whitelist_date,
                sale_start_date: project.sale_start_date,
                sale_end_date: project.sale_end_date,
                token_symbol: project.token_symbol,
                fund_symbol: project.fund_symbol,
                fund_contract_id: project.fund_contract_id,
                token_raised_amount: project.token_raised_amount,
                fund_raised_amount: project.fund_raised_amount,
                token_price: project.token_price,
                distribution_type: project.distribution_type,
                configuration: project.configuration,
                whitelist_accounts: 0,
            })
        } else {
            None
        }
    }

    pub fn get_projects(&self, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonProject>{
        // TODO: Do paging

        self.projects
        .iter()
        .map(|(project_id, project_info)| self.get_project(project_id.clone()).unwrap())
        .collect()
    }
}