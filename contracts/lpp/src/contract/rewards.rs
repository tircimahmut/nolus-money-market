use cosmwasm_std::{Addr, Deps, DepsMut, Env, Response, Storage, MessageInfo};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::ContractError;
use crate::lpp::LiquidityPool;
use crate::msg::{LppBalanceResponse, RewardsResponse};
use crate::state::Deposit;
use finance::bank::{self, BankStub, BankAccount};
use finance::currency::{Currency, Nls};
use finance::coin::Coin;

pub fn try_distribute_rewards(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    let amount: Coin<Nls> = bank::received(&info.funds)?;
    Deposit::distribute_rewards(deps, amount)?;

    Ok(Response::new().add_attribute("method", "try_distribute_rewards"))
}

pub fn try_claim_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    other_recipient: Option<Addr>,
) -> Result<Response, ContractError> {
    let recipient = other_recipient.unwrap_or_else(|| info.sender.clone());
    let mut deposit = Deposit::load(deps.storage, info.sender)?;
    let reward = deposit.claim_rewards(deps.storage)?;

    let bank = BankStub::my_account(&env, &deps.querier);
    let msg = bank.send(reward, &recipient)?;

    let response = Response::new()
        .add_attribute("method", "try_claim_rewards")
        .add_submessage(msg);

    Ok(response)
}

pub fn query_lpp_balance<LPN>(
    deps: Deps,
    env: Env,
) -> Result<LppBalanceResponse<LPN>, ContractError>
where
    LPN: 'static + Currency + DeserializeOwned + Serialize,
{
    let lpp = LiquidityPool::<LPN>::load(deps.storage)?;
    lpp.query_lpp_balance(&deps, &env)
}

pub fn query_rewards(storage: &dyn Storage, addr: Addr) -> Result<RewardsResponse, ContractError> {
    let deposit = Deposit::load(storage, addr)?;
    let rewards = deposit.query_rewards(storage)?;
    Ok(RewardsResponse {
        rewards
    })
}
