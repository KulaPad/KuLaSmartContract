use crate::*;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum SaleType {
    FullUnlocked,
    Vested
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
pub enum ProjectStatus {
    New,
    Approved,
    Rejected,
    Preparation,
    Whitelist,
    Sales,
    Distribution,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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
impl ProjectInfo {

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

        self.internal_get_project(project_id, project)
    }

    pub fn get_projects(&self, status: Option<ProjectStatus>, from_index: Option<u64>, limit: Option<u64>) -> Vec<JsonProject>{
        self.projects
        .iter()
        .filter(|(project_id, project_info)| match &status { None => true, Some(s) => &project_info.status == s })
        .skip(from_index.unwrap_or(0) as usize)
        .take(limit.unwrap_or(DEFAULT_PAGE_SIZE) as usize)
        .map(|(project_id, project_info)| self.internal_get_project(project_id.clone(), Some(project_info)).unwrap())
        .collect()
    }

    pub(crate) fn internal_get_project(&self, project_id: ProjectId, project_info: Option<ProjectInfo>) -> Option<JsonProject> {
        if let Some(project) = project_info {
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
}