#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::{string::String, vec::Vec};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc1155::{
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, AMOUNTS_RUNTIME_ARG_NAME,
        AMOUNT_RUNTIME_ARG_NAME, APPROVED_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME,
        OPERATOR_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        TOKEN_IDS_RUNTIME_ARG_NAME, TOKEN_ID_RUNTIME_ARG_NAME, URI_RUNTIME_ARG_NAME,
    },
    Address, ERC1155,
};
use casper_types::{CLValue, U256};

#[no_mangle]
pub extern "C" fn uri() {
    let uri = ERC1155::default().uri();
    runtime::ret(CLValue::from_t(uri).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let total_supply = ERC1155::default().total_supply(&id);
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let balance = ERC1155::default().balance_of(account, &id);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<Address> = runtime::get_named_arg(ACCOUNTS_RUNTIME_ARG_NAME);
    let ids: Vec<String> = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let balance = ERC1155::default().balance_of_batch(accounts, ids);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_approval_for_all() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved: bool = runtime::get_named_arg(APPROVED_RUNTIME_ARG_NAME);
    ERC1155::default()
        .set_approval_for_all(operator, approved)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn is_approval_for_all() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved = ERC1155::default().is_approval_for_all(account, operator);
    runtime::ret(CLValue::from_t(approved).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn safe_transfer_from() {
    let from: Address = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default()
        .safe_transfer_from(from, to, &id, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn safe_batch_transfer_from() {
    let from: Address = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let ids: Vec<String> = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let amounts: Vec<U256> = runtime::get_named_arg(AMOUNTS_RUNTIME_ARG_NAME);
    ERC1155::default()
        .safe_batch_transfer_from(from, to, ids, amounts)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn mint() {
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default().mint(to, &id, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default()
        .burn(owner, &id, amount)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let uri = runtime::get_named_arg(URI_RUNTIME_ARG_NAME);
    let _token = ERC1155::install(uri).unwrap_or_revert();
}
