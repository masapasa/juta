use cosmwasm_schema::write_api;
use juta::msg::{ExecuteMsg, InstantiateMsg};
use kujira::ghost::basic_vault::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
