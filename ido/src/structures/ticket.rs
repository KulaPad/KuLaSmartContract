use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub enum TicketType {
    Staking,
    Social,
    Referral,
}

pub(crate) fn build_ticket_id(ticket_type: TicketType, ticket_numer: TicketNumber) -> String {
    let prefix = match ticket_type {
        TicketType::Staking => "L",
        TicketType::Social => "S",
        TicketType::Referral => "R",
    };
    format!("{}{}", prefix, ticket_numer)
}