#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:11-08";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/*
Instantiate is like a constructor, here we are defining deps (Dependencies), the Enivronment, MessageInfo, 
and the InstantiateMsg. Furthermore, we define the state struct which has values count and owner. 
State will be used to define our data types that we use in the counter applicaiton. Whenever we make changes 
to these values (count, owner), we are changing the State item. Lastly we define the contract version and save that 
to our contract storage which is accessed with the "deps" variable.
*/
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}




