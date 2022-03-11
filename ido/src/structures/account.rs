use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct AccountTicket {
    /// The number of eligible ticket. Ex: 10
    pub eligible_tickets: TicketAmount,
    /// The number of ticket that user commits to join. Ex 1 -> 10
    pub deposit_tickets: TicketAmount,
    /// When user deposit fund to get the deposit ticket, the list of ticket would be generated based on the deposit order of the user.
    pub ticket_ids: Vec<TicketNumber>,
    /// The list of win tickets
    pub win_ticket_ids: Vec<TicketNumber>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Default)]
pub struct AccountTickets {
    pub staking_tier: StakingTier,
    pub staking_tickets: AccountTicket,
    pub social_tickets: AccountTicket,
    pub referral_tickets: AccountTicket,
    pub allocations: TicketAmount,
    pub deposit_allocations: TicketAmount,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct AccountTokenSales {
    pub funding_amount: Balance,
    pub token_unlocked_amount: Balance,
    pub token_locked_amount: Balance,
    pub token_withdrawal_amount: Balance,
}

#[derive(Serialize, Deserialize, Default)]
pub struct JsonAccountTicketInfo {
    pub account_id: AccountId,
    pub staking_tickets: u16,
    pub social_tickets: u16,
    pub referral_tickets: u16,
    pub win_tickets: u8,
}

impl JsonAccountTicketInfo {
    pub fn new(account_id: AccountId) -> Self {
        let mut account_ticket_info = Self::default();
        account_ticket_info.account_id = account_id;
        account_ticket_info   
    }
}

#[derive(Serialize, Deserialize,PartialEq, Debug)]
pub struct JsonAccountTokenSales {
    pub funding_amount: U128,
    pub token_unlocked_amount: U128,
    pub token_locked_amount: U128,
    pub token_withdrawal_amount: U128,
}
