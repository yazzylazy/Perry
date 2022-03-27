use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    name::{Strin}

};

    // mutable storage
    //deps.api.addr_validate

    // this stuff is more useful for bocking money for a certain period of time
    //env.contract.address
   // env.block.height

   // ffind out sutff about the sender
   //info.sender

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> { // response , err = Result
   
  // States.save(deps.storage, &data) // is this correct?
    Ok(Response::new())
}

//numbers.save -> allows to store data in map(key is the second parameter and value is the 3rd one)
// numbers.remove
//match msg?
// what if the terra station instation had actual codes
// terra finder

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::execute {address} => try_execute(deps,info.sender.clone(),address),
        ExecuteMsg::accept {address} => try_accept(deps,info.sender.clone(),address),
        ExecuteMsg::cancel {address} => try_cancel(deps,info.sender.clone(),address),
    }
}

pub fn try_execute(deps: DepsMut,address: Addr,) -> Result<Response, ContractError> {
    States.update(deps.storage, |mut state| -> Result<_, ContractError> {

        if(deps.){

        }
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_execute"))
}

pub fn try_accept(deps: DepsMut,address: Addr,) -> Result<Response, ContractError> {
    States.update(deps.storage, |mut state| -> Result<_, ContractError> {

        if(deps.){

        }
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_execute"))
}

pub fn try_cancel(deps: DepsMut,address: Addr,) -> Result<Response, ContractError> {
    States.update(deps.storage, |mut state| -> Result<_, ContractError> {

        if(deps.){

        }
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_execute"))
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_binary(&"")?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env,mock_info,mock_dependencies}

    #[test]
    fn button_is_clicked() {
        let mut deps = mock_dependecies(&[]); 

        let info =mock_info("sender",&[]);// put funds?

        

    }
}
