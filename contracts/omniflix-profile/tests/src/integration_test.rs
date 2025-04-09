use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coin};
use omniflix_profile::contract::{execute, instantiate};
use omniflix_profile::msg::{ExecuteMsg, InstantiateMsg};


#[test]
fn test_profile_creation() {
    let mut deps = mock_dependencies();

    let instantiate_msg = InstantiateMsg {
        admin: "admin".into(),
        fee_amount: 1000,
        fee_denom: "uflix".into(),
        denom_id: "profiledenom".into(),
        denom_name: "OmniFlix Profiles".into(),
        denom_symbol: "OFP".into(),
    };

    instantiate(deps.as_mut(), mock_env(), mock_info("admin", &[]), instantiate_msg).unwrap();

    let create_profile_msg = ExecuteMsg::CreateProfile {
        username: "harish".into(),
        bio: "Blockchain dev".into(),
        social_links: vec!["twitter.com/harish".into()],
        profile_image: "ipfs://img".into(),
    };

    let info = mock_info("user1", &[coin(1000, "uflix")]);
    let res = execute(deps.as_mut(), mock_env(), info, create_profile_msg).unwrap();

    assert_eq!(res.attributes[0].value, "create_profile");
}

