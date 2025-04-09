use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub fee_amount: u128,
    pub fee_denom: String,
    pub denom_id: String,
    pub denom_name: String,
    pub denom_symbol: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateProfile {
        username: String,
        bio: String,
        social_links: Vec<String>,
        profile_image: String,
    },
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetProfile { address: String },
}
