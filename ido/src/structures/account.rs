use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountTickets {
    pub staking_ticket_ids: Vec<TicketId>,
    pub social_ticket_ids: Vec<TicketId>,
    pub referral_ticket_ids: Vec<TicketId>,
    pub win_ticket_ids: Vec<TicketId>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountTokenSales {
    pub funding_amount: Balance,
    pub token_unlocked_amount: Balance,
    pub token_locked_amount: Balance,
    pub token_withdrawal_amount: Balance,
}

#[derive(Serialize, Deserialize)]
pub struct JsonAccountTicketInfo {
    pub staking_tickets: u16,
    pub social_tickets: u16,
    pub referral_tickets: u16,
    pub win_tickets: u8,
}
