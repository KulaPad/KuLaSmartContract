use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountTickets {
    pub ticket_ids: Vec<TicketId>,
    pub win_ticket_ids: Vec<TicketId>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AccountTokenSales {
    // Deposit amount to buy project token
    pub funding_amount: Balance,
    pub token_unlocked_amount: Balance,
    pub token_locked_amount: Balance,
    pub token_withdrawal_amount: Balance,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfoJson {
    pub staking_tickets: u16,
    pub social_tickets: u16,
    pub referral_tickets: u16,
    pub win_tickets: u8,
}