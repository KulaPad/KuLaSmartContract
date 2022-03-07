use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum TicketType {
    Staking,
    Social,
    Referral,
}

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct Ticket {
    
    pub account_id: AccountId,
    
    pub ticket_type: TicketType,
}

impl IDOContract{

    pub fn mint_staking_ticket(&mut self, project_id:ProjectId,account_id:AccountId){
        let ticket = Ticket {
            account_id: account_id,
            ticket_type: TicketType::Staking,
        };

        let mut project_tickets = self.project_tickets.get(&project_id).unwrap_or_else(||{
                                                        UnorderedSet::new(StorageKey::ProjectTicketInnerKey{
                                                            project_id_hash: hash_project_id(project_id)                  
                                                }.try_to_vec().unwrap())
                                            });
        
        let ticket_id = project_tickets.len() +1;

        project_tickets.insert(&ticket_id);
        self.project_tickets.insert(&project_id,&project_tickets);

        let project_and_ticket_id = format!("{}_{}",&project_id,&ticket_id);
        self.ticket_info.insert(&project_and_ticket_id,&ticket);
    }
}