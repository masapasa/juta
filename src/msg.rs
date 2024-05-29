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
    pub count: i32
}

#[cw_serde]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    AutoBalance {},
    Deposit {},
    Withdraw {
        amount: Option<Uint128>,
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

#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[serde(rename = "vault_info")]
    #[returns (VaultInfoResponse)]
    VaultInfo {},
    #[serde(rename = "get_count")]
    #[returns (GetCount)]
    GetCount {},
}

impl QueryMsg {
    pub fn vault_info_response(self) -> VaultInfoResponse {
        VaultInfoResponse {
            deposit_amount: Uint128::zero(),
        }
    }

    pub fn get_count_response(self, count: i32) -> GetCountResponse {
        GetCountResponse {
            count,
        }
    }
    pub fn get_count(self) -> GetCount {
        GetCount { count: 0 }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetCountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GetCount {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultInfoResponse {
    pub deposit_amount: Uint128,
}

#[derive(Serialize, Deserialize)]
pub struct VaultInfo {
    pub deposit_amount: Uint128,
}
