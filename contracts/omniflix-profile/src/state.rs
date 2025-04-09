use cw_storage_plus::{Item, Map};
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub admin: Addr,
    pub denom_id: String,
    pub fee_amount: u128,
    pub fee_denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub social_links: Vec<String>,
    pub profile_image: String,
    pub nft_id: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROFILES: Map<&Addr, Profile> = Map::new("profiles");
pub const PROFILE_SEQ: Item<u64> = Item::new("profile_seq");
