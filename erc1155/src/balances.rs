//! Implementation of balances.
use alloc::string::String;
use alloc::vec::Vec;
use casper_contract::{contract_api::{ storage, runtime}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{
    constants::{BALANCES_KEY_NAME},
    detail,
    error::Error,
    Address,
};

/// Creates a dictionary item key for a dictionary item.
#[inline]
fn make_dictionary_item_key(id: &str, account: Address) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut id.to_bytes().unwrap_or_revert());
    preimage.append(&mut account.to_bytes().unwrap_or_revert());

    let key_bytes = runtime::blake2b(&preimage);
    base64::encode(&key_bytes)
}
/// Get Balance uref of contract context.
pub(crate) fn get_balances_uref() -> URef {
    detail::get_uref(BALANCES_KEY_NAME)
}
/// Writes token balance of a specified account into a dictionary.
pub(crate) fn write_balance_to(balances_uref: URef, address: Address, token_id: &str, amount: U256) {
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

/// Transfer tokens from the `sender` to the `recipient`.
///
/// This function should not be used directly by contract's entrypoint as it does not validate the
/// sender.
pub(crate) fn transfer_balance(
    balances_uref: URef,
    from: Address,
    to: Address,
    id: &str,
    amount: U256,
) -> Result<(), Error> {
    let sender_balance = read_balance_from(balances_uref, from, id);
        sender_balance
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance)?;
    let recipient_balance = read_balance_from(balances_uref, to, id);
        recipient_balance
            .checked_add(amount)
            .ok_or(Error::Overflow)?;
    write_balance_to(balances_uref, from, id, sender_balance);
    write_balance_to(balances_uref, to, id, recipient_balance);

    Ok(())
}