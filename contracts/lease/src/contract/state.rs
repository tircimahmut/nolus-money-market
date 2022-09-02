use cosmwasm_std::{to_binary, Addr, Binary, Timestamp};
use serde::Serialize;

use finance::currency::{Currency, SymbolOwned};
use lpp::stub::Lpp as LppTrait;
use market_price_oracle::stub::Oracle as OracleTrait;
use platform::bank::BankAccount;

use crate::{
    error::ContractError,
    lease::{Lease, WithLease},
};

pub struct LeaseState<Bank> {
    now: Timestamp,
    account: Bank,
    lease: Addr,
}

impl<Bank> LeaseState<Bank> {
    pub fn new(now: Timestamp, account: Bank, lease: Addr) -> Self {
        Self {
            now,
            account,
            lease,
        }
    }
}

impl<Bank> WithLease for LeaseState<Bank>
where
    Bank: BankAccount,
{
    type Output = Binary;

    type Error = ContractError;

    fn exec<Lpn, Lpp, Oracle>(
        self,
        lease: Lease<Lpn, Lpp, Oracle>,
    ) -> Result<Self::Output, Self::Error>
    where
        Lpn: Currency + Serialize,
        Lpp: LppTrait<Lpn>,
        Oracle: OracleTrait<Lpn>,
    {
        let resp = lease.state(self.now, &self.account, self.lease)?;
        to_binary(&resp).map_err(ContractError::from)
    }

    fn unknown_lpn(self, symbol: SymbolOwned) -> Result<Self::Output, Self::Error> {
        Err(ContractError::UnknownCurrency { symbol })
    }
}
