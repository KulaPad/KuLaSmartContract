use crate::*;

#[near_bindgen]
impl IDOContract{

    fn internal_new_project_1()-> ProjectInput{
        ProjectInput {
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

    pub fn create_sample_projects(&mut self) {
        self.assert_test_mode_and_owner();

        self.create_project(Self::internal_new_project_1());
    }

    pub fn create_sample_project(&mut self, project_no: u8) {
        self.assert_test_mode_and_owner();

        let project: ProjectInput = match project_no {
            1 => Self::internal_new_project_1(),
            _ => panic!("No project no: {}", project_no)
        };

        self.create_project(project);
    }

    pub fn update_project_whitelist_date(&mut self, project_id: ProjectId, new_whitelist_start_date: Option<Timestamp>, new_whitelist_end_date: Option<Timestamp>) {
        self.assert_test_mode_and_owner();

        let mut project = self.projects.get(&project_id).expect("No project found");

        let a_half_of_whitelist_period = (project.whitelist_end_date - project.whitelist_start_date) / 2;
        let current_time = env::block_timestamp();

        println!("update_project_whitelist_date: current_time: {}, a_half_of_period: {}", current_time, a_half_of_whitelist_period);

        project.whitelist_start_date = new_whitelist_start_date.unwrap_or(current_time - a_half_of_whitelist_period);
        project.whitelist_end_date = new_whitelist_end_date.unwrap_or(current_time + a_half_of_whitelist_period);

        self.projects.insert(&project_id, &project);

    }

    pub fn update_project_sales_date(&mut self, project_id: ProjectId) {
        self.assert_test_mode_and_owner();

        let mut project = self.projects.get(&project_id).expect("No project found");

        let a_half_of_sales_period = (project.sale_end_date - project.sale_start_date) / 2;
        let current_time = env::block_timestamp();
        let time_to_change: i128 = current_time as i128 - project.sale_start_date as i128 - a_half_of_sales_period as i128;

        println!("Current time: {}", current_time);
        println!("Sales start: {}", project.sale_start_date);
        println!("Period: {}", a_half_of_sales_period);
        println!("To be change: {}", time_to_change);
        
        project.whitelist_start_date = (project.whitelist_start_date as i128 + time_to_change) as u64;
        project.whitelist_end_date = (project.whitelist_end_date as i128 + time_to_change) as u64;
        project.sale_start_date = (project.sale_start_date as i128 + time_to_change) as u64;
        project.sale_end_date = (project.sale_end_date as i128 + time_to_change) as u64;

        self.projects.insert(&project_id, &project);
    }

    pub fn update_project_sales_date_to_end(&mut self, project_id: ProjectId) {
        self.assert_test_mode_and_owner();

        let mut project = self.projects.get(&project_id).expect("No project found");
        let current_timestamp = get_current_time();
        if project.sale_end_date <= current_timestamp {
            env::log("The sales end time is correct. No need to update.".as_bytes());
        }

        let time_period_after_sales_end = project.sale_end_date - current_timestamp + 1000;

        println!("Sales end date: {}, Current time: {}, Period: {}", project.sale_end_date, current_timestamp, time_period_after_sales_end);
        println!("Whitelist start date: {}", project.whitelist_start_date);
        println!("Whitelist end date: {}", project.whitelist_end_date);
        println!("Sales start date: {}", project.sale_start_date);
        println!("Sales end date: {}", project.sale_end_date);
        
        project.whitelist_start_date -= time_period_after_sales_end;
        project.whitelist_end_date -= time_period_after_sales_end;
        project.sale_start_date -= time_period_after_sales_end;
        project.sale_end_date -= time_period_after_sales_end;

        self.projects.insert(&project_id, &project);
    }

    pub fn update_project_status(&mut self, project_id: ProjectId, new_status: ProjectStatus) {
        self.assert_test_mode_and_owner();

        let mut project = self.projects.get(&project_id).expect("No project found");
        project.status = new_status;

        self.projects.insert(&project_id, &project);
    }
}