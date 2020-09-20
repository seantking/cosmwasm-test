use cosmwasm_std::{
    from_slice, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage,
};

use crate::msg::{FileAddressResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State, CONFIG_KEY};
use std::collections::hash_map::{Entry, OccupiedEntry, VacantEntry};
use std::collections::HashMap;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    // Initialize the root namespace
    let mut initial_registry: HashMap<String, String> = HashMap::new();
    let state = State {
        owner: deps.api.canonical_address(&env.message.sender)?,
        address_registry: initial_registry,
        directory_name: msg.directory_name,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddFileAddress {
            ipfs_address,
            file_name,
        } => try_add_file(deps, env, ipfs_address, file_name),
    }
}

pub fn try_add_file<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    ipfs_address: String,
    file_name: String,
) -> StdResult<HandleResponse> {
    config(&mut deps.storage).update(|mut state| {
        state
            .address_registry
            .entry(file_name)
            .or_insert(ipfs_address);
        Ok(state)
    })?;

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFileAddress { file_name } => to_binary(&query_file(deps, file_name)),
    }
}

fn query_file<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    file_name: String,
) -> StdResult<FileAddressResponse> {
    let state = config_read(&deps.storage).load()?;
    let ipfs_address = &state.address_registry[&file_name];

    Ok(FileAddressResponse {
        ipfs_address: ipfs_address.to_string(),
    })
}
