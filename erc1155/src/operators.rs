//! Implementation of allowances.
use alloc::string::String;
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, URef};

use crate::{constants::OPERATORS_KEY_NAME, detail, Address};

#[inline]
/// Creates a dictionary item key for a dictionary item.
fn make_dictionary_item_key(account: Address, operator: Address) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut account.to_bytes().unwrap_or_revert());
    preimage.append(&mut operator.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    hex::encode(&key_bytes)
}

/// Get Operators uref of contract context.
pub(crate) fn operators_uref() -> URef {
    detail::get_uref(OPERATORS_KEY_NAME)
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_operator_to(
    operators_uref: URef,
    account: Address,
    operator: Address,
    approved: bool,
) {
    let dictionary_item_key = make_dictionary_item_key(account, operator);
    storage::dictionary_put(operators_uref, &dictionary_item_key, approved);
}
/// Reads an allowance for a owner and spender
pub(crate) fn read_operator_from(
    operators_uref: URef,
    account: Address,
    operator: Address,
) -> bool {
    let dictionary_item_key = make_dictionary_item_key(account, operator);
    storage::dictionary_get(operators_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
