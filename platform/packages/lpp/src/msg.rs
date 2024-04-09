// use oracle_platform::OracleRef;
use serde::{Deserialize, Serialize};

/// The query message variants each Lpp must implement
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return the total value of a pool in a stable currency as [CoinStable]
    /// // TODO oracle: OracleRef
    StableBalance {},
}

/// The execute message variants each Lpp must implement
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ExecuteMsg {
    DistributeRewards(),
}
