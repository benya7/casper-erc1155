#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use casper_erc1155::{
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME,
        AMOUNTS_RUNTIME_ARG_NAME, APPROVED_RUNTIME_ARG_NAME, OPERATOR_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME,
        SAFE_TRANSFER_FROM_ENTRY_POINT_NAME, SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME,
        TOKEN_IDS_RUNTIME_ARG_NAME, TOKEN_ID_RUNTIME_ARG_NAME,
    },
    Address,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLTyped, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};

const CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "check_total_supply";
const CHECK_BALANCE_OF_ENTRY_POINT_NAME: &str = "check_balance_of";
const CHECK_BALANCE_OF_BATCH_ENTRY_POINT_NAME: &str = "check_balance_of_batch";
const CHECK_IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME: &str = "check_is_approval_for_all";
const SET_APPROVAL_FOR_ALL_STORED_CONTRACT_ENTRY_POINT_NAME: &str =
    "set_approval_for_all_stored_contract";
const SAFE_TRANSFER_FROM_STORED_CONTRACT_ENTRY_POINT_NAME: &str =
    "safe_transfer_from_stored_contract";
const SAFE_BATCH_TRANFER_FROM_STORED_CONTRACT_ENTRY_POINT_NAME: &str =
    "safe_batch_transfer_from_stored_contract";
const TOKEN_CONTRACT_RUNTIME_ARG_NAME: &str = "token_contract";
const RESULT_KEY: &str = "result";
const ERC1155_TEST_CALL_KEY: &str = "erc1155_test_call";

fn store_result<T: CLTyped + ToBytes>(result: T) {
    match runtime::get_key(RESULT_KEY) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(RESULT_KEY, new_uref.into());
        }
    }
}

#[no_mangle]
extern "C" fn check_total_supply() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let total_supply_arg = runtime_args! {
      casper_erc1155::constants::TOKEN_ID_RUNTIME_ARG_NAME => id,
    };
    let total_supply: U256 = runtime::call_contract(
        token_contract,
        casper_erc1155::constants::TOTAL_SUPPLY_ENTRY_POINT_NAME,
        total_supply_arg,
    );
    store_result(total_supply);
}
#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);

    let balance_args = runtime_args! {
        casper_erc1155::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
        casper_erc1155::constants::TOKEN_ID_RUNTIME_ARG_NAME => id,
    };
    let result: U256 = runtime::call_contract(
        token_contract,
        casper_erc1155::constants::BALANCE_OF_ENTRY_POINT_NAME,
        balance_args,
    );

    store_result(result);
}
#[no_mangle]
extern "C" fn check_balance_of_batch() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let accounts: Address = runtime::get_named_arg(ACCOUNTS_RUNTIME_ARG_NAME);
    let ids: String = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);

    let balance_args = runtime_args! {
        casper_erc1155::constants::ACCOUNTS_RUNTIME_ARG_NAME => accounts,
        casper_erc1155::constants::TOKEN_IDS_RUNTIME_ARG_NAME => ids,
    };
    let result: Vec<U256> = runtime::call_contract(
        token_contract,
        casper_erc1155::constants::BALANCE_OF_BATCH_ENTRY_POINT_NAME,
        balance_args,
    );

    store_result(result);
}
#[no_mangle]
extern "C" fn set_approval_for_all_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved: bool = runtime::get_named_arg(APPROVED_RUNTIME_ARG_NAME);

    let set_approval_args = runtime_args! {
        OPERATOR_RUNTIME_ARG_NAME => operator,
        APPROVED_RUNTIME_ARG_NAME => approved,
    };

    runtime::call_contract::<()>(
        token_contract,
        SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME,
        set_approval_args,
    );
}
#[no_mangle]
extern "C" fn check_is_approval_for_all() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);

    let operators_args = runtime_args! {
        casper_erc1155::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
        casper_erc1155::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
    };
    let result: bool = runtime::call_contract(
        token_contract,
        casper_erc1155::constants::IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME,
        operators_args,
    );

    store_result(result);
}
#[no_mangle]
extern "C" fn safe_transfer_from_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    let transfer_args = runtime_args! {
        RECIPIENT_RUNTIME_ARG_NAME => recipient,
        TOKEN_ID_RUNTIME_ARG_NAME => id,
        AMOUNT_RUNTIME_ARG_NAME => amount,
    };

    runtime::call_contract::<()>(
        token_contract,
        SAFE_TRANSFER_FROM_ENTRY_POINT_NAME,
        transfer_args,
    );
}
#[no_mangle]
extern "C" fn safe_batch_transfer_from_stored_contract() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let ids: String = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let amounts: U256 = runtime::get_named_arg(AMOUNTS_RUNTIME_ARG_NAME);

    let transfer_args = runtime_args! {
        RECIPIENT_RUNTIME_ARG_NAME => recipient,
        TOKEN_IDS_RUNTIME_ARG_NAME => ids,
        AMOUNTS_RUNTIME_ARG_NAME => amounts,
    };

    runtime::call_contract::<()>(
        token_contract,
        SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME,
        transfer_args,
    );
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    let check_total_supply_entrypoint = EntryPoint::new(
        String::from(CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME),
        vec![
          Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
          Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
          ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_entrypoint = EntryPoint::new(
        String::from(CHECK_BALANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_batch_entrypoint = EntryPoint::new(
        String::from(CHECK_BALANCE_OF_BATCH_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(ACCOUNTS_RUNTIME_ARG_NAME, Vec::<Address>::cl_type()),
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let set_approval_for_all_stored_contract_entrypoint = EntryPoint::new(
        String::from(SET_APPROVAL_FOR_ALL_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(APPROVED_RUNTIME_ARG_NAME, bool::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_is_approval_for_all_entrypoint = EntryPoint::new(
        String::from(CHECK_IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(APPROVED_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let safe_transfer_from_stored_contract_entrypoint = EntryPoint::new(
        String::from(SAFE_TRANSFER_FROM_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let safe_batch_transfer_from_stored_contract_entrypoint = EntryPoint::new(
        String::from(SAFE_BATCH_TRANFER_FROM_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
            Parameter::new(AMOUNTS_RUNTIME_ARG_NAME, Vec::<U256>::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(check_total_supply_entrypoint);
    entry_points.add_entry_point(check_balance_of_entrypoint);
    entry_points.add_entry_point(check_balance_of_batch_entrypoint);
    entry_points.add_entry_point(set_approval_for_all_stored_contract_entrypoint);
    entry_points.add_entry_point(check_is_approval_for_all_entrypoint);
    entry_points.add_entry_point(safe_transfer_from_stored_contract_entrypoint);
    entry_points.add_entry_point(safe_batch_transfer_from_stored_contract_entrypoint);

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(ERC1155_TEST_CALL_KEY.to_string()),
        None,
    );
}
