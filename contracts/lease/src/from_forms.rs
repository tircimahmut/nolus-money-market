use cosmwasm_std::{StdResult, Api};

use crate::{opening::{LoanForm, NewLeaseForm}, loan::Loan, lease::Lease};

impl LoanForm {
    pub fn into(self, api: &dyn Api) -> StdResult<Loan> {
        let lpp = api.addr_validate(&self.lpp)?;
        let _lpp_stub = lpp::stub::LppStub::from(lpp.clone());
        // lpp_stub.create_open_loan_msg();
        Ok(Loan::new(
            self.annual_margin_interest_permille,
            lpp,
            self.interest_due_period_secs,
            self.grace_period_secs,
        ))
    }
}

impl NewLeaseForm {
    pub fn into(self, api: &dyn Api) -> StdResult<Lease> {
        let customer = api.addr_validate(&self.customer)?;
        Ok(Lease::new(
            customer,
            self.currency,
            self.liability,
            self.loan.into(api)?,
        ))
    }
}
