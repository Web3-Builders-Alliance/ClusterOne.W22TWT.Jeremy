#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::Decrement {} => execute::decrement(deps),
        ExecuteMsg::IncrementBy {amount} => execute::increment_by(deps, amount),
        ExecuteMsg::DecrementBy {amount} => execute::decrement_by(deps, amount),
    }
}

pub mod execute {
    use super::*;

    pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }
    pub fn decrement(deps: DepsMut) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count -= 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "decrement"))
    }
    pub fn increment_by(deps: DepsMut, amount:i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += amount;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment_by"))
    }
    pub fn decrement_by(deps: DepsMut, amount:i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count -= amount;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "decrement_by"))
    }
    


    pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count;
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
}