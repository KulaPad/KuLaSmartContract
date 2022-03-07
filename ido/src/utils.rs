use crate::*;

pub(crate) fn get_storage_key(key: StorageKey) -> Vec<u8> {
    key.try_to_vec().unwrap()
}

pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash{
    let mut hash = CryptoHash::default();
    // Hash account ID rồi return chính nó
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));

    hash
}

pub(crate) fn hash_project_id(project_id: ProjectId) -> CryptoHash{
    let mut hash = CryptoHash::default();

    hash.copy_from_slice(&env::sha256(project_id.to_string().as_bytes()));
    hash
}

pub(crate) fn assert_one_yocto_near(){
    assert_eq!(env::attached_deposit()
                ,1
                ,"Require attached deposit of exactly 1 yoctoNEAR")
}

pub(crate) fn assert_at_least_one_yocto_near(){
    assert!(env::attached_deposit() >= 1
                ,"Require attached deposit of exactly 1 yoctoNEAR")
}


#[near_bindgen]
impl IDOContract{
    pub fn get_tickets_amount_for_owner_id(&self, owner_id: AccountId)-> u64{
        self.staking_tickets_amount_per_owner_id.get(&owner_id).unwrap_or(0)
    }

    pub fn create_10_sample_project(&mut self){
        Self::new("ido-kulapad.testnet".to_string());
        self.projects.insert(&1,&Self::internal_new_project_1());
        self.projects.insert(&2,&Self::internal_new_project_2());
        self.projects.insert(&3,&Self::internal_new_project_3());
        self.projects.insert(&4,&Self::internal_new_project_4());
        self.projects.insert(&5,&Self::internal_new_project_5());
        self.projects.insert(&6,&Self::internal_new_project_6());
        self.projects.insert(&7,&Self::internal_new_project_7());
        self.projects.insert(&8,&Self::internal_new_project_8());
        self.projects.insert(&9,&Self::internal_new_project_9());
        self.projects.insert(&10,&Self::internal_new_project_10());

    }
}

