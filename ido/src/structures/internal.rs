use crate::*;

#[near_bindgen]
impl IDOContract{

    pub fn internal_new_project_1()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project1.near".to_string(),
            name: "Project 1".to_string(),
            logo_url: "https://twitter.com/alevoro_/status/1441701904581492745/photo/1".to_string(),
            description: "Marketplace for collateralized NFT loans".to_string(),
            introduction:"Near Metabuild Hackathon top 3".to_string(),
            categories: vec!["NFT".to_string(),"DeFi".to_string()],
            whitelist_date: 1644159117000000000,
            sale_start_date: 1644763917000000000,
            sale_end_date: 1645368717000000000,
            token_contract_id: "contract.project1.near".to_string(),
            token_symbol:"PRJ1".to_string(),
            token_raised_amount: 40_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 10_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::FullUnlocked,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 100, 
            },
            current_ticket_id: 100_000,
            status:ProjectStatus::Distribution
        }
    }

    pub fn internal_new_project_2()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project2.near".to_string(),
            name: "Project 2".to_string(),
            logo_url: "https://coinpricepredict.com/wp-content/uploads/2022/03/Reffinance.jpg".to_string(),
            description: "Starting point to the #NEAR Ecosystem with a leading AMM & a synchronous #DeFi Shard.".to_string(),
            introduction:"The first AMM dex in Near Protocol".to_string(),
            categories: vec!["AMM DEX".to_string(),"DeFi".to_string()],
            whitelist_date: 1642258317000000000,
            sale_start_date: 1642690895000000000,
            sale_end_date: 1643122895000000000,
            token_contract_id: "contract.project2.near".to_string(),
            token_symbol:"PRJ2".to_string(),
            token_raised_amount: 10_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 200_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::FullUnlocked,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 800, 
                max_win_tickets_per_user: 150, 
            },
            current_ticket_id: 250_000,
            status:ProjectStatus::Distribution
        }
    }

    pub fn internal_new_project_3()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project3.near".to_string(),
            name: "Project 3".to_string(),
            logo_url: "https://neardao.com/img/logo_with_text_cropt.png".to_string(),
            description: "Launch a DAO in 5 minutes".to_string(),
            introduction:"Near Metabuild Hackathon top 1".to_string(),
            categories: vec!["DAO".to_string(),"DeFi".to_string()],
            whitelist_date: 1641052657000000000,
            sale_start_date: 1641398257000000000,
            sale_end_date: 1641830257000000000,
            token_contract_id: "contract.project3.near".to_string(),
            token_symbol:"PRJ3".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::FullUnlocked,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Rejected
        }
    }

    pub fn internal_new_project_4()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project4.near".to_string(),
            name: "Project 4".to_string(),
            logo_url: "https://pbs.twimg.com/media/FIYR_Z4XoAUf1qs?format=jpg&name=large".to_string(),
            description: "The adventure game where everything is an NFT".to_string(),
            introduction:"Near Metabuild Hackathon Play Hacks Prize Winners top 3".to_string(),
            categories: vec!["GameFi".to_string()],
            whitelist_date: 1647316800000000000,
            sale_start_date: 1647576000000000000,
            sale_end_date: 1647835200000000000,
            token_contract_id: "contract.project4.near".to_string(),
            token_symbol:"PRJ4".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::FullUnlocked,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Rejected
        }
    }


    pub fn internal_new_project_5()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project5.near".to_string(),
            name: "Project 5".to_string(),
            logo_url: "https://cryptorobin.com/wp-content/uploads/2021/05/featured-image-flux.jpg".to_string(),
            description: "The trustless data layer for web3".to_string(),
            introduction:"Prj5 introduction".to_string(),
            categories: vec!["Oracle".to_string()],
            whitelist_date: 1647835201000000000,
            sale_start_date: 1648094401000000000,
            sale_end_date: 1648353601000000000,
            token_contract_id: "contract.project5.near".to_string(),
            token_symbol:"PRJ5".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::FullUnlocked,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Distribution
        }
    }

    pub fn internal_new_project_6()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project6.near".to_string(),
            name: "Project 6".to_string(),
            logo_url: "https://d235dzzkn2ryki.cloudfront.net/oin-finance_large.png".to_string(),
            description: "A robust turnkey stablecoin issuance platform built for the multi-chain universe.".to_string(),
            introduction:"Prj6 introduction".to_string(),
            categories: vec!["Stablecoin".to_string(),"DeFi".to_string()],
            whitelist_date: 1648353602000000000,
            sale_start_date: 1648612802000000000,
            sale_end_date: 1648785602000000000,
            token_contract_id: "contract.project6.near".to_string(),
            token_symbol:"PRJ6".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::Vested,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Sales
        }
    }

    pub fn internal_new_project_7()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project7.near".to_string(),
            name: "Project 7".to_string(),
            logo_url: "https://paras-media.s3-ap-southeast-1.amazonaws.com/paras-v2-twitter-card-large.png".to_string(),
            description: "Prj7 description".to_string(),
            introduction:"NFT Marketplace.".to_string(),
            categories: vec!["NFT".to_string()],
            whitelist_date: 1648353603000000000,
            sale_start_date: 1649044803000000000,
            sale_end_date: 1649304003000000000,
            token_contract_id: "contract.project7.near".to_string(),
            token_symbol:"PRJ7".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::Vested,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Sales
        }
    }

    pub fn internal_new_project_8()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project8.near".to_string(),
            name: "Project 8".to_string(),
            logo_url: "https://pbs.twimg.com/profile_images/1487610929646739459/EbB5N8IN.jpg".to_string(),
            description: "Noncustodial lending and borrowing haven on Aurora".to_string(),
            introduction:"Prj8 introduction".to_string(),
            categories: vec!["Lending".to_string(),"DeFi".to_string()],
            whitelist_date: 1649131204000000000,
            sale_start_date: 1649390404000000000,
            sale_end_date: 1649736004000000000,
            token_contract_id: "contract.project8.near".to_string(),
            token_symbol:"PRJ8".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::Vested,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Whitelist
        }
    }

    pub fn internal_new_project_9()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project9.near".to_string(),
            name: "Project 9".to_string(),
            logo_url: "https://pbs.twimg.com/media/FMsQe8CXIAsAUFK?format=jpg&name=large".to_string(),
            description: "Prj9 description".to_string(),
            introduction:"Multi-chain, multi-engine, Metaverse-as-a-Service.".to_string(),
            categories: vec!["GameFi".to_string(),"Metaverse".to_string()],
            whitelist_date: 1649221201000000000,
            sale_start_date: 1649480401000000000,
            sale_end_date: 1649739601000000000,
            token_contract_id: "contract.project9.near".to_string(),
            token_symbol:"PRJ9".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::Vested,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Whitelist
        }
    }

    pub fn internal_new_project_10()-> ProjectInfo{
        ProjectInfo{
            owner_id: "project10.near".to_string(),
            name: "Project 10".to_string(),
            logo_url: "https://near.org/wp-content/uploads/2021/05/aurora_fb.png".to_string(),
            description: "A product that helps Ethereum users and dApps to easily move to the NEAR blockchain. ".to_string(),
            introduction:"Prj10 introduction".to_string(),
            categories: vec![],
            whitelist_date: 1649566801000000000,
            sale_start_date: 1649826001000000000,
            sale_end_date: 1650085201000000000,
            token_contract_id: "contract.project10.near".to_string(),
            token_symbol:"PRJ10".to_string(),
            token_raised_amount: 200_000_000_000_000_000_000_000_000_000_000	,
            token_sale_rate: 2_000_000_000_000_000_000_000,
            fund_contract_id: None,
            fund_symbol: "Near".to_string(),
            sale_type: SaleType::Vested,
            configuration: ProjectConfiguration{ 
                max_staking_tickets_per_user: 500, 
                max_win_tickets_per_user: 250, 
            },
            current_ticket_id: 80_000,
            status:ProjectStatus::Preparation
        }
    }
}