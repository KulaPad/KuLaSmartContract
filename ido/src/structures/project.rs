use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,PartialEq)]
pub enum SaleType {
    FullUnlocked,
    Vested
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug,PartialEq)]
pub enum ProjectStatus {
    Proposed,
    Approved,
    Rejected,
    Preparation,
    Whitelist,
    Sales,
    Distribution,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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

    pub(crate) fn get_rate(&self) -> f64 {
        self.numberator as f64 / self.denominator as f64
    }
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
    pub fund_symbol: String,
    pub sale_type: SaleType,
    pub hard_cap: u64,
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

    pub(crate) fn internal_get_project(&self, project_id: ProjectId, project_info: Option<ProjectInfo>) -> Option<ProjectInfoJson> {
        if let Some(project) = project_info {
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
                hard_cap: project.token_sale_rate.multiply(project.token_raised_amount as u128) as u64,
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