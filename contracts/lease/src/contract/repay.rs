use cosmwasm_std::{Addr, Coin as CwCoin, Env, Timestamp};
use serde::Serialize;

use finance::currency::{Currency, SymbolOwned};
use lpp::stub::Lpp as LppTrait;
use platform::{
    bank,
    batch::{Emit, Emitter},
};

use crate::{
    lease::{
        Lease,
        WithLease,
        LeaseDTO
    },
    event::TYPE,
    error::ContractError,
};

pub struct Repay<'a> {
    payment: &'a [CwCoin],
    now: Timestamp,
    lease: Addr,
    env: &'a Env,
}

impl<'a> Repay<'a> {
    pub fn new(payment: &'a [CwCoin], now: Timestamp, lease: Addr, env: &'a Env) -> Self {
        Self {
            payment,
            now,
            lease,
            env,
        }
    }
}

pub struct RepayResult {
    pub lease_dto: LeaseDTO,
    pub emitter: Emitter,
}

impl<'a> WithLease for Repay<'a> {
    type Output = RepayResult;

    type Error = ContractError;

    fn exec<Lpn, Lpp>(self, mut lease: Lease<Lpn, Lpp>) -> Result<Self::Output, Self::Error>
    where
        Lpp: LppTrait<Lpn>,
        Lpn: Currency + Serialize,
    {
        // TODO 'receive' the payment from the bank using any currency it might be in
        let payment = bank::received::<Lpn>(self.payment)?;

        let receipt = lease.repay(payment, self.now, self.lease.clone())?;

        let (lease_dto, lpp) = lease.into_dto();
        let emitter = lpp
            .into()
            .into_emitter(TYPE::Repay)
            .emit_tx_info(self.env)
            .emit("to", self.lease)
            .emit("payment-symbol", Lpn::SYMBOL)
            .emit_coin_amount("payment-amount", payment)
            .emit_timestamp("at", &self.now)
            .emit_to_string_value("loan-close", receipt.close())
            .emit_coin_amount("prev-margin-interest", receipt.previous_margin_paid())
            .emit_coin_amount("prev-loan-interest", receipt.previous_interest_paid())
            .emit_coin_amount("curr-margin-interest", receipt.current_margin_paid())
            .emit_coin_amount("curr-loan-interest", receipt.current_interest_paid())
            .emit_coin_amount("principal", receipt.principal_paid());

        Ok(RepayResult { lease_dto, emitter })
    }

    fn unknown_lpn(self, symbol: SymbolOwned) -> Result<Self::Output, Self::Error> {
        Err(ContractError::UnknownCurrency { symbol })
    }
}
