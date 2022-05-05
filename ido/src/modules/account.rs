use crate::*;
use crate::tests::test_project::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub struct LotteryAccountSaleData {
    /// The number of eligible ticket. Ex: 10
    pub eligible_tickets: TicketNumber,
    /// The number of ticket that user commits to join. Ex 1 -> 10
    pub deposit_tickets: TicketNumber,
    /// When user deposit fund to get the deposit ticket, the list of ticket would be generated based on the deposit order of the user.
    pub ticket_ids: Vec<TicketNumber>,
    /// The list of win tickets
    pub win_ticket_ids: Vec<TicketNumber>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq)]
pub enum AccountSaleData {
    Shared,
    Lottery(LotteryAccountSaleData),
}

impl Default for AccountSaleData {
    fn default() -> Self {
        AccountSaleData::Shared
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct AccountSale {
    pub committed_amount: Balance,
    pub sale_data: AccountSaleData,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct AccountSaleJson {
    pub committed_amount: U128,
    // This property is used for AccountSaleData::Lottery only.
    pub lottery_sale_data: Option<LotteryAccountSaleData>,
}

impl AccountSaleJson {
    pub fn from(account_sale: AccountSale) -> Self {
        Self {
            committed_amount: U128::from(account_sale.committed_amount),
            lottery_sale_data: match account_sale.sale_data {
                AccountSaleData::Shared => None,
                AccountSaleData::Lottery(data) => Some(data),
            },
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct AccountDistribution {
    pub unlocked_amount: Balance,
    pub locked_amount: Balance,
    pub claimed_amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct AccountDistributionJson {
    pub unlocked_amount: U128,
    pub locked_amount: U128,
    pub claimed_amount: U128,
}

impl AccountDistributionJson {
    fn from(account_distribution: AccountDistribution) -> Self {
        Self {
            unlocked_amount: U128::from(account_distribution.unlocked_amount),
            locked_amount: U128::from(account_distribution.locked_amount),
            claimed_amount: U128::from(account_distribution.claimed_amount),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct ProjectAccount {
    pub sale_data: Option<AccountSale>,
    pub distribution_data: Option<AccountDistribution>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectAccountJson {
    pub project_id: ProjectId,
    pub account_id: AccountId,
    pub is_whitelist: bool,
    pub sale_data: Option<AccountSaleJson>,
    pub distribution_data: Option<AccountDistributionJson>,
}

impl IDOContract {
    pub(crate) fn internal_get_project_account_info(&self, project_id: ProjectId, account_id: AccountId) -> ProjectAccountJson {
        let mut account_json = ProjectAccountJson {
            project_id,
            account_id: account_id.clone(),
            is_whitelist: false,
            sale_data: None,
            distribution_data: None,
        };
        
        // Get project by id
        let project = self.internal_get_project_or_panic(project_id);

        if project.status == ProjectStatus::Preparation {
            return account_json;
        }

        // If the user account doesn't exist in the project that means the user has not registered yet.
        let account = self.internal_get_account_by_project(project_id, &account_id);
        if account.is_none() {
            return account_json;
        }

        if let Some(account) = account {
            account_json.is_whitelist = true;
            
            if let Some(sale_data) = account.sale_data {
                account_json.sale_data = Some(AccountSaleJson::from(sale_data));
            }

            if let Some(distribution_data) = account.distribution_data {
                account_json.distribution_data = Some(AccountDistributionJson::from(distribution_data));
            }
        }

        account_json
    }
}