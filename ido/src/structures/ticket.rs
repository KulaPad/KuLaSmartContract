use crate::*;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub account_id: AccountId,
    pub ticket_type: TicketType,
}

pub type TicketId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TicketType {
    Staking,
    Social,
    Referral,
}