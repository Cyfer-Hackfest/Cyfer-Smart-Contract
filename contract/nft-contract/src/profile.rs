use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UserProfile {
    pub name: String,
    pub image: String,
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn update_profile(
        &mut self,
        user_id: AccountId,
        name: String,
        image: String
    ) {
        let user_info = UserProfile {
            name,
            image,
        };
        self.user_profile.insert(&user_id, &user_info);
    }
    
    pub fn get_profile(self, user_id: AccountId) -> Option<UserProfile> {
        self.user_profile.get(&user_id)
    }
}