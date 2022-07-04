#[cfg(feature = "cosmwasm-bindings")]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Api, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;
use cw_utils::one_coin;
use finance::bank::BankStub;
use finance::coin_legacy::from_cosmwasm;
use lpp::stub::{Lpp, LppStub};

use crate::error::{ContractError, ContractResult};
use crate::lease::{Currency, Lease};
use crate::msg::{ExecuteMsg, NewLeaseForm, StatusQuery, StatusResponse};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: NewLeaseForm,
) -> ContractResult<Response> {
    // TODO restrict the Lease instantiation only to the Leaser addr by using `nolusd tx wasm store ... --instantiate-only-address <addr>`
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let downpayment = one_coin(&info)?;
    let borrow = msg.amount_to_borrow(downpayment)?;
    let lpp = lpp(msg.loan.lpp.clone(), deps.api)?;
    msg.save(deps.storage)?;
    let req = <LppStub as Lpp<Currency>>::open_loan_req(&lpp, from_cosmwasm(borrow)?)?;

    // TODO define an OpenLoanRequest(downpayment, borrowed_amount) and persist it

    Ok(Response::new().add_submessage(req))
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> ContractResult<Response> {
    // TODO debug_assert the balance is increased with the borrowed amount
    // TODO load the top request and pass it as a reply
    let new_lease_form = NewLeaseForm::pull(deps.storage)?;
    let lpp = lpp(new_lease_form.loan.lpp.clone(), deps.api)?;
    <LppStub as Lpp<Currency>>::open_loan_resp(&lpp, msg).map_err(ContractError::OpenLoanError)?;

    let lease = new_lease_form.into_lease(lpp, env.block.time, deps.api)?;
    lease.store(deps.storage)?;

    Ok(Response::default())
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::Repay() => try_repay(deps, env, info),
        ExecuteMsg::Close() => try_close(deps, env, info),
        ExecuteMsg::Alarm(price) => todo!(),
    }
}

#[cfg_attr(feature = "cosmwasm-bindings", entry_point)]
pub fn query(deps: Deps, env: Env, _msg: StatusQuery) -> ContractResult<Binary> {
    let lease = Lease::<LppStub>::load(deps.storage)?;
    let bank_account = BankStub::my_account(&env, &deps.querier);
    let resp: StatusResponse = lease.state(
        env.block.time,
        bank_account,
        &deps.querier,
        env.contract.address.clone(),
    )?;
    to_binary(&resp).map_err(ContractError::from)
}

fn try_repay(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let payment = one_coin(&info)?;
    let mut lease = Lease::<LppStub>::load(deps.storage)?;
    let lpp_loan_repay_req =
        lease.repay(payment, env.block.time, &deps.querier, env.contract.address)?;
    lease.store(deps.storage)?;
    let resp = if let Some(req) = lpp_loan_repay_req {
        Response::default().add_submessage(req)
    } else {
        Response::default()
    };
    Ok(resp)
}

fn try_close(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let lease = Lease::<LppStub>::load(deps.storage)?;
    if !lease.owned_by(&info.sender) {
        return ContractResult::Err(ContractError::Unauthorized {});
    }

    let bank_account = BankStub::my_account(&env, &deps.querier);
    let bank_req = lease.close(env.contract.address.clone(), &deps.querier, bank_account)?;
    Ok(Response::default().add_submessage(bank_req))
}

fn lpp(address: String, api: &dyn Api) -> StdResult<LppStub> {
    lpp::stub::LppStub::try_from(address, api)
}
