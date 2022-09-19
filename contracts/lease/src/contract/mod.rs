#[cfg(feature = "cosmwasm-bindings")]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response as CwResponse};
use cw_storage_plus::Item;

use crate::{
    contract::state::{Response, State},
    error::{ContractError, ContractResult},
    msg::{ExecuteMsg, NewLeaseForm, StateQuery},
};

use self::state::{Active, Controller, NoLease, NoLeaseFinish};

mod alarms;
mod close;
mod cmd;
mod open;
mod repay;
mod state;

const DB_ITEM: Item<State> = Item::new("state");

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    form: NewLeaseForm,
) -> ContractResult<CwResponse> {
    NoLease {}.instantiate(&mut deps, env, info, form).and_then(
        |Response {
             cw_response,
             next_state,
         }| {
            save(&next_state, &mut deps)?;

            Ok(cw_response)
        },
    )
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn reply(mut deps: DepsMut, env: Env, msg: Reply) -> ContractResult<CwResponse> {
    NoLeaseFinish {}.reply(&mut deps, env, msg).and_then(
        |Response {
             cw_response,
             next_state,
         }| {
            save(&next_state, &mut deps)?;

            Ok(cw_response)
        },
    )
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn execute(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<CwResponse> {
    Active {}.execute(&mut deps, env, info, msg).and_then(
        |Response {
             cw_response,
             next_state,
         }| {
            save(&next_state, &mut deps)?;

            Ok(cw_response)
        },
    )
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn query(deps: Deps, env: Env, msg: StateQuery) -> ContractResult<Binary> {
    Active {}.query(deps, env, msg)
}

fn save(next_state: &State, deps: &mut DepsMut) -> ContractResult<()> {
    DB_ITEM
        .save(deps.storage, next_state)
        .map_err(ContractError::from)
}
