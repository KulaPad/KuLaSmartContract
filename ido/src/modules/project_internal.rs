use crate::*;

#[near_bindgen]
impl IDOContract{

    pub(crate) fn internal_new_project_1()-> ProjectInput{
        ProjectInput{
            owner_id: "alevoro.near".to_string(),
            whitelist_start_date: 1651172401000000000,
            whitelist_end_date: 1651604401000000000,
            sale_start_date: 1651604401000000000,
            sale_end_date: 1652068717000000000,
            token_contract_id: "contract.alevoro.near".to_string(),
            token_raised_amount: U128(1000000000000000),
            token_sale_rate_numberator: 10u64,
            token_sale_rate_denominator: 1u64,
            fund_contract_id: "".to_string(),
            whitelist_type: WhitelistType::None,
            sale_type : SaleType::Shared{
                min_allocation_per_user: 10,
                max_allocation_per_user: 200
            },
            distribution_type: DistributionType::Unlocked,

        }
    }


    pub(crate) fn internal_new_project_2()-> ProjectInput{
        ProjectInput{
            owner_id: "ref_finance.near".to_string(),
            whitelist_start_date: 1651172401000000000,
            whitelist_end_date: 1651604401000000000,
            sale_start_date: 1651604401000000000,
            sale_end_date: 1652068717000000000,
            token_contract_id: "contract.ref_finance.near".to_string(),
            token_raised_amount: U128(1000000000000000),
            token_sale_rate_numberator: 10u64,
            token_sale_rate_denominator: 1u64,
            fund_contract_id: "".to_string(),
            whitelist_type: WhitelistType::Ticket,
            sale_type : SaleType::Shared{
                min_allocation_per_user: 10,
                max_allocation_per_user: 200
            },
            distribution_type: DistributionType::Unlocked,

        }
    }


    pub(crate) fn internal_new_project_3()-> ProjectInput{
        ProjectInput{
            owner_id: "near_dao.near".to_string(),
            whitelist_start_date: 1651172401000000000,
            whitelist_end_date: 1651604401000000000,
            sale_start_date: 1651604401000000000,
            sale_end_date: 1652068717000000000,
            token_contract_id: "contract.near_dao.near".to_string(),
            token_raised_amount: U128(1000000000000000),
            token_sale_rate_numberator: 10u64,
            token_sale_rate_denominator: 1u64,
            fund_contract_id: "".to_string(),
            whitelist_type: WhitelistType::XToken(100),
            sale_type : SaleType::Shared{
                min_allocation_per_user: 10,
                max_allocation_per_user: 200
            },
            distribution_type: DistributionType::Unlocked,

        }
    }

    pub(crate) fn internal_new_project_4()-> ProjectInput{
        ProjectInput{
            owner_id: "owner_titan.testnet".to_string(),
            whitelist_start_date: 1640995200000000000,
            whitelist_end_date: 1641250800000000000,
            sale_start_date: 1641254400000000000,
            sale_end_date: 1641340800000000000,
            token_contract_id: "titan.testnet".to_string(),
            fund_contract_id: "usn.testnet".to_string(),
            token_raised_amount: U128(1000000000000000),
            token_sale_rate_numberator: 10u64,
            token_sale_rate_denominator: 1u64,
            whitelist_type: WhitelistType::None,
            sale_type: SaleType::Shared {
                min_allocation_per_user: 5000000000,
                max_allocation_per_user: 10000000000,
            },
            distribution_type: DistributionType::Unlocked,
        }
    }
}
    
    // pub(crate) fn internal_new_project_4()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "cryptocubes.near".to_string(),
    //         name: " Hunt for the Lost Cryptocubes".to_string(),
    //         logo_url: "https://pbs.twimg.com/media/FIYR_Z4XoAUf1qs?format=jpg&name=large".to_string(),
    //         description: "The adventure game where everything is an NFT".to_string(),
    //         introduction:"Near Metabuild Hackathon Play Hacks Prize Winners top 3".to_string(),
    //         categories: vec!["GameFi".to_string()],
    //         whitelist_start_date: 1647316800000000000,
    //         whitelist_end_date: 1647576000000000000,
    //         sale_start_date: 1647576000000000000,
    //         sale_end_date: 1647835200000000000,
    //         token_contract_id: "contract.cryptocubes.near".to_string(),
    //         token_symbol:"CUBE".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         total_fund_received: 0,
    //         sale_type: SaleType::FullUnlocked,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Rejected,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }


    // pub(crate) fn internal_new_project_5()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "flux_protocol.near".to_string(),
    //         name: "Flux".to_string(),
    //         logo_url: "https://cryptorobin.com/wp-content/uploads/2021/05/featured-image-flux.jpg".to_string(),
    //         description: "The trustless data layer for web3".to_string(),
    //         introduction:"Oracle connect to Near Protocol".to_string(),
    //         categories: vec!["Oracle".to_string()],
    //         whitelist_start_date: 1647835201000000000,
    //         whitelist_end_date: 1648094401000000000,
    //         sale_start_date: 1648094401000000000,
    //         sale_end_date: 1648353601000000000,
    //         token_contract_id: "contract.flux_protocol.near".to_string(),
    //         token_symbol:"FLX".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         total_fund_received: 0,
    //         sale_type: SaleType::FullUnlocked,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Distribution,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

    // pub(crate) fn internal_new_project_6()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "oin_finance.near".to_string(),
    //         name: "Oin Finance 6".to_string(),
    //         logo_url: "https://d235dzzkn2ryki.cloudfront.net/oin-finance_large.png".to_string(),
    //         description: "A robust turnkey stablecoin issuance platform built for the multi-chain universe.".to_string(),
    //         introduction:"Near's Stablecoin".to_string(),
    //         categories: vec!["Stablecoin".to_string(),"DeFi".to_string()],
    //         whitelist_start_date: 1646353602000000000,
    //         whitelist_end_date: 1646412802000000000,
    //         sale_start_date: 1646467201000000000,
    //         sale_end_date: 1647244801000000000,
    //         token_contract_id: "contract.oin_finance.near".to_string(),
    //         token_symbol:"OIN".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         total_fund_received: 0,
    //         sale_type: SaleType::Vested,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Sales,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

    // pub(crate) fn internal_new_project_7()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "paras.near".to_string(),
    //         name: "Project 7".to_string(),
    //         logo_url: "https://paras-media.s3-ap-southeast-1.amazonaws.com/paras-v2-twitter-card-large.png".to_string(),
    //         description: "See NFT digital card collectibles and creations from paras.near.".to_string(),
    //         introduction:"NFT Marketplace.".to_string(),
    //         categories: vec!["NFT".to_string()],
    //         whitelist_start_date: 1646131204000000000,
    //         whitelist_end_date: 1646390404000000000,
    //         sale_start_date: 1646531204000000000,
    //         sale_end_date: 1647374401000000000,
    //         token_contract_id: "contract.paras.near".to_string(),
    //         token_symbol:"PARAS".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         total_fund_received: 0,
    //         sale_type: SaleType::Vested,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Sales,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

    // pub(crate) fn internal_new_project_8()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "aurigami.near".to_string(),
    //         name: "Aurigami".to_string(),
    //         logo_url: "https://pbs.twimg.com/profile_images/1487610929646739459/EbB5N8IN.jpg".to_string(),
    //         description: "Noncustodial lending and borrowing haven on Aurora".to_string(),
    //         introduction:"Near lending&borrowing platform".to_string(),
    //         categories: vec!["Lending".to_string(),"DeFi".to_string()],
    //         whitelist_start_date: 1646510401000000000,
    //         whitelist_end_date: 1647360592000000000,
    //         sale_start_date: 1647374401000000000,
    //         sale_end_date: 1647633601000000000,
    //         token_contract_id: "contract.aurigami.near".to_string(),
    //         token_symbol:"AURI".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         sale_type: SaleType::Vested,
    //         total_fund_received: 0,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Whitelist,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

    // pub(crate) fn internal_new_project_9()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "reality_chain.near".to_string(),
    //         name: "Reality Chain".to_string(),
    //         logo_url: "https://pbs.twimg.com/media/FMsQe8CXIAsAUFK?format=jpg&name=large".to_string(),
    //         description: "Create your Metaverse with Reality Chain".to_string(),
    //         introduction:"Multi-chain, multi-engine, Metaverse-as-a-Service.".to_string(),
    //         categories: vec!["GameFi".to_string(),"Metaverse".to_string()],
    //         whitelist_start_date: 1646510401000000000,
    //         whitelist_end_date: 1647360592000000000,
    //         sale_start_date: 1647374401000000000,
    //         sale_end_date: 1647633601000000000,
    //         token_contract_id: "contract.reality_chain.near".to_string(),
    //         token_symbol:"REAL".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         sale_type: SaleType::Vested,
    //         total_fund_received: 0,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Whitelist,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

    // pub(crate) fn internal_new_project_10()-> ProjectInfo{
    //     ProjectInfo{
    //         owner_id: "aurora.near".to_string(),
    //         name: "Aurora".to_string(),
    //         logo_url: "https://near.org/wp-content/uploads/2021/05/aurora_fb.png".to_string(),
    //         description: "A product that helps Ethereum users and dApps to easily move to the NEAR blockchain. ".to_string(),
    //         introduction:"Near&Ethereum bridge".to_string(),
    //         categories: vec![],
    //         whitelist_start_date: 1647374401000000000,
    //         whitelist_end_date: 1647892801000000000,
    //         sale_start_date: 1649826001000000000,
    //         sale_end_date: 1650085201000000000,
    //         token_contract_id: "contract.aurora.near".to_string(),
    //         token_symbol:"AURORA".to_string(),
    //         token_decimal: 8,
    //         token_amount_per_sale_slot: 100,
    //         token_raised_amount: 400_000,
    //         token_sale_rate: Rate::new(1u64, 100u64),
    //         fund_contract_id: None,
    //         fund_symbol: "Near".to_string(),
    //         sale_type: SaleType::Vested,
    //         total_fund_received: 0,
    //         configuration: ProjectConfiguration{ 
    //             max_staking_tickets_per_user: 500, 
    //             max_win_tickets_per_user: 250, 
    //         },
    //         status:ProjectStatus::Preparation,
    //         total_allocations: 0,
    //         total_staking_tickets: 0,
    //         total_social_tickets: 0,
    //         total_referral_tickets: 0,
    //     }
    // }

