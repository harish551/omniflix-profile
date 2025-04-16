use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
    pub fee_amount: u128,
    pub fee_denom: String,
    pub denom_id: String,
    pub denom_name: String,
    pub denom_symbol: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateProfile {
        username: String,
        bio: String,
        social_links: Vec<String>,
        profile_image: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ProfileResponse)]
    Profile { address: String },
}

#[cw_serde]
pub struct ProfileResponse {
    pub username: String,
    pub bio: String,
    pub social_links: Vec<String>,
    pub profile_image: String,
    pub nft_id: String,
}
