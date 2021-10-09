//! Implementation of allowances.
use alloc::{string::String};

use casper_contract::{
    contract_api::{storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, URef};

use crate::{constants::ALLOWANCES_KEY_NAME, detail, Address};

#[inline]
pub(crate) fn allowances_uref() -> URef {
    detail::get_uref(ALLOWANCES_KEY_NAME)
}

/// Creates a dictionary item key for an (owner, spender) pair.
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    // NOTE: As for now dictionary item keys are limited to 64 characters only. Instead of using
    // hashing (which will effectively hash a hash) we'll use base64. Preimage is about 33 bytes for
    // both Address variants, and approximated base64-encoded length will be 4 * (33 / 3) ~ 44
    // characters.
    // Even if the preimage increased in size we still have extra space but even in case of much
    // larger preimage we can switch to base85 which has ratio of 4:5.
    base64::encode(&preimage)
}

/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_allowance(
    allowances_uref: URef,
    owner: Address,
    spender: Address,
    approved: bool,
) {
    let dictionary_item_key = make_dictionary_item_key(owner);
    let allowance_uref = storage::dictionary_get::<URef>(allowances_uref, &dictionary_item_key)
    .unwrap_or_revert()
    .unwrap_or_default();
    write_allowance_to(allowance_uref, spender, approved);
}
/// Writes an allowance for owner and spender for a specific amount.
pub(crate) fn write_allowance_to(
    allowance_uref: URef,
    spender: Address,
    approved: bool,
) {
    let dictionary_item_key = make_dictionary_item_key(spender);
    storage::dictionary_put(allowance_uref, &dictionary_item_key, approved);
}

/// Reads an allowance for a owner and spender
pub(crate) fn read_allowance(allowances_uref: URef, owner: Address, spender: Address) -> bool {
    let dictionary_item_key = make_dictionary_item_key(owner);
    let allowance_uref = storage::dictionary_get::<URef>(allowances_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default();
  read_allowance_from(allowance_uref, spender)
    
}
/// Reads an allowance for a owner and spender
pub(crate) fn read_allowance_from(allowance_uref: URef, spender: Address) -> bool {
    let dictionary_item_key = make_dictionary_item_key(spender);
    storage::dictionary_get::<bool>(allowance_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}
