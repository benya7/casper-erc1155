#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::ops::{Deref, DerefMut};

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc1155::{
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME,
        BURN_ENTRY_POINT_NAME, MINT_ENTRY_POINT_NAME, OPERATOR_RUNTIME_ARG_NAME,
        OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME, TOKEN_IDS_RUNTIME_ARG_NAME,
        TOKEN_ID_RUNTIME_ARG_NAME,
    },
    Address, Error, ERC1155,
};
use casper_types::{
    account::AccountHash, CLType, CLTyped, ContractPackageHash, CLValue, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Parameter, U256,
};

// const MINT_ENTRY_POINT_NAME: &str = "mint";
// const BURN_ENTRY_POINT_NAME: &str = "burn";

/// "erc1155" is not mentioned here intentionally as the functionality is not compatible with ERC1155
/// token standard.
const TEST_CONTRACT_KEY_NAME: &str = "test_contract";
const TOKEN_URI: &str = "https://myuri-example.com/";
// const TOKEN_SYMBOL: &str = "CSPRT";
// const TOKEN_DECIMALS: u8 = 8;
// const TOKEN_TOTAL_SUPPLY: u64 = 1_000_000_000;

const TOKEN_OWNER_ADDRESS_1: Address = Address::Account(AccountHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
const TOKEN_OWNER_ADDRESS_2: Address = Address::Contract(ContractPackageHash::new([42; 32]));
const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

#[derive(Default)]
struct TestToken {
    erc1155: ERC1155,
}

impl TestToken {
    pub fn install() -> Result<TestToken, Error> {
        let uri = TOKEN_URI.to_string();
        let mut entry_points = EntryPoints::new();

        let mint_entrypoint = EntryPoint::new(
            MINT_ENTRY_POINT_NAME,
            vec![
                Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
                Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
                Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            ],
            CLType::Unit,
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );
        let burn_entrypoint = EntryPoint::new(
            BURN_ENTRY_POINT_NAME,
            vec![
                Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
                Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
                Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            ],
            CLType::Unit,
            // NOTE: For security reasons never use this entrypoint definition in a production
            // contract. This is marks the entry point as public.
            EntryPointAccess::Public,
            EntryPointType::Contract,
        );

        entry_points.add_entry_point(casper_erc1155::entry_points::total_supply());
        entry_points.add_entry_point(casper_erc1155::entry_points::balance_of());
        entry_points.add_entry_point(mint_entrypoint);
        entry_points.add_entry_point(burn_entrypoint);

        // Caution: This test uses `install_custom` without providing default entrypoints as
        // described by ERC1155 token standard.
        //
        // This is unsafe and this test contract is not a ERC1155 token standard-compliant token.
        // Contract developers should use example/erc1155 contract instead as a template for writing
        // their own tokens.
        let erc1155 = ERC1155::install_custom(uri, TEST_CONTRACT_KEY_NAME, entry_points)?;
        Ok(TestToken { erc1155 })
    }
}

impl Deref for TestToken {
    type Target = ERC1155;

    fn deref(&self) -> &Self::Target {
        &self.erc1155
    }
}

impl DerefMut for TestToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.erc1155
    }
}

#[no_mangle]
pub extern "C" fn uri() {
    let uri = ERC1155::default().uri();
    runtime::ret(CLValue::from_t(uri).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn total_supply() {
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let total_supply = TestToken::default().total_supply(&id);
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let balance = TestToken::default().balance_of(account, &id);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<Address> = runtime::get_named_arg(ACCOUNTS_RUNTIME_ARG_NAME);
    let ids: Vec<String> = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let balances = TestToken::default().balance_of_batch(accounts, ids);
    runtime::ret(CLValue::from_t(balances).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn is_approval_for_all() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved = TestToken::default().is_approval_for_all(account, operator);
    runtime::ret(CLValue::from_t(approved).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn mint() {
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestToken::default()
        .mint(to, &id, amount)
        .unwrap_or_revert();
}
#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    TestToken::default()
        .burn(owner, &id, amount)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let mut test_token = TestToken::install().unwrap_or_revert();
    let id = String::from("1");
    test_token
        .mint(TOKEN_OWNER_ADDRESS_1, &id, U256::from(TOKEN_OWNER_AMOUNT_1))
        .unwrap_or_revert();

    test_token
        .mint(TOKEN_OWNER_ADDRESS_2, &id, U256::from(TOKEN_OWNER_AMOUNT_2))
        .unwrap_or_revert();
}
