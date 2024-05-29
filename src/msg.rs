use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub ghost_token: String,
    pub ghost_vaults: Vec<String>,
    pub threshold: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    AutoBalance {},
    Deposit {},
    Withdraw {
        amount: Option<Uint128>,
    },
}

#[cw_serde]
pub enum ReceiveMsg {
    Deposit {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
#[serde(rename = "vault_info")]
#[returns (VaultInfoResponse)]
VaultInfo {},
    //needs a return type

}

impl QueryMsg {
    pub fn vault_info_response(self) -> VaultInfoResponse {
        VaultInfoResponse {
            deposit_amount: Uint128::zero(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultInfoResponse {
    pub deposit_amount: Uint128,
}

#[derive(Serialize, Deserialize)]
pub struct VaultInfo {
    pub deposit_amount: Uint128,
}
