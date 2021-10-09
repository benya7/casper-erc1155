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
mod allowances;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
// mod total_supply;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, EntryPoints, Key, URef, U256};

pub use address::Address;
use constants::{ALLOWANCES_KEY_NAME, ERC1155_TOKEN_CONTRACT_KEY_NAME, TOKENS_KEY_NAME};
pub use error::Error;

/// Implementation of ERC1155 standard functionality.
#[derive(Default)]
pub struct ERC1155 {
    tokens_uref: OnceCell<URef>,
    allowances_uref: OnceCell<URef>,
}
impl ERC1155 {
    fn new(tokens_uref: URef, allowances_uref: URef) -> Self {
        Self {
            tokens_uref: tokens_uref.into(),
            allowances_uref: allowances_uref.into(),
        }
    }

    // fn total_supply_uref(&self) -> URef {
    //     *self
    //         .total_supply_uref
    //         .get_or_init(total_supply::total_supply_uref)
    // }

    // fn read_total_supply(&self) -> U256 {
    //     total_supply::read_total_supply_from(self.total_supply_uref())
    // }

    // fn write_total_supply(&self, total_supply: U256) {
    //     total_supply::write_total_supply_to(self.total_supply_uref(), total_supply)
    //

    fn tokens_uref(&self) -> URef {
        *self.tokens_uref.get_or_init(balances::get_tokens_uref)
    }

    fn allowances_uref(&self) -> URef {
        *self
            .allowances_uref
            .get_or_init(allowances::allowances_uref)
    }

    fn read_balance(&self, token_id: &String, owner: Address) -> U256 {
        balances::read_balance(self.tokens_uref(), &token_id, owner)
    }

    // fn write_balance(&mut self, token_id: &str, owner: Address, amount: U256) {
    //     balances::write_balance(self.tokens_uref(), token_id, owner, amount)
    // }

    fn read_allowance(&self, owner: Address, spender: Address) -> bool {
        allowances::read_allowance(self.allowances_uref(), owner, spender)
    }

    fn write_allowance(&mut self, owner: Address, spender: Address, approved: bool) {
        allowances::write_allowance(self.allowances_uref(), owner, spender, approved)
    }

    fn transfer_balance(
        &mut self,
        from: Address,
        token_id: String,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        assert_eq!(sender == from || self.read_allowance(from, recipient), true);
        balances::transfer_balance(self.tokens_uref(), sender, token_id, recipient, amount)
    }

    /// Installs the ERC1155 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install() -> Result<ERC1155, Error> {
        let default_entry_points = entry_points::default();
        ERC1155::install_custom(ERC1155_TOKEN_CONTRACT_KEY_NAME, default_entry_points)
    }

    // /// Returns the name of the token.
    // pub fn name(&self) -> String {
    //     detail::read_from(NAME_KEY_NAME)
    // }

    // /// Returns the symbol of the token.
    // pub fn symbol(&self) -> String {
    //     detail::read_from(SYMBOL_KEY_NAME)
    // }

    // /// Returns the decimals of the token.
    // pub fn decimals(&self) -> u8 {
    //     detail::read_from(DECIMALS_KEY_NAME)
    // }

    /// Returns the balance of `owner`.
    pub fn balance_of(&self, token_id: String, owner: Address) -> U256 {
        let id = token_id.clone();
        self.read_balance(&id, owner)
    }
    /// Returns the balances of `accounts`.
    pub fn balance_of_batch(&self, token_ids: Vec<String>, accounts: Vec<Address>) -> Vec<U256> {
        assert_eq!(token_ids.len(), accounts.len());
        let mut balances: Vec<U256> = Vec::new();
        for (i, _) in accounts.iter().enumerate() {
            let id = token_ids[i].clone();
            let balance = self.balance_of(id, accounts[i]);
            balances.push(balance);
        }
        balances
    }
    /// Grants or revokes permission to operator to transfer the caller’s tokens, according to approved.
    pub fn set_approval_for_all(&mut self, operator: Address, approved: bool) -> Result<(), Error> {
        let owner = detail::get_immediate_caller_address()?;
        self.write_allowance(owner, operator, approved);
        Ok(())
    }
    /// Returns true if operator is approved to transfer account's tokens.
    pub fn is_approval_for_all(&self, owner: Address, spender: Address) -> bool {
        self.read_allowance(owner, spender)
    }
    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn safe_transfer_from(
        &mut self,
        token_id: &String,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        let balance = self.read_balance(token_id, sender);
        let id = token_id.clone();
        assert_eq!(balance > amount, true);
        self.transfer_balance(sender, id, recipient, amount)
    }
    /// Batched version of safe_transfer_from.
    pub fn safe_transfer_from_batch(
        &mut self,
        recipient: Address,
        token_ids: Vec<String>,
        amounts: Vec<U256>,
    ) -> Result<(), Error> {
        assert_eq!(token_ids.len(), amounts.len());
        let sender = detail::get_immediate_caller_address()?;

        for (i, _) in token_ids.iter().enumerate() {
            let balance = self.read_balance(&token_ids[i], sender);
            let id = token_ids[i].clone();
            assert_eq!(balance > amounts[i], true);
            self.transfer_balance(sender, id, recipient, amounts[i])?;
        }
        Ok(())
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    // pub fn mint(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
    //     let new_balance = {
    //         let balance = self.read_balance(owner);
    //         balance.checked_add(amount).ok_or(Error::Overflow)?
    //     };
    //     let new_total_supply = {
    //         let total_supply: U256 = self.read_total_supply();
    //         total_supply.checked_add(amount).ok_or(Error::Overflow)?
    //     };
    //     self.write_balance(owner, new_balance);
    //     self.write_total_supply(new_total_supply);
    //     Ok(())
    // }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total
    /// supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    // pub fn burn(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
    //     let new_balance = {
    //         let balance = self.read_balance(owner);
    //         balance
    //             .checked_sub(amount)
    //             .ok_or(Error::InsufficientBalance)?
    //     };
    //     let new_total_supply = {
    //         let total_supply = self.read_total_supply();
    //         total_supply.checked_sub(amount).ok_or(Error::Overflow)?
    //     };
    //     self.write_balance(owner, new_balance);
    //     self.write_total_supply(new_total_supply);
    //     Ok(())
    // }

    /// Installs the ERC1155 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC1155::install`] instead, as it will create the default set
    /// of ERC1155 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC1155, Error> {
        let tokens_uref = storage::new_dictionary(TOKENS_KEY_NAME).unwrap_or_revert();
        let allowances_uref = storage::new_dictionary(ALLOWANCES_KEY_NAME).unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        // let total_supply_key = Key::from(total_supply_uref);

        let tokens_dictionary_key = {
            runtime::remove_key(TOKENS_KEY_NAME);
            Key::from(tokens_uref)
        };

        let allowances_dictionary_key = {
            runtime::remove_key(ALLOWANCES_KEY_NAME);
            Key::from(allowances_uref)
        };

        named_keys.insert(TOKENS_KEY_NAME.to_string(), tokens_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY_NAME.to_string(), allowances_dictionary_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC1155::new(tokens_uref, allowances_uref))
    }
}