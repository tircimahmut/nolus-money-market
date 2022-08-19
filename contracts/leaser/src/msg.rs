use cosmwasm_std::{Addr, Uint64};

use finance::{coin::CoinDTO, currency::SymbolOwned, liability::Liability, percent::Percent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{state::config::Config, ContractError};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub lease_code_id: Uint64,
    pub lpp_ust_addr: Addr,
    pub lease_interest_rate_margin: Percent, // LeaseInterestRateMargin%, for example 3%
    pub liability: Liability,                // LeaseMaxLiability%, for example 80%
    pub repayment: Repayment,                // GracePeriodSec, for example 10 days = 10*24*60*60
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Repayment {
    pub period_sec: u32,       // PeriodLengthSec, for example 90 days = 90*24*60*60
    pub grace_period_sec: u32, // GracePeriodSec, for example 10 days = 10*24*60*60
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Config {
        lease_interest_rate_margin: Percent,
        liability: Liability,
        repayment: Repayment,
    },
    OpenLease {
        currency: SymbolOwned,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Quote { downpayment: CoinDTO },
    Leases { owner: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub config: Config,
}

// totalUST, borrowUST, annualInterestRate%
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QuoteResponse {
    pub total: CoinDTO,
    pub borrow: CoinDTO,
    pub annual_interest_rate: Percent,
    pub annual_interest_rate_margin: Percent,
}

impl Repayment {
    pub fn new(period_sec: u32, grace_period_sec: u32) -> Self {
        Repayment {
            period_sec,
            grace_period_sec,
        }
    }

    pub fn validate_period(&self) -> Result<(), ContractError> {
        if self.period_sec > self.grace_period_sec {
            Result::Ok(())
        } else {
            Result::Err(ContractError::validation_err::<Repayment>(String::from(
                "Period length should be greater than grace period",
            )))
        }
    }
}
