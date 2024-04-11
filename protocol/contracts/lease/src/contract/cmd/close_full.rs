use currency::Currency;
use lpp::stub::loan::LppLoan as LppLoanTrait;
use oracle::stub::Oracle as OracleTrait;
use platform::{bank::FixedAddressSender, message::Response as MessageResponse};
use sdk::cosmwasm_std::Timestamp;

use crate::{
    error::ContractError,
    finance::{LpnCoinDTO, LpnCurrencies, LpnCurrency, ReserveRef},
    lease::{with_lease::WithLease, Lease},
};

use super::repayable::Emitter;

pub(crate) struct Close<ProfitSender, ChangeSender, EmitterT> {
    payment: LpnCoinDTO,
    now: Timestamp,
    profit: ProfitSender,
    reserve: ReserveRef,
    change: ChangeSender,
    emitter_fn: EmitterT,
}

impl<ProfitSender, ChangeSender, EmitterT> Close<ProfitSender, ChangeSender, EmitterT> {
    pub fn new(
        payment: LpnCoinDTO,
        now: Timestamp,
        profit: ProfitSender,
        reserve: ReserveRef,
        change: ChangeSender,
        emitter_fn: EmitterT,
    ) -> Self {
        Self {
            payment,
            now,
            profit,
            reserve,
            change,
            emitter_fn,
        }
    }
}

impl<ProfitSender, ChangeSender, EmitterT> WithLease for Close<ProfitSender, ChangeSender, EmitterT>
where
    ProfitSender: FixedAddressSender,
    ChangeSender: FixedAddressSender,
    EmitterT: Emitter,
{
    type Output = MessageResponse;

    type Error = ContractError;

    fn exec<Asset, Lpp, Oracle>(
        self,
        lease: Lease<Asset, Lpp, Oracle>,
    ) -> Result<Self::Output, Self::Error>
    where
        Asset: Currency,
        Lpp: LppLoanTrait<LpnCurrency, LpnCurrencies>,
        Oracle: OracleTrait<LpnCurrency>,
    {
        let lease_addr = lease.addr().clone();

        self.payment
            .try_into()
            .map_err(Into::into)
            .and_then(|payment| {
                lease.close_full(
                    payment,
                    self.now,
                    self.profit,
                    self.reserve.into_reserve(),
                    self.change,
                )
            })
            .map(|result| {
                let (receipt, messages) = result.decompose();
                MessageResponse::messages_with_events(
                    messages,
                    self.emitter_fn.emit(&lease_addr, &receipt),
                )
            })
    }
}
