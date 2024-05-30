use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub ghost_token: String,
    pub ghost_vaults: Vec<String>,
    pub threshold: Uint128,
    pub count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    AutoBalance {},
    Deposit {},
    Withdraw {amount: Option<Uint128>,
    },
    Reset {
        count: i32,
    },
    Increment {},
}

#[cw_serde]
pub enum ReceiveMsg {
    Deposit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    VaultInfo {},
    GetCount {},
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub ghost_token: Addr,
    pub ghost_vaults: Vec<Addr>,
    pub threshold: Uint128,
    pub count: u64,
}
