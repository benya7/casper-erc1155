//! A library for developing ERC1155 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC1155`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC1155 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc1155 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod address;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
mod operators;
mod total_supply;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, EntryPoints, Key, URef, U256};

pub use address::Address;
use constants::{
    BALANCES_KEY_NAME, ERC1155_TOKEN_CONTRACT_KEY_NAME, OPERATORS_KEY_NAME, TOTAL_SUPPLY_KEY_NAME,
    URI_KEY_NAME,
};
pub use error::Error;

/// Implementation of ERC1155 standard functionality.
#[derive(Default)]
pub struct ERC1155 {
    balances_uref: OnceCell<URef>,
    operators_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
}

impl ERC1155 {
    fn new(balances_uref: URef, operators_uref: URef, total_supply_uref: URef) -> Self {
        Self {
            balances_uref: balances_uref.into(),
            operators_uref: operators_uref.into(),
            total_supply_uref: total_supply_uref.into(),
        }
    }

    fn balances_uref(&self) -> URef {
        *self.balances_uref.get_or_init(balances::get_balances_uref)
    }

    fn operators_uref(&self) -> URef {
        *self.operators_uref.get_or_init(operators::operators_uref)
    }

    fn total_supply_uref(&self) -> URef {
        *self
            .total_supply_uref
            .get_or_init(total_supply::total_supply_uref)
    }

    fn read_total_supply(&self, id: &str) -> U256 {
        total_supply::read_total_supply_from(self.total_supply_uref(), &id)
    }

    fn write_total_supply(&self, id: &str, amount: U256) {
        total_supply::write_total_supply_to(self.total_supply_uref(), &id, amount)
    }

    fn read_balance(&self, account: Address, token_id: &str) -> U256 {
        balances::read_balance_from(self.balances_uref(), account, &token_id)
    }

    fn write_balance(&mut self, to: Address, token_id: &str, amount: U256) {
        balances::write_balance_to(self.balances_uref(), to, &token_id, amount)
    }

    fn read_operator(&self, owner: Address, spender: Address) -> bool {
        operators::read_operator_from(self.operators_uref(), owner, spender)
    }

    fn write_operator(&mut self, owner: Address, spender: Address, approved: bool) {
        operators::write_operator_to(self.operators_uref(), owner, spender, approved)
    }

    /// Installs the ERC1155 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(uri: String) -> Result<ERC1155, Error> {
        let default_entry_points = entry_points::default();
        ERC1155::install_custom(uri, ERC1155_TOKEN_CONTRACT_KEY_NAME, default_entry_points)
    }

    /// Returns the URI of the token.
    pub fn uri(&self) -> String {
        detail::read_from(URI_KEY_NAME)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self, id: &str) -> U256 {
        self.read_total_supply(&id)
    }

    /// Returns the balance of `account`.
    pub fn balance_of(&self, account: Address, id: &str) -> U256 {
        self.read_balance(account, id)
    }

    /// Returns the balances of `accounts`.
    pub fn balance_of_batch(&self, accounts: Vec<Address>, ids: Vec<String>) -> Vec<U256> {
        assert_eq!(ids.len(), accounts.len());
        let mut balances: Vec<U256> = Vec::new();
        for (i, _) in accounts.iter().enumerate() {
            let id: &str = &ids[i];
            let balance = self.balance_of(accounts[i], id);
            balances.push(balance);
        }
        balances
    }

    /// Grants or revokes permission to operator to transfer the caller’s tokens, according to approved.
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) -> Result<(), Error> {
        let owner = detail::get_immediate_caller_address()?;
        self.write_operator(owner, operator, approved);
        Ok(())
    }

    /// Returns true if operator is approved to transfer account's tokens.
    pub fn is_approval_for_all(&self, account: Address, operator: Address) -> bool {
        self.read_operator(account, operator)
    }

    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn safe_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        id: &str,
        amount: U256,
    ) -> Result<(), Error> {
        let spender = detail::get_immediate_caller_address()?;
        let operator = self.read_operator(from, spender);
        if (from != spender && !operator) || amount == U256::zero() || from == to {
            return Ok(());
        } else {
            let sender_balance = {
                let balance = self.read_balance(from, &id);
                balance
                    .checked_sub(amount)
                    .ok_or(Error::InsufficientBalance)?
            };
            let recipient_balance = {
                let balance = self.read_balance(to, &id);
                balance.checked_add(amount).ok_or(Error::Overflow)?
            };
            self.write_balance(from, &id, sender_balance);
            self.write_balance(to, &id, recipient_balance);
            Ok(())
        }
    }

    /// Batched version of safe_transfer_from.
    pub fn safe_batch_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        ids: Vec<String>,
        amounts: Vec<U256>,
    ) -> Result<(), Error> {
        let spender = detail::get_immediate_caller_address()?;
        let operator = self.read_operator(from, spender);
        if (from != spender && !operator) || from == to {
            return Ok(());
        } else {
            for (i, _) in ids.iter().enumerate() {
                let sender_balance = {
                    let balance = self.read_balance(from, &ids[i]);
                    balance
                        .checked_sub(amounts[i])
                        .ok_or(Error::InsufficientBalance)?
                };
                let recipient_balance = {
                    let balance = self.read_balance(to, &ids[i]);
                    balance.checked_add(amounts[i]).ok_or(Error::Overflow)?
                };
                self.write_balance(from, &ids[i], sender_balance);
                self.write_balance(to, &ids[i], recipient_balance);
            }
            Ok(())
        }
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    /// # Security
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn mint(&mut self, to: Address, id: &str, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(to, &id);
            balance.checked_add(amount).ok_or(Error::Overflow)?
        };
        let new_total_supply = {
            let total_supply = self.read_total_supply(&id);
            total_supply.checked_add(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(to, &id, new_balance);
        self.write_total_supply(&id, new_total_supply);
        Ok(())
    }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total supply.
    /// # Security
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn burn(&mut self, owner: Address, id: &str, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner, &id);
            balance
                .checked_sub(amount)
                .ok_or(Error::InsufficientBalance)?
        };
        let new_total_supply = {
            let total_supply = self.read_total_supply(&id);
            total_supply.checked_sub(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, &id, new_balance);
        self.write_total_supply(&id, new_total_supply);
        Ok(())
    }

    /// Installs the ERC1155 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC1155::install`] instead, as it will create the default set
    /// of ERC1155 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        uri: String,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC1155, Error> {
        let balances_uref = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
        let operators_uref = storage::new_dictionary(OPERATORS_KEY_NAME).unwrap_or_revert();
        let total_supply_uref = storage::new_dictionary(TOTAL_SUPPLY_KEY_NAME).unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        let uri_key = {
            let uri_uref = storage::new_uref(uri).into_read();
            Key::from(uri_uref)
        };
        let balances_dictionary_key = {
            runtime::remove_key(BALANCES_KEY_NAME);
            Key::from(balances_uref)
        };
        let operators_dictionary_key = {
            runtime::remove_key(OPERATORS_KEY_NAME);
            Key::from(operators_uref)
        };
        let total_supply_key = {
            runtime::remove_key(TOTAL_SUPPLY_KEY_NAME);
            Key::from(total_supply_uref)
        };
        named_keys.insert(URI_KEY_NAME.to_string(), uri_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(OPERATORS_KEY_NAME.to_string(), operators_dictionary_key);
        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC1155::new(
            balances_uref,
            operators_uref,
            total_supply_uref,
        ))
    }
}
