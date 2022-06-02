use std::collections::HashSet;

use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, Env, Response, StdError, StdResult, SubMsg,
    Uint128, WasmMsg,
};

use finance::percent::Percent;
use lease::msg::{LoanForm, NewLeaseForm};

use crate::error::ContractError;
use crate::lpp_querier::LppQuerier;
use crate::msg::{ConfigResponse, QuoteResponse};
use crate::state::config::Config;
use crate::state::leaser::Loans;

pub struct Leaser {}

impl Leaser {
    pub fn try_borrow(
        deps: DepsMut,
        amount: Vec<Coin>,
        sender: Addr,
        currency: String,
    ) -> Result<Response, ContractError> {
        let config = Config::load(deps.storage)?;
        let instance_reply_id = Loans::next(deps.storage, sender.clone())?;
        Ok(
            Response::new().add_submessages(vec![SubMsg::reply_on_success(
                CosmosMsg::Wasm(WasmMsg::Instantiate {
                    admin: None,
                    code_id: config.lease_code_id,
                    funds: amount,
                    label: "lease".to_string(),
                    msg: to_binary(&Leaser::open_lease_msg(sender, config, currency))?,
                }),
                instance_reply_id,
            )]),
        )
    }

    pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
        let config = Config::load(deps.storage)?;
        Ok(ConfigResponse { config })
    }

    pub fn query_loans(deps: Deps, owner: Addr) -> StdResult<HashSet<Addr>> {
        Loans::get(deps.storage, owner)
    }

    pub fn query_quote(_env: Env, deps: Deps, downpayment: Coin) -> StdResult<QuoteResponse> {
        // borrowUST = LeaseInitialLiability% * downpaymentUST / (1 - LeaseInitialLiability%)
        if downpayment.amount.is_zero() {
            return Err(StdError::generic_err(
                "cannot open lease with zero downpayment",
            ));
        }
        let config = Config::load(deps.storage)?;
        let numerator = Uint128::from(config.liability.initial) * downpayment.amount;
        let denominator = Uint128::from(100 - config.liability.initial);

        let borrow_amount = numerator / denominator;
        let total_amount = borrow_amount + downpayment.amount;

        Ok(QuoteResponse {
            total: Coin::new(total_amount.u128(), downpayment.denom.clone()),
            borrow: Coin::new(borrow_amount.u128(), downpayment.denom.clone()),
            annual_interest_rate: LppQuerier::get_annual_interest_rate(deps, downpayment)?,
        })
    }
    pub(crate) fn open_lease_msg(sender: Addr, config: Config, currency: String) -> NewLeaseForm {
        NewLeaseForm {
            customer: sender.into_string(),
            currency,
            liability: finance::liability::Liability::new(
                Percent::from_percent(config.liability.initial.into()),
                Percent::from_percent((config.liability.healthy - config.liability.initial).into()),
                Percent::from_percent((config.liability.max - config.liability.healthy).into()),
                config.recalc_hours,
            ),
            loan: LoanForm {
                annual_margin_interest: config.lease_interest_rate_margin,
                lpp: config.lpp_ust_addr.into_string(),
                interest_due_period_secs: config.repayment.period_sec, // 90 days TODO use a crate for daytime calculations
                grace_period_secs: config.repayment.grace_period_sec,
            },
        }
    }
}
