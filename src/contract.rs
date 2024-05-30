#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg, from_json
};
use cw2::set_contract_version;
use cw20::Cw20ReceiveMsg;
use kujira::ghost::basic_vault::query::*;
use kujira::ghost::basic_vault::execute::*;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:juta";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        ghost_token: deps.api.addr_validate(&msg.ghost_token)?,
        ghost_vaults: msg
            .ghost_vaults
            .into_iter()
            .map(|v| deps.api.addr_validate(&v))
            .collect::<StdResult<Vec<Addr>>>()?,
        threshold: msg.threshold,
        count: msg.count,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => receive_cw20(deps, info, msg),
        ExecuteMsg::AutoBalance {} => auto_balance(deps),
        ExecuteMsg::Deposit {} => execute_deposit(deps, info),
        ExecuteMsg::Withdraw { amount } => {
            let amount = amount.expect("Expected Uint128, found None");
            execute_withdraw(deps, info, amount)
        },
        ExecuteMsg::Reset { count } => {
            let mut config: Config = CONFIG.load(deps.storage)?;
            config.count = count;
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::new())
        },
        ExecuteMsg::Increment {} => {
            let mut config: Config = CONFIG.load(deps.storage)?;
            config.count += 1;
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::new())
        },
    }
}

pub fn receive_cw20(
    deps: DepsMut,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&cw20_msg.msg)? {
        ReceiveMsg::Deposit {} => {
            let config = CONFIG.load(deps.storage)?;
            if config.ghost_token != deps.api.addr_validate(info.sender.as_str())? {
                return Err(ContractError::Unauthorized {});
            }
            let amount = cw20_msg.amount;
            auto_deposit(deps, amount)
        }
    }
}

fn auto_deposit(deps: DepsMut, amount: Uint128) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let vaults = config.ghost_vaults;
    let vault_count = vaults.len() as u128;
    let deposit_amount = amount.checked_div(Uint128::from(vault_count))?;
    let mut messages = vec![];
    for vault in &vaults {
        let msg = DepositMsg { callback: None };
        messages.push(WasmMsg::Execute {
            contract_addr: vault.to_string(),
            msg: to_json_binary(&msg)?,
            funds: vec![],
        });
    }
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "auto_deposit")
        .add_attribute("amount", amount))
}

fn auto_balance(deps: DepsMut) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let vaults = config.ghost_vaults;
    let mut total_deposits = Uint128::zero();
    let mut balances = vec![];
    for vault in &vaults {
        let query_msg = QueryMsg::VaultInfo {};
        let vault_info: StatusResponse = deps.querier.query_wasm_smart(vault, &query_msg)?;
        let balance = vault_info.deposited;
        balances.push(balance);
        total_deposits += balance;
    }
    let target_balance = total_deposits.checked_div(Uint128::from(vaults.len() as u128))?;
    let mut messages = vec![];
    for (i, balance) in balances.iter().enumerate() {
        if balance > &(target_balance + config.threshold) {
            let withdraw_amount = *balance - target_balance;
            let msg = WithdrawMsg {
                amount: withdraw_amount,
                callback: None,
            };
            messages.push(WasmMsg::Execute {
                contract_addr: vaults[i].to_string(),
                msg: to_json_binary(&msg)?,
                funds: vec![],
            });
        } else if balance < &(target_balance - config.threshold) {
            let deposit_amount = target_balance - *balance;
            let msg = DepositMsg { callback: None };
            messages.push(WasmMsg::Execute {
                contract_addr: vaults[i].to_string(),
                msg: to_json_binary(&msg)?,
                funds: vec![],
            });
        }
    }
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "auto_balance"))
}

fn execute_deposit(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let vaults = config.ghost_vaults;
    let mut messages = vec![];
    for vault in &vaults {
        let msg = DepositMsg { callback: None };
        messages.push(WasmMsg::Execute {
            contract_addr: vault.to_string(),
            msg: to_json_binary(&msg)?,
            funds: info.funds.clone(),
        });
    }
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "deposit"))
}

fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let vaults = config.ghost_vaults;
    let vault_count = vaults.len() as u128;
    let withdraw_amount = amount.checked_div(Uint128::from(vault_count))?;
    let mut messages = vec![];
    for vault in &vaults {
        let msg = WithdrawMsg {
            amount: withdraw_amount,
            callback: None,
        };
        messages.push(WasmMsg::Execute {
            contract_addr: vault.to_string(),
            msg: to_json_binary(&msg)?,
            funds: vec![],
        });
    }
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "withdraw")
        .add_attribute("amount", amount))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::VaultInfo {} => {
            let config = CONFIG.load(deps.storage)?;
            let vaults = config.ghost_vaults;
            let mut vault_infos = vec![];
            for vault in &vaults {
                let query_msg = kujira::ghost::basic_vault::query::QueryMsg::Status {};
                let vault_info: StatusResponse = deps.querier.query_wasm_smart(vault, &query_msg)?;
                vault_infos.push(vault_info);
            }
            to_json_binary(&vault_infos)
        },
        QueryMsg::GetCount {} => {
            let config = CONFIG.load(deps.storage)?;
            to_json_binary(&config.count)
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use crate::state::CONFIG;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json, Uint128};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            ghost_token: "token".to_string(),
            ghost_vaults: vec![],
            threshold: Uint128::zero(),
            count: 17,
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        let config = CONFIG.load(&deps.storage).unwrap();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: i32 = from_json(&res).unwrap();
        assert_eq!(17, config.count);
        assert_eq!(17, value);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            ghost_token: "token".to_string(),
            ghost_vaults: vec![],
            threshold: Uint128::zero(),
            count: 17,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: i32 = from_json(&res).unwrap();
        assert_eq!(18, value);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            ghost_token: "token".to_string(),
            ghost_vaults: vec![],
            threshold: Uint128::zero(),
            count: 17,
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {},
            _ => panic!("Must return unauthorized error"),
        }
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: i32 = from_json(&res).unwrap();
        assert_eq!(5, value);
    }
}
