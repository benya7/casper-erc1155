//! Implementation of total supply.

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U256};
use alloc::{format, string::String};

use crate::{constants::TOTAL_SUPPLY_KEY_NAME, detail};


/// Creates a total supply item key for a dictionary item.
#[inline]
pub(crate) fn total_supply_key(token_id: &str) -> String {
    format!("total_supply_{}", token_id)
}

/// Get Operators uref of contract context.
pub(crate) fn total_supply_uref() -> URef {
    detail::get_uref(TOTAL_SUPPLY_KEY_NAME)
}

/// Reads a total supply from a specified [`URef`].
pub(crate) fn read_total_supply_from(total_supply_uref: URef, id: &str) -> U256 {
let dictionary_item_key = total_supply_key(&id);
    let total_supply = storage::dictionary_get::<U256>(total_supply_uref, &dictionary_item_key).unwrap_or_revert().unwrap_or_default();
    total_supply
}

/// Writes a total supply to a specific [`URef`].
pub(crate) fn write_total_supply_to(total_supply_uref: URef, id: &str, amount: U256) {
    let dictionary_item_key = total_supply_key(&id);
    storage::dictionary_put::<U256>(total_supply_uref, &dictionary_item_key, amount);
}
