use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, WasmMsg,
};
use omniflix_std::types::omniflix::onft::v1::{MsgCreateDenom, MsgMintONFT, Metadata};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Profile, CONFIG, PROFILES};
use crate::error::ContractError;
use rand::{distributions::Alphanumeric, Rng};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        admin: deps.api.addr_validate(&msg.admin)?,
        denom_id: msg.denom_id.clone(),
        fee_amount: msg.fee_amount,
        fee_denom: msg.fee_denom,
    };

    CONFIG.save(deps.storage, &config)?;

    let create_denom_msg = MsgCreateDenom {
        id: msg.denom_id,
        name: msg.denom_name,
        symbol: msg.denom_symbol,
        schema: r#"{
            "username":"string",
            "bio":"string",
            "social_links":"string[]",
            "profile_image":"string"
        }"#.into(),
        sender: env.contract.address.to_string(),
        description: "OmniFlix Profile Collection".into(),
        preview_uri: "".into(),
        uri: "".into(),
        uri_hash: "".into(),
        data: "".into(),
    };

    Ok(Response::new()
        .add_message(create_denom_msg)
        .add_attribute("action", "instantiate")
        .add_attribute("denom_created", "true"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateProfile {
            username,
            bio,
            social_links,
            profile_image,
        } => execute_create_profile(deps, env, info, username, bio, social_links, profile_image),
    }
}

pub fn execute_create_profile(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    username: String,
    bio: String,
    social_links: Vec<String>,
    profile_image: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if PROFILES.has(deps.storage, &info.sender) {
        return Err(ContractError::ProfileExists {});
    }

    let paid = info.funds.iter().find(|coin| {
        coin.denom == config.fee_denom && coin.amount.u128() >= config.fee_amount
    }).ok_or(ContractError::InsufficientFunds {})?;

    // Increment Profile Sequence
    let seq = PROFILE_SEQ.may_load(deps.storage)?.unwrap_or_default() + 1;
    PROFILE_SEQ.save(deps.storage, &seq)?;

    // Generate NFT ID with sequence
    let nft_id = format!("ofp", seq);

    let metadata = Metadata {
        name: username.clone(),
        description: bio.clone(),
        uri: profile_image.clone(),
        uri_hash: "".into(),
        data: serde_json::json!({
            "username": username,
            "bio": bio,
            "social_links": social_links,
            "profile_image": profile_image
        }).to_string(),
    };

    let mint_onft_msg = MsgMintONFT {
        id: nft_id.clone(),
        denom_id: config.denom_id,
        metadata: Some(metadata),
        transferable: false,
        sender: env.contract.address.to_string(),
        recipient: info.sender.to_string(),
    };

    PROFILES.save(deps.storage, &info.sender, &Profile {
        username,
        bio,
        social_links,
        profile_image,
        nft_id,
    })?;

    Ok(Response::new()
        .add_message(mint_onft_msg)
        .add_attribute("action", "create_profile")
        .add_attribute("nft_id", nft_id)
        .add_attribute("fee_paid", paid.amount.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<cosmwasm_std::Binary> {
    match msg {
        QueryMsg::GetProfile { address } => {
            let addr = deps.api.addr_validate(&address)?;
            let profile = PROFILES.load(deps.storage, &addr)?;
            to_binary(&profile)
        }
    }
}
