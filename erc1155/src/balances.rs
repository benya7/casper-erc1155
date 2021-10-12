//! Implementation of balances.
use alloc::string::String;
use alloc::vec::Vec;
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{constants::BALANCES_KEY_NAME, detail, Address};

/// Creates a dictionary item key for a dictionary item.
#[inline]
fn make_dictionary_item_key(id: &str, account: Address) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut id.to_bytes().unwrap_or_revert());
    preimage.append(&mut account.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    base64::encode(&key_bytes)
}
/// Get Balances uref of contract context.
pub(crate) fn get_balances_uref() -> URef {
    detail::get_uref(BALANCES_KEY_NAME)
}
/// Writes token balance of a specified account into a dictionary.
pub(crate) fn write_balance_to(
    balances_uref: URef,
    address: Address,
    token_id: &str,
    amount: U256,
) {
    let dictionary_item_key = make_dictionary_item_key(token_id, address);
    storage::dictionary_put(balances_uref, &dictionary_item_key, amount);
}
/// Read token balance of a specified account into a dictionary.
pub(crate) fn read_balance_from(balances_uref: URef, account: Address, token_id: &str) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(token_id, account);
    storage::dictionary_get(balances_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
