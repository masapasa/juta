use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub ghost_token: Addr,
    pub ghost_vaults: Vec<Addr>,
    pub threshold: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
