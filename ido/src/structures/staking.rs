/*
This file is temporary
It should be on staking module instead
 */
use crate::*;

#[derive(BorshSerialize, BorshDeserialize,)]
pub struct UserStakingInfo {
    pub tier: StakingTier,
    pub staked: Balance,
    pub un_staked: Balance,
    pub staked_at: Timestamp,
    pub lock_day_count: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Debug)]
pub enum StakingTier {
    Tier0,
    Tier1,
    Tier2,
    Tier3,
    Tier4,
}

impl Default for StakingTier {
    fn default() -> Self { StakingTier::Tier0 }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TierInfo {
    pub locked_amount: u64,
    pub no_of_tickets: UnorderedMap<u16, u16>,
    pub no_of_allocations: UnorderedMap<u16, u8>,
}

impl TierInfo {
    fn new(tier: StakingTier, locked_amount: u64) -> Self {
        env::log(format!("Creating tier... -> Tier: {:?}", tier).as_bytes());

        Self {
            locked_amount,
            no_of_tickets: UnorderedMap::new(get_storage_key(StorageKey::TierTicketInnerKey(format!("{:?}", tier)))),
            no_of_allocations: UnorderedMap::new(get_storage_key(StorageKey::TierAllocationInnerKey(format!("{:?}", tier)))),
        }
    }
}

pub(crate) fn initialize_tiers(token_decimal: u8) -> UnorderedMap<StakingTier, TierInfo> {
    env::log(format!("Initializing tiers...").as_bytes());

    let mut tiers = UnorderedMap::new(get_storage_key(StorageKey::TierKey));
    
    let mut tier1 = TierInfo::new(StakingTier::Tier1, 200 * token_decimal as u64);
    let mut tier2 = TierInfo::new(StakingTier::Tier2, 1000 * token_decimal as u64);
    let mut tier3 = TierInfo::new(StakingTier::Tier3, 5000 * token_decimal as u64);
    let mut tier4 = TierInfo::new(StakingTier::Tier4, 10000 * token_decimal as u64);

    env::log(format!("Initializing tiers... -> Tiers created.").as_bytes());

    tier1.no_of_tickets.insert(&7, &1);
    tier1.no_of_tickets.insert(&14, &2);
    tier1.no_of_tickets.insert(&30, &4);
    tier1.no_of_tickets.insert(&90, &8);
    tier1.no_of_tickets.insert(&180, &12);
    tier1.no_of_tickets.insert(&365, &20);

    env::log(format!("Initializing tiers... -> Tier1 inserted.").as_bytes());

    tier2.no_of_tickets.insert(&7, &6);
    tier2.no_of_tickets.insert(&14, &12);
    tier2.no_of_tickets.insert(&30, &24);
    tier2.no_of_tickets.insert(&90, &48);
    tier2.no_of_tickets.insert(&180, &72);
    tier2.no_of_tickets.insert(&365, &120);

    tier3.no_of_tickets.insert(&7, &35);
    tier3.no_of_tickets.insert(&14, &70);
    tier3.no_of_tickets.insert(&30, &140);
    tier3.no_of_tickets.insert(&90, &280);
    tier3.no_of_tickets.insert(&180, &420);
    tier3.no_of_tickets.insert(&365, &700);

    tier4.no_of_tickets.insert(&7, &35);
    tier4.no_of_tickets.insert(&14, &70);
    tier4.no_of_tickets.insert(&30, &140);
    tier4.no_of_tickets.insert(&90, &280);
    tier4.no_of_tickets.insert(&180, &420);
    tier4.no_of_tickets.insert(&365, &700);

    tier4.no_of_allocations.insert(&7, &1);
    tier4.no_of_allocations.insert(&30, &2);
    tier4.no_of_allocations.insert(&180, &3);
    
    env::log(format!("Initializing tiers... -> Tiers created").as_bytes());

    tiers.insert(&StakingTier::Tier1, &tier1);
    tiers.insert(&StakingTier::Tier2, &tier2);
    tiers.insert(&StakingTier::Tier3, &tier3);
    tiers.insert(&StakingTier::Tier4, &tier4);

    env::log(format!("Tiers initialized.").as_bytes());

    tiers
}

#[derive(Serialize, Deserialize)]
/// Stores the information of an account
/// The tier is from Tier1 to Tier4, if the locked amount is less than Tier1, it would be Tier0
/// The no_of_staking_tickets is the number of staking tickets of current tier with a number of locked days.
/// The no_of_allocations is user for Tier4 only. This is the number of allocation that users can have a ido buy slot.
pub struct TierInfoJson {
    pub tier: StakingTier,
    pub locked_amount: U64,
    pub locked_days: u32,
    pub calculating_time: Timestamp,
    pub no_of_staking_tickets: TicketAmount,
    pub no_of_allocations: TicketAmount,
}

impl Default for TierInfoJson {
    fn default() -> Self {
        Self {
            tier: StakingTier::default(),
            locked_amount: U64::from(0),
            locked_days: 0,
            calculating_time: 0,
            no_of_staking_tickets: 0,
            no_of_allocations: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectWhitelistInfo {
    pub tier: StakingTier,
    pub no_of_staking_tickets: TicketAmount,
    pub no_of_social_tickets: TicketAmount,
    pub no_of_referral_tickets: TicketAmount,
    pub no_of_allocations: TicketAmount,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectDistributionInfo {
    
}

/// Preparation                 -> AccountId, ProjectId, ProjectStatus
/// Whitelist (+ Registered)    -> [Fields in Preparation] + Staking Tier, tickets (staking, social, referral), allocations
/// Sales                       -> [Fields in Whitelist] + 
/// Distribution                -> Ticket + Fund + Token
/// Other statuses              -> Panic
#[derive(Serialize, Deserialize, Default)]
pub struct ProjectAccountInfoJson {
    pub account_id: AccountId,
    pub project_id: ProjectId,
    pub project_status: ProjectStatus,
    pub whitelist_info: Option<ProjectWhitelistInfo>,
    pub sale_info: Option<JsonAccountTokenSales>,
}

impl ProjectAccountInfoJson {
    pub fn new(account_id: AccountId, project_id: ProjectId, project_status: ProjectStatus) -> Self {
        Self {
            account_id,
            project_id,
            project_status,
            whitelist_info: None,
            sale_info: None,
        }
    }
}

#[near_bindgen]
impl IDOContract {
    pub(crate) fn internal_get_staking_tier_info(&self, locked_amount: u64, locked_timestamp: Timestamp, calculating_timestamp: Option<Timestamp>) -> TierInfoJson {
        let calculating_timestamp = calculating_timestamp.unwrap_or(env::block_timestamp());
        
        // Step 1: Use locked_amount to identify staking tier
        // IDOContract.tiers is stored TierConfiguration
        // Refer the initializing function initialize_tiers
        // Or the sheet https://docs.google.com/spreadsheets/d/1XWL2vtGIX89kGgj6M-X-ocCrQfz05fm9n4HncDrSuSU/edit#gid=778618928

        let tier = match locked_amount {
            0...20000000000 => {
                StakingTier::Tier0
            },
            20000000000...100000000000 => {
                StakingTier::Tier1

            },
            100000000000...500000000000 => {
                StakingTier::Tier2
            },
            500000000000...1000000000000 => {
                StakingTier::Tier3
            },
            _ => {
                StakingTier::Tier4
            },
        };

        // If the locked amount is less than Tier1.locked_amount (TierInfo), return the default of TierInfoJson with Tier0.

        // Step 2: Calculating the number of day between calculating_timestamp (Project.whitelist_start_date) and locked_timestamp.

        let locked_days: u32 =((locked_timestamp - calculating_timestamp) / 84600000000000) as u32;
        let day =locked_days as u16;

        // Step 3: Using calculating day (Ex: 30 days) to identify the number of staking tickets & the number of allocation (For Tier4 only)
        let tier_info = self.tiers.get(&tier).unwrap();
        let no_of_staking_tickets = tier_info.no_of_tickets.get(&day).unwrap() as u32;
        let no_of_allocations = tier_info.no_of_allocations.get(&day).unwrap() as u32;

        // Step 4: Return data
        // tier: StakingTier,
        // locked_amount: U64,
        // locked_days: u32,
        // calculating_time: Timestamp,
        // no_of_staking_tickets: TicketAmount,
        // no_of_allocations: TicketAmount,
        TierInfoJson {
            tier,
            locked_amount: U64::from(locked_amount),
            locked_days,
            calculating_time: calculating_timestamp,
            no_of_staking_tickets,
            no_of_allocations,
        }
    }

    pub(crate) fn internal_get_project_staking_tier_info(&self, project_id: ProjectId, account_id: AccountId) -> ProjectAccountInfoJson {
        // Validating
        // Project must be existed
        let project = &self.get_project_or_panic(project_id);
        
        let mut result = ProjectAccountInfoJson::new(account_id.clone(), project_id, project.status.clone());


        // Project's status is in Whitelist, Sale, Distribution
        
        match project.status{

            // Status: Whitelist -> User must be registered whitelist
            ProjectStatus:: Whitelist =>{    
                    // Ticket information of this account
                    let ticket_info = self.unwrap_project_account_ticket(project_id,&account_id);

                    // ProjectWhitelistInfo
                    //     tier: StakingTier,
                    //     no_of_staking_tickets: TicketAmount,
                    //     no_of_social_tickets: TicketAmount,
                    //     no_of_referral_tickets: TicketAmount,
                    //     no_of_allocations: TicketAmount,
                    let mut whitelist_info = ProjectWhitelistInfo::default();

                    // Get from self.project_account_tickets. Project -> Account -> Tickets
                    // Tickets: staking_tier, staking_tickets.eligible_tickets, allocations, social_tickets.eligible_tickets, referral_tickets.eligible_tickets

                    whitelist_info.tier = ticket_info.staking_tier;
                    whitelist_info.no_of_staking_tickets = ticket_info.staking_tickets.eligible_tickets;
                    whitelist_info.no_of_social_tickets = ticket_info.social_tickets.eligible_tickets;
                    whitelist_info.no_of_referral_tickets = ticket_info.referral_tickets.eligible_tickets;
                    whitelist_info.no_of_allocations = ticket_info.allocations;

                    result.whitelist_info = Some(whitelist_info);
                    result.sale_info = None;
            },

            // Status: Sales
            // JsonAccountTokenSales {
            //     funding_amount: U128,
            //     token_unlocked_amount: U128,
            //     token_locked_amount: U128,
            //     token_withdrawal_amount: U128,
            ProjectStatus:: Sales =>
            {    
                    // Get from self.project_account_token_sales. Project -> Account -> AccountTokenSales
                    // Token Sales: funding_amount, token_unlocked_amount, allocations, token_locked_amount, token_withdrawal_amount
                    
                    let account_token_sales = self.unwrap_project_account_token_sales(project_id);
                    let sale_info = account_token_sales.get(&account_id).expect("Account id are allow buy token");


                    result.whitelist_info = None; 
                    result.sale_info = Some(
                        JsonAccountTokenSales{
                            funding_amount: U128(sale_info.funding_amount),
                            token_unlocked_amount: U128(sale_info.token_unlocked_amount),
                            token_locked_amount: U128(sale_info.token_locked_amount),
                            token_withdrawal_amount:U128(sale_info.token_withdrawal_amount),
                        }
                    )
            },
        

            // Status: Distribution
            ProjectStatus::Distribution =>
            {

            } ,

            _ => 
            {
                
            }


        }


        result.project_status = project.status.clone();
        // Return data
        result
    }
}
