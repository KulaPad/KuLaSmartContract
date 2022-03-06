use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum TicketType {
    Staking,
    Social,
    Referral,
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct Ticket {
    pub ticket_type: TicketType,
    pub account_id: AccountId,
}