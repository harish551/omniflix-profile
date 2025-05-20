use cosmwasm_std::{testing::mock_dependencies, coin, Addr, Empty};
use cw_multi_test::{App, AppResponse, Contract, ContractWrapper, Executor};
use crate::msg::{ExecuteMsg, InstantiateMsg};

fn mock_app() -> App {
    App::default()
}

fn contract_omniflix_profile() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query,
    );
    Box::new(contract)
}

#[test]
fn test_instantiate_and_create_profile() {
    let mut app = App::new(|router, _, storage| {
        router
           .bank
           .init_balance(storage, &Addr::unchecked("owner"), ([coin(100000000, "uflix")]).to_vec())
           .unwrap()
    });
    let owner = mock_dependencies().api.addr_make("owner").to_string();

    let code_id = app.store_code(contract_omniflix_profile());

    // Instantiate contract
    let instantiate_msg = InstantiateMsg {
        denom_id: "onftdenomtest1".to_string(),
        denom_name: "OmniFlix Profile".to_string(),
        denom_symbol: "OFP1".to_string(),
        fee_denom: "uflix".to_string(),
        fee_amount: 10000
    };

    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(owner.clone()),
            &instantiate_msg,
            &[coin(1000000, "uflix")],
            "omniflix-profile",
            None,
        )
        .unwrap();

    // Execute: Create a profile
    let execute_msg = ExecuteMsg::CreateProfile {
        username: "anakin".to_string(),
        bio: "A builder at OmniFlix".to_string(),
        profile_image: "https://example.com/image.png".to_string(),
        social_links: vec![
            "https://twitter.com/anakin".to_string(),
            "https://github.com/anakin".to_string(),
        ],
    };

    let res: AppResponse = app
        .execute_contract(
            Addr::unchecked("user1"),
            contract_addr.clone(),
            &execute_msg,
            &[coin(1000, "uflix")],
        )
        .unwrap();

    // Assert successful execution
    assert_eq!(res.events.iter().any(|e| e.ty == "wasm"), true);
}

