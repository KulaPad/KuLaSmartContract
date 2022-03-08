use crate::*;


#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub account_id: AccountId,
    pub ticket_type: TicketType,
    pub rank: TicketRank,
}

pub type TicketId = u64;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TicketType {
    Staking,
    Social,
    Referral,
}

/// There 2 type of ticket:
/// - VIP ticket: For staking Tier 4: This ticket can always join the campaign sale event
/// - normal ticket: This ticket need to pass lucky round to be chosen
/// Only the chosen normal ticket can join the sale round
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TicketRank {
    Normal,
    Vip,
}
