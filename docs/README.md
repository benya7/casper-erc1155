# ERC-1155 Tutorial
This tutorial introduces you to an implementation of the ERC-1155 standard for the Casper blockchain.

The Ethereum Request for Comment (ERC-1155) standard is an integral part of the Ethereum ecosystem. This standard is well established for building new tokens based on smart contracts. These ERC-1155 tokens are blockchain-based assets that have value and can be transferred or recorded.

The ERC-1155 standard defines a set of rules that dictate the total supply of tokens, how the tokens are transferred, how transactions are approved, and how token data is accessed. These rules are implemented using the following functions defined by ERC-1155, _**totalSupply**, _**balanceOf**_, _**balanceOfBatch**_, _**isApprovalForAll**_, _**setApprovalForAll**_, _**safeTransferFrom**_, _**safeBatchTransferFrom**_, _**mint**_ and _**burn**_ which are described in detail within this tutorial.

The code for this tutorial is available in [GitHub](setApprovalForAll). If you haven’t read [Writing Rust Contracts on Casper](https://docs.casperlabs.io/en/latest/dapp-dev-guide/writing-contracts/writing-rust-contracts.html), we recommend you start there.

You can read more about the original specification in [Ethereum (ERC-1155)](https://docs.casperlabs.io/en/latest/dapp-dev-guide/writing-contracts/writing-rust-contracts.html).

### **Tutorial Content**

- [ERC-1155 Tutorial](#erc-1155-tutorial)
    - [**Tutorial Content**](#tutorial-content)
  - [Preparation {#prepraration-id}](#preparation-prepraration-id)
  - [ERC-1155 Implementation](#erc-1155-implementation)
  - [Cloning the Example Contract](#cloning-the-example-contract)
  - [Installing the Required Crates](#installing-the-required-crates)
  - [Initializing the Contract](#initializing-the-contract)
  - [The **uri**, **total_supply**, **balance_of**, **balance_of_batch** and **is_approval_for_all** functions](#the-uri-total_supply-balance_of-balance_of_batch-and-is_approval_for_all-functions)
  - [The **safe_transfer_from**, **safe_batch_transfer_from**, **set_approval_for_all** functions](#the-safe_transfer_from-safe_batch_transfer_from-set_approval_for_all-functions)
  - [The **mint** and **burn** functions.](#the-mint-and-burn-functions)
  - [Testing the Contract {#testing-id}](#testing-the-contract-testing-id)
  - [Writing documentation..](#writing-documentation)



## Preparation {#prepraration-id}

First clone the contract from GitHub:
```cmd
$ git clone https://github.com/casper-ecosystem/erc20 && cd erc20
```
Prepare your environment with the following command:
```
$ make prepare
```
If your environment is setup correctly, you will see this output:

```
rustup target add wasm32-unknown-unknown
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
```
If you do not see this message, check [the Getting Started guide](https://docs.casperlabs.io/en/latest/dapp-dev-guide/setup-of-rust-contract-sdk.html).

Next, compile your contract and run the contract unit tests.

``` shell
$ make build-contracts
$ make test
```

## ERC-1155 Implementation

The ERC-1155 standard is defined in an Ethereum Improvement Proposal (EIP). Read it carefully, as it defines the methods we have implemented:

- total_supply
- balance_of
- balance_of_batch
- set_approval_for_all
- is_approval_for_all
- safe_transfer_from
- safe_batch_transfer_from
- mint
- burn

## Cloning the Example Contract

An example ERC-1155 for Casper is located in [GitHub](https://github.com/en0c-026/casper-erc1155).

## Installing the Required Crates

This is a Rust contract. In Rust, the keyword **use** is like an **include** statement in C/C++. Casper contracts require a few crates to be included. They are:

- contract: The Casper contract API for runtime and storage
- types: The Casper contract type system


```rust
use alloc::{string::String, vec::Vec};
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc1155::{
    constants::{
        ACCOUNTS_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, AMOUNTS_RUNTIME_ARG_NAME,
        AMOUNT_RUNTIME_ARG_NAME, APPROVED_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME,
        OPERATOR_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        TOKEN_IDS_RUNTIME_ARG_NAME, TOKEN_ID_RUNTIME_ARG_NAME, URI_RUNTIME_ARG_NAME,
    },
    Address, ERC1155,
};
use casper_types::{CLValue, U256};

```
## Initializing the Contract

When the contract is deployed, it must be initialized with some values; this is done with the help of the **call()** function. The contract is initialized with a name, symbol, decimals, starting balances, and the starting token supply.

```rust
#[no_mangle]
fn call() {
    let uri = runtime::get_named_arg(URI_RUNTIME_ARG_NAME);
    let _token = ERC1155::install(uri).unwrap_or_revert();
}**
```
## The **uri**, **total_supply**, **balance_of**, **balance_of_batch** and **is_approval_for_all** functions
Let’s explore the implementation of some key ERC-1155 methods: **uri**, **total_supply**, **balance_of**, **balance_of_batch and**, **is_approval_for_all**.

The **is_approval_for_all** method gets returns true if the operator is approved for the owner.

```rust
#[no_mangle]
pub extern "C" fn uri() {
    let uri = ERC1155::default().uri();
    runtime::ret(CLValue::from_t(uri).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let total_supply = ERC1155::default().total_supply(&id);
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let balance = ERC1155::default().balance_of(account, &id);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of_batch() {
    let accounts: Vec<Address> = runtime::get_named_arg(ACCOUNTS_RUNTIME_ARG_NAME);
    let ids: Vec<String> = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let balance = ERC1155::default().balance_of_batch(accounts, ids);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}
#[no_mangle]
pub extern "C" fn is_approval_for_all() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved = ERC1155::default().is_approval_for_all(account, operator);
    runtime::ret(CLValue::from_t(approved).unwrap_or_revert());
}
```
## The **safe_transfer_from**, **safe_batch_transfer_from**, **set_approval_for_all** functions

Here is the **safe_transfer_from** method, which makes it possible to transfer tokens from the sender address to the recipient address. If the sender address has enough balance, then tokens should be transferred to the recipient address.

This function can also be used by an approved operator to spend funds from an owner.

```rust
#[no_mangle]
pub extern "C" fn safe_transfer_from() {
    let from: Address = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default()
        .safe_transfer_from(from, to, &id, amount)
        .unwrap_or_revert();
}

```
The **safe_batch_transfer_from** method is the batch version of **safe_transfer_from**.
```rust
#[no_mangle]
pub extern "C" fn safe_batch_transfer_from() {
    let from: Address = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let ids: Vec<String> = runtime::get_named_arg(TOKEN_IDS_RUNTIME_ARG_NAME);
    let amounts: Vec<U256> = runtime::get_named_arg(AMOUNTS_RUNTIME_ARG_NAME);
    ERC1155::default()
        .safe_batch_transfer_from(from, to, ids, amounts)
        .unwrap_or_revert();
}
```
The **set_approval_for_all method** allows you to set whether the operator is approved for the owner.

```rust

#[no_mangle]
pub extern "C" fn set_approval_for_all() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let approved: bool = runtime::get_named_arg(APPROVED_RUNTIME_ARG_NAME);
    ERC1155::default()
        .set_approval_for_all(operator, approved)
        .unwrap_or_revert();
}
```
## The **mint** and **burn** functions.

These functions **mint** and **burn** are experimental, they should not be implemented in mainnet until a permission system is implemented.

It allows the possibility of mint or burn new tokens given an account and a token id.

```rust
#[no_mangle]
pub extern "C" fn mint() {
    let to: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default().mint(to, &id, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let id: String = runtime::get_named_arg(TOKEN_ID_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC1155::default()
        .burn(owner, &id, amount)
        .unwrap_or_revert();
}
```

## Testing the Contract {#testing-id}


STOP!
## Writing documentation..