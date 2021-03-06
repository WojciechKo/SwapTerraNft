use cosmwasm_std::StdError;

use swaps::contract::{execute, instantiate, query};
use swaps::error::ContractError;
use swaps::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SwapResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn cancel_swap() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // When Swap does not exists
        // Cancel Swap returns an error
        let cancel_swap_msg = ExecuteMsg::CancelSwap {
            swap_id: String::from("1"),
        };
        match execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            cancel_swap_msg.clone(),
        ) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(err, ContractError::SwapNotFound {}),
        };

        // Initiate Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::InitiateSwap {
            collection: String::from("gp_collection"),
            token_id: String::from("123"),
        };
        let swap_created =
            execute(deps.as_mut(), mock_env(), swapper_info, create_swap_msg).unwrap();
        let created_swap_id = swap_created
            .attributes
            .iter()
            .find(move |x| x.key == String::from("swap_id"))
            .unwrap()
            .value
            .clone();

        let get_swap_msg = QueryMsg::GetSwap {
            swap_id: created_swap_id.clone(),
        };

        let get_swap_response = query(deps.as_ref(), mock_env(), get_swap_msg.clone()).unwrap();
        let _: SwapResponse = from_binary(&get_swap_response).unwrap();

        // Cancel Swap
        match execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            cancel_swap_msg.clone(),
        ) {
            Ok(_) => (),
            Err(e) => panic!("Unexpected error: {:#?}", e),
        };

        // When Swap is canceled
        // Fetch Swap returns an error
        match query(deps.as_ref(), mock_env(), get_swap_msg.clone()) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(
                err,
                StdError::NotFound {
                    kind: String::from("Swap")
                }
            ),
        };

        Ok(())
    }
}
