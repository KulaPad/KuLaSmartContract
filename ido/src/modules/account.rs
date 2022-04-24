use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct LotteryAccountSaleData {
    /// The number of eligible ticket. Ex: 10
    eligible_tickets: TicketNumber,
    /// The number of ticket that user commits to join. Ex 1 -> 10
    deposit_tickets: TicketNumber,
    /// When user deposit fund to get the deposit ticket, the list of ticket would be generated based on the deposit order of the user.
    ticket_ids: Vec<TicketNumber>,
    /// The list of win tickets
    win_ticket_ids: Vec<TicketNumber>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct AccountSaleJson {
    pub committed_amount: U128,
    pub lottery_sale_data: LotteryAccountSaleData,
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

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct ProjectAccount {
    sale_data: Option<AccountSale>,
    distribution_data: Option<AccountDistribution>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectAccountJson {
    account_id: AccountId,
    sale_data: Option<AccountSaleJson>,
    distribution_data: Option<AccountDistributionJson>,
}