use crate::asset::{Asset, AssetInfo, PairInfo};
use crate::factory::{NativeTokenDecimalsResponse, QueryMsg as FactoryQueryMsg};
use crate::pair::{QueryMsg as PairQueryMsg, ReverseSimulationResponse, SimulationResponse};

use injective_cosmwasm::querier::InjectiveQuerier;
use injective_cosmwasm::tokenfactory::response::TokenFactoryDenomSupplyResponse;
use injective_cosmwasm::query::InjectiveQueryWrapper;

use cosmwasm_std::{
    to_json_binary, Addr, AllBalanceResponse, BalanceResponse, BankQuery, Coin, CustomQuery, DepsMut, QuerierWrapper, QueryRequest, StdResult, Uint128, WasmQuery
};

use cw20::{BalanceResponse as Cw20BalanceResponse, Cw20QueryMsg, TokenInfoResponse};

pub fn query_balance<Q: CustomQuery>(
    querier: &QuerierWrapper<Q>,
    account_addr: Addr,
    denom: String,
) -> StdResult<Uint128> {
    // load price form the oracle
    let balance: BalanceResponse = querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount)
}

pub fn query_all_balances(querier: &QuerierWrapper, account_addr: Addr) -> StdResult<Vec<Coin>> {
    // load price form the oracle
    let all_balances: AllBalanceResponse =
        querier.query(&QueryRequest::Bank(BankQuery::AllBalances {
            address: account_addr.to_string(),
        }))?;
    Ok(all_balances.amount)
}

pub fn query_token_balance<Q: CustomQuery>(
    querier: &QuerierWrapper<Q>,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint128> {
    let res: Cw20BalanceResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_json_binary(&Cw20QueryMsg::Balance {
            address: account_addr.to_string(),
        })?,
    }))?;

    // load balance form the token contract
    Ok(res.balance)
}

pub fn query_token_info(
    querier: &QuerierWrapper,
    contract_addr: Addr,
) -> StdResult<TokenInfoResponse> {
    let token_info: TokenInfoResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(),
        msg: to_json_binary(&Cw20QueryMsg::TokenInfo {})?,
    }))?;

    Ok(token_info)
}

pub fn query_token_factory_denom_total_supply(
    deps: DepsMut<InjectiveQueryWrapper>,
    denom: String,
) -> StdResult<Uint128> {
    let querier: InjectiveQuerier<'_> = InjectiveQuerier::new(&deps.querier);
    let query_msg: TokenFactoryDenomSupplyResponse = querier.query_token_factory_denom_total_supply(&denom).unwrap();
    let total_share: Uint128 = query_msg.total_supply;
    Ok(total_share)
}

pub fn query_native_decimals(
    querier: &QuerierWrapper,
    factory_contract: Addr,
    denom: String,
) -> StdResult<u8> {
    let res: NativeTokenDecimalsResponse =
        querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: factory_contract.to_string(),
            msg: to_json_binary(&FactoryQueryMsg::NativeTokenDecimals { denom })?,
        }))?;
    Ok(res.decimals)
}

pub fn query_pair_info(
    querier: &QuerierWrapper,
    factory_contract: Addr,
    asset_infos: &[AssetInfo; 2],
) -> StdResult<PairInfo> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: factory_contract.to_string(),
        msg: to_json_binary(&FactoryQueryMsg::Pair {
            asset_infos: asset_infos.clone(),
        })?,
    }))
}

pub fn simulate(
    querier: &QuerierWrapper,
    pair_contract: Addr,
    offer_asset: &Asset,
) -> StdResult<SimulationResponse> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pair_contract.to_string(),
        msg: to_json_binary(&PairQueryMsg::Simulation {
            offer_asset: offer_asset.clone(),
        })?,
    }))
}

pub fn reverse_simulate(
    querier: &QuerierWrapper,
    pair_contract: Addr,
    ask_asset: &Asset,
) -> StdResult<ReverseSimulationResponse> {
    querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pair_contract.to_string(),
        msg: to_json_binary(&PairQueryMsg::ReverseSimulation {
            ask_asset: ask_asset.clone(),
        })?,
    }))
}

pub fn query_pair_info_from_pair(
    querier: &QuerierWrapper,
    pair_contract: Addr,
) -> StdResult<PairInfo> {
    let pair_info: PairInfo = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: pair_contract.to_string(),
        msg: to_json_binary(&PairQueryMsg::Pair {})?,
    }))?;

    Ok(pair_info)
}
