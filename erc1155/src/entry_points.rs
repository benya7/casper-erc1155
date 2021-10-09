//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256,
};

use crate::{
    address::Address,
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ADDRESS_RUNTIME_ARG_NAME, AMOUNTS_RUNTIME_ARG_NAME,
        AMOUNT_RUNTIME_ARG_NAME, APPROVED_RUNTIME_ARG_NAME, BALANCE_OF_BATCH_ENTRY_POINT_NAME,
        BALANCE_OF_ENTRY_POINT_NAME, IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME, OWNER_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME,
        SAFE_TRANSFER_FROM_ENTRY_POINT_NAME, SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME,
        SPENDER_RUNTIME_ARG_NAME, TOKEN_IDS_RUNTIME_ARG_NAME, TOKEN_ID_RUNTIME_ARG_NAME,
    },
};

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(ADDRESS_RUNTIME_ARG_NAME, Address::cl_type()),
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
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
            Parameter::new(ACCOUNTS_RUNTIME_ARG_NAME, Vec::<Address>::cl_type()),
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
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(APPROVED_RUNTIME_ARG_NAME, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `allowance` entry point.
pub fn is_approval_for_all() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_APPROVAL_FOR_ALL_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `safe_transfer_from` entry point.
pub fn safe_transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(SAFE_TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_ID_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
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
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(TOKEN_IDS_RUNTIME_ARG_NAME, Vec::<String>::cl_type()),
            Parameter::new(AMOUNTS_RUNTIME_ARG_NAME, Vec::<U256>::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
// pub fn total_supply() -> EntryPoint {
//     EntryPoint::new(
//         String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
//         Vec::new(),
//         U256::cl_type(),
//         EntryPointAccess::Public,
//         EntryPointType::Contract,
//     )
// }

/// Returns the default set of ERC1155 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(balance_of_batch());
    entry_points.add_entry_point(set_approval_for_all());
    entry_points.add_entry_point(is_approval_for_all());
    entry_points.add_entry_point(safe_transfer_from());
    entry_points.add_entry_point(safe_batch_transfer_from());
    entry_points
}
