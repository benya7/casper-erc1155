//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

use crate::{
    address::Address,
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, AMOUNTS_RUNTIME_ARG_NAME,
        AMOUNT_RUNTIME_ARG_NAME, APPROVED_RUNTIME_ARG_NAME, BALANCE_OF_BATCH_ENTRY_POINT_NAME,
        BALANCE_OF_ENTRY_POINT_NAME, BURN_ENTRY_POINT_NAME, FROM_RUNTIME_ARG_NAME,
        IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME, MINT_ENTRY_POINT_NAME, OPERATOR_RUNTIME_ARG_NAME,
        OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME, SAFE_TRANSFER_FROM_ENTRY_POINT_NAME,
        SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME, TOKEN_IDS_RUNTIME_ARG_NAME,
        TOKEN_ID_RUNTIME_ARG_NAME, TOTAL_SUPPLY_ENTRY_POINT_NAME, URI_ENTRY_POINT_NAME,
    },
};

/// Returns the `uri` entry point.
pub fn uri() -> EntryPoint {
    EntryPoint::new(
        String::from(URI_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
        vec![Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `balance_of_batch` entry point.
pub fn balance_of_batch() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_BATCH_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ACCOUNTS_RUNTIME_ARG_NAME, Vec::<Address>::cl_type()),
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
        ],
        Vec::<U256>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `set_approval_for_all` entry point.
pub fn set_approval_for_all() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(APPROVED_RUNTIME_ARG_NAME, bool::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `is_approval_for_all` entry point.
pub fn is_approval_for_all() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `safe_transfer_from` entry point.
pub fn safe_transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(SAFE_TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(FROM_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `safe_batch_transfer_from` entry point.
pub fn safe_batch_transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(FROM_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
            Parameter::new(AMOUNTS_RUNTIME_ARG_NAME, Vec::<U256>::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `mint` entry point.
pub fn mint() -> EntryPoint {
    EntryPoint::new(
        String::from(MINT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the default set of ERC1155 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(uri());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(balance_of_batch());
    entry_points.add_entry_point(set_approval_for_all());
    entry_points.add_entry_point(is_approval_for_all());
    entry_points.add_entry_point(safe_transfer_from());
    entry_points.add_entry_point(safe_batch_transfer_from());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(burn());
    entry_points
}
