// use once_cell::sync::Lazy;

// use casper_engine_test_support::{
//     internal::{ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST},
//     DEFAULT_ACCOUNT_ADDR, MINIMUM_ACCOUNT_CREATION_BALANCE,
// };
// use casper_execution_engine::core::{
//     engine_state::{Error as CoreError, ExecuteRequest},
//     execution::Error as ExecError,
// };
// use casper_types::{
//     account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint, ApiError, CLTyped,
//     ContractHash, ContractPackageHash, Key, PublicKey, RuntimeArgs, SecretKey, U256,
// };

// const EXAMPLE_ERC1155_TOKEN: &str = "erc1155_token.wasm";
// const CONTRACT_ERC1155_TEST: &str = "erc1155_test.wasm";
// const CONTRACT_ERC1155_TEST_CALL: &str = "erc1155_test_call.wasm";
// const ERC1155_TOKEN_CONTRACT_KEY: &str = "erc1155_token_contract";
// const BALANCES_KEY: &str = "balances";
// const OPERATORS_KEY: &str = "operators";
// const TEST_CONTRACT_KEY: &str = "test_contract";

// const _ERROR_INVALID_CONTEXT: u16 = u16::MAX;
// const ERROR_INSUFFICIENT_BALANCE: u16 = u16::MAX - 1;
// const ERROR_INSUFFICIENT_ALLOWANCE: u16 = u16::MAX - 2;
// const ERROR_OVERFLOW: u16 = u16::MAX - 3;

// const METHOD_SAFE_TRANSFER_FROM: &str = "safe_transfer_from";
// const ARG_RECIPIENT: &str = "to";
// const ARG_ACCOUNTS: &str = "accounts";
// const ARG_TOKEN_ID: &str = "id";
// const ARG_TOKEN_IDS: &str = "ids";
// const ARG_AMOUNT: &str = "amount";
// const ARG_APPROVED: &str = "approved";

// const METHOD_SET_APPROVAL_FOR_ALL: &str = "set_approval_for_all";
// const ARG_ACCOUNT: &str = "account";
// const ARG_OPERATOR: &str = "operator";
// const ARG_AMOUNTS: &str = "amounts";
// const CHECK_BALANCE_OF_ENTRYPOINT: &str = "check_balance_of";
// const CHECK_BALANCE_OF_BATCH_ENTRYPOINT: &str = "check_balance_of_batch";
// const CHECK_IS_APPROVAL_FOR_ALL_ENTRYPOINT: &str = "check_is_approval_for_all";
// const ARG_TOKEN_CONTRACT: &str = "token_contract";
// const ARG_ADDRESS: &str = "address";
// const RESULT_KEY: &str = "result";
// const ERC1155_TEST_CALL_KEY: &str = "erc1155_test_call";

// static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
//     Lazy::new(|| SecretKey::secp256k1_from_bytes(&[221u8; 32]).unwrap());
// static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
//     Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
// static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

// static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
//     Lazy::new(|| SecretKey::secp256k1_from_bytes(&[212u8; 32]).unwrap());
// static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
//     Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
// static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

// const TRANSFER_AMOUNT_1: u64 = 11550_001;
// const TRANSFER_AMOUNT_2: u64 = 19_999;
// const ALLOWANCE_AMOUNT_1: u64 = 456_789;
// const ALLOWANCE_AMOUNT_2: u64 = 87_654;

// const METHOD_SAFE_TRANSFER_FROM_STORED_CONTRACT: &str = "safe_transfer_from_stored_contract";
// const METHOD_SET_APPROVAL_FOR_ALL_STORED_CONTRACT: &str = "set_approval_for_all_stored_contract";

// const TOKEN_OWNER_ADDRESS_1: Key = Key::Account(AccountHash::new([42; 32]));
// const TOKEN_OWNER_AMOUNT_1: u64 = 1_000_000;
// const TOKEN_OWNER_ADDRESS_2: Key = Key::Hash([42; 32]);
// const TOKEN_OWNER_AMOUNT_2: u64 = 2_000_000;

// // const METHOD_MINT: &str = "mint";
// // const METHOD_BURN: &str = "burn";

// /// Converts hash addr of Account into Hash, and Hash into Account
// ///
// /// This is useful for making sure ERC1155 library respects different variants of Key when storing
// /// balances.
// fn invert_erc1155_address(address: Key) -> Key {
//     match address {
//         Key::Account(account_hash) => Key::Hash(account_hash.value()),
//         Key::Hash(contract_hash) => Key::Account(AccountHash::new(contract_hash)),
//         _ => panic!("Unsupported Key variant"),
//     }
// }

// #[derive(Copy, Clone)]
// struct TestContext {
//     erc1155_token: ContractHash,
//     test_contract: ContractHash,
//     erc1155_test_call: ContractPackageHash,
// }

// fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
//     let mut builder = InMemoryWasmTestBuilder::default();
//     builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

//     let id: Option<u64> = None;
//     let transfer_1_args = runtime_args! {
//         mint::ARG_TARGET => *ACCOUNT_1_ADDR,
//         mint::ARG_ID => id,
//         mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
//     };
//     let transfer_2_args = runtime_args! {
//         mint::ARG_TARGET => *ACCOUNT_2_ADDR,
//         mint::ARG_ID => id,
//         mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
//     };

//     let transfer_request_1 =
//         ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
//     let transfer_request_2 =
//         ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_2_args).build();

//     let install_request_1 = ExecuteRequestBuilder::standard(
//         *DEFAULT_ACCOUNT_ADDR,
//         EXAMPLE_ERC1155_TOKEN,
//         RuntimeArgs::default(),
//     )
//     .build();
//     let install_request_2 = ExecuteRequestBuilder::standard(
//         *DEFAULT_ACCOUNT_ADDR,
//         CONTRACT_ERC1155_TEST,
//         RuntimeArgs::default(),
//     )
//     .build();
//     let install_request_3 = ExecuteRequestBuilder::standard(
//         *DEFAULT_ACCOUNT_ADDR,
//         CONTRACT_ERC1155_TEST_CALL,
//         RuntimeArgs::default(),
//     )
//     .build();

//     builder.exec(transfer_request_1).expect_success().commit();
//     builder.exec(transfer_request_2).expect_success().commit();
//     builder.exec(install_request_1).expect_success().commit();
//     builder.exec(install_request_2).expect_success().commit();
//     builder.exec(install_request_3).expect_success().commit();

//     let account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");

//     let erc1155_token = account
//         .named_keys()
//         .get(ERC1155_TOKEN_CONTRACT_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractHash::new)
//         .expect("should have contract hash");

//     let test_contract = account
//         .named_keys()
//         .get(TEST_CONTRACT_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractHash::new)
//         .expect("should have contract hash");

//     let erc1155_test_call = account
//         .named_keys()
//         .get(ERC1155_TEST_CALL_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractPackageHash::new)
//         .expect("should have contract hash");

//     let test_context = TestContext {
//         erc1155_token,
//         test_contract,
//         erc1155_test_call,
//     };

//     (builder, test_context)
// }

// fn erc1155_check_total_supply(
//     builder: &mut InMemoryWasmTestBuilder,
//     erc1155_contract_hash: &ContractHash,
// ) -> U256 {
//     let account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");

//     let erc1155_test_contract_hash = account
//         .named_keys()
//         .get(ERC1155_TEST_CALL_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractPackageHash::new)
//         .expect("should have test contract hash");

//     let check_total_supply_args = runtime_args! {
//         ARG_TOKEN_CONTRACT => *erc1155_contract_hash,
//     };

//     let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         erc1155_test_contract_hash,
//         None,
//         CHECK_TOTAL_SUPPLY_ENTRYPOINT,
//         check_total_supply_args,
//     )
//     .build();
//     builder.exec(exec_request).expect_success().commit();

//     get_test_result(builder, erc1155_test_contract_hash)
// }

// fn get_test_result<T: FromBytes + CLTyped>(
//     builder: &mut InMemoryWasmTestBuilder,
//     erc1155_test_contract_hash: ContractPackageHash,
// ) -> T {
//     let contract_package = builder
//         .get_contract_package(erc1155_test_contract_hash)
//         .expect("should have contract package");
//     let enabled_versions = contract_package.enabled_versions();
//     let (_version, contract_hash) = enabled_versions
//         .iter()
//         .rev()
//         .next()
//         .expect("should have latest version");

//     builder.get_value(*contract_hash, RESULT_KEY)
// }

// fn erc1155_check_balance_of(
//     builder: &mut InMemoryWasmTestBuilder,
//     erc1155_contract_hash: &ContractHash,
//     account: Key,
//     id: String
// ) -> U256 {
//     let _account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");

//     let erc1155_test_contract_hash = _account
//         .named_keys()
//         .get(ERC1155_TEST_CALL_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractPackageHash::new)
//         .expect("should have test contract hash");
//     let id = String::from(id);
//     let check_balance_args = runtime_args! {
//         ARG_TOKEN_CONTRACT => *erc1155_contract_hash,
//         ARG_ADDRESS => account,
//         ARG_TOKEN_ID => id,
//     };
//     let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         erc1155_test_contract_hash,
//         None,
//         CHECK_BALANCE_OF_ENTRYPOINT,
//         check_balance_args,
//     )
//     .build();
//     builder.exec(exec_request).expect_success().commit();

//     get_test_result(builder, erc1155_test_contract_hash)
// }

// fn erc1155_check_balance_of_batch(
//     builder: &mut InMemoryWasmTestBuilder,
//     erc1155_contract_hash: &ContractHash,
//     accounts: Vec<Key>,
//     ids: Vec<String>
// ) -> Vec<U256> {
//     let _account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");

//     let erc1155_test_contract_hash = _account
//         .named_keys()
//         .get(ERC1155_TEST_CALL_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractPackageHash::new)
//         .expect("should have test contract hash");

//     let check_balance_batch_args = runtime_args! {
//         ARG_TOKEN_CONTRACT => *erc1155_contract_hash,
//         ARG_ACCOUNTS => accounts,
//         ARG_TOKEN_IDS => ids,
//     };
//     let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         erc1155_test_contract_hash,
//         None,
//         CHECK_BALANCE_OF_BATCH_ENTRYPOINT,
//         check_balance_batch_args,
//     )
//     .build();
//     builder.exec(exec_request).expect_success().commit();

//     get_test_result(builder, erc1155_test_contract_hash)
// }

// fn erc1155_check_is_approval_for_all(
//     builder: &mut InMemoryWasmTestBuilder,
//     address: Key,
//     operator: Key,
// ) -> bool {
//     let account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");
//     let erc1155_contract_hash = account
//         .named_keys()    
//         .get(ERC1155_TOKEN_CONTRACT_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractHash::new)
//         .expect("should have test contract hash");
//     let erc1155_test_contract_hash = address
//         .get(ERC1155_TOKEN_CONTRACT_KEY)
//         .and_then(|key| key.into_hash())
//         .map(ContractHash::new)
//         .expect("should have test contract hash");

//     let check_is_approval_for_all_args = runtime_args! {
//         ARG_TOKEN_CONTRACT => erc1155_contract_hash,
//         ARG_ACCOUNT => account,
//         ARG_OPERATOR => operator,
//     };
//     let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         erc1155_test_contract_hash,
//         None,
//         CHECK_IS_APPROVAL_FOR_ALL_ENTRYPOINT,
//         check_is_approval_for_all_args,
//     )
//     .build();
//     builder.exec(exec_request).expect_success().commit();

//     get_test_result(builder, erc1155_test_contract_hash)
// }

// fn test_erc1155_safe_transfer_from(
//     builder: &mut InMemoryWasmTestBuilder,
//     test_context: &TestContext,
//     sender1: Key,
//     recipient1: Key,
//     sender2: Key,
//     recipient2: Key,
//     id: String
// ) {
//     let TestContext { erc1155_token, .. } = test_context;

//     let transfer_amount_1 = U256::from(TRANSFER_AMOUNT_1);
//     let transfer_amount_2 = U256::from(TRANSFER_AMOUNT_2);

//     let sender_balance_before = erc1155_check_balance_of(builder, erc1155_token, sender1, id);
//     assert_ne!(sender_balance_before, U256::zero());

//     let account_1_balance_before = erc1155_check_balance_of(builder, erc1155_token, recipient1, id);
//     assert_eq!(account_1_balance_before, U256::zero());

//     let account_2_balance_before = erc1155_check_balance_of(builder, erc1155_token, recipient2, id);
//     assert_eq!(account_2_balance_before, U256::zero());

//     let token_transfer_request_1 =
//         make_erc1155_safe_transfer_from_request(sender1, erc1155_token, recipient1, id, transfer_amount_1);

//     builder
//         .exec(token_transfer_request_1)
//         .expect_success()
//         .commit();

//     let account_1_balance_after = erc1155_check_balance_of(builder, erc1155_token, recipient1, id);
//     assert_eq!(account_1_balance_after, transfer_amount_1);
//     let account_1_balance_before = account_1_balance_after;

//     let sender_balance_after = erc1155_check_balance_of(builder, erc1155_token, sender1, id);
//     assert_eq!(
//         sender_balance_after,
//         sender_balance_before - transfer_amount_1
//     );
//     let sender_balance_before = sender_balance_after;

//     let token_transfer_request_2 =
//         make_erc1155_safe_transfer_from_request(sender2, erc1155_token, recipient2, id, transfer_amount_2);

//     builder
//         .exec(token_transfer_request_2)
//         .expect_success()
//         .commit();

//     let sender_balance_after = erc1155_check_balance_of(builder, erc1155_token, sender2, id);
//     assert_eq!(sender_balance_after, sender_balance_before);

//     let account_1_balance_after = erc1155_check_balance_of(builder, erc1155_token, recipient1, id);
//     assert!(account_1_balance_after < account_1_balance_before);
//     assert_eq!(
//         account_1_balance_after,
//         transfer_amount_1 - transfer_amount_2
//     );

//     let account_2_balance_after = erc1155_check_balance_of(builder, erc1155_token, recipient2, id);
//     assert_eq!(account_2_balance_after, transfer_amount_2);
// }

// fn make_erc1155_safe_transfer_from_request(
//     sender: Key,
//     erc1155_token: &ContractHash,
//     recipient: Key,
//     id: String,
//     amount: U256,
// ) -> ExecuteRequest {
//     match sender {
//         Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
//             sender,
//             *erc1155_token,
//             METHOD_SAFE_TRANSFER_FROM,
//             runtime_args! {
//               ARG_RECIPIENT => recipient,
//               ARG_TOKEN_ID => id,
//               ARG_AMOUNT => amount,
//             },
//         )
//         .build(),
//         Key::Hash(contract_package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
//             *DEFAULT_ACCOUNT_ADDR,
//             ContractPackageHash::new(contract_package_hash),
//             None,
//             METHOD_SAFE_TRANSFER_FROM_STORED_CONTRACT,
//             runtime_args! {
//                 ARG_TOKEN_CONTRACT => *erc1155_token,
//                 ARG_RECIPIENT => recipient,
//                 ARG_TOKEN_ID => id,
//                 ARG_AMOUNT => amount,
//             },
//         )
//         .build(),
//         _ => panic!("Unknown variant"),
//     }
// }

// fn make_erc1155_set_approval_for_all_request(
//     sender: Key,
//     erc1155_token: &ContractHash,
//     operator: Key,
//     approved: bool,
// ) -> ExecuteRequest {
//     match sender {
//         Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
//             sender,
//             *erc1155_token,
//             METHOD_SET_APPROVAL_FOR_ALL,
//             runtime_args! {
//                 ARG_OPERATOR => operator,
//                 ARG_APPROVED => approved,
//             },
//         )
//         .build(),
//         Key::Hash(contract_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
//             *DEFAULT_ACCOUNT_ADDR,
//             ContractPackageHash::new(contract_hash),
//             None,
//             METHOD_SET_APPROVAL_FOR_ALL_STORED_CONTRACT,
//             runtime_args! {
//                 ARG_TOKEN_CONTRACT => *erc1155_token,
//                 ARG_OPERATOR => operator,
//                 ARG_APPROVED => approved,
//             },
//         )
//         .build(),
//         _ => panic!("Unknown variant"),
//     }
// }

// fn test_set_approval_for_all(
//     builder: &mut InMemoryWasmTestBuilder,
//     test_context: &TestContext,
//     sender: Key,
//     account: Key,
//     operator: Key,
// ) {
//     let TestContext { erc1155_token, .. } = test_context;

//     let account_is_approval_before = erc1155_check_is_approval_for_all(builder, account, operator);
//     assert_eq!(account_is_approval_before, false);

//     let approve_request = make_erc1155_set_approval_for_all_request(sender, erc1155_token, operator, true);

//     builder.exec(approve_request).expect_success().commit();

//     let account_is_approval_after = erc1155_check_is_approval_for_all(builder, account, operator);
//     assert_eq!(account_is_approval_before, true);

//     // Swap Key::Account into Hash and other way
//     let inverted_operator_key = invert_erc1155_address(operator);

//     let inverted_operator_is_approval = erc1155_check_is_approval_for_all(builder, account, inverted_operator_key);
//     assert_eq!(inverted_operator_is_approval, false);

// }

// {
//   #[test]
// fn should_have_queryable_properties() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let name: String = builder.get_value(erc1155_token, NAME_KEY);
//     assert_eq!(name, TOKEN_NAME);

//     let symbol: String = builder.get_value(erc1155_token, SYMBOL_KEY);
//     assert_eq!(symbol, TOKEN_SYMBOL);

//     let decimals: u8 = builder.get_value(erc1155_token, DECIMALS_KEY);
//     assert_eq!(decimals, TOKEN_DECIMALS);

//     let total_supply: U256 = builder.get_value(erc1155_token, TOTAL_SUPPLY_KEY);
//     assert_eq!(total_supply, U256::from(TOKEN_TOTAL_SUPPLY));

//     let owner_key = Key::Account(*DEFAULT_ACCOUNT_ADDR);

//     let owner_balance = erc1155_check_balance_of(&mut builder, &erc1155_token, owner_key);
//     assert_eq!(owner_balance, total_supply);

//     let contract_balance =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Hash(erc1155_token.value()));
//     assert_eq!(contract_balance, U256::zero());

//     // Ensures that Account and Contract ownership is respected and we're not keying ownership under
//     // the raw bytes regardless of variant.
//     let inverted_owner_key = invert_erc1155_address(owner_key);
//     let inverted_owner_balance =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, inverted_owner_key);
//     assert_eq!(inverted_owner_balance, U256::zero());
// }

// #[test]
// fn should_not_store_balances_or_operators_under_account_after_install() {
//     let (builder, _contract_hash) = setup();

//     let account = builder
//         .get_account(*DEFAULT_ACCOUNT_ADDR)
//         .expect("should have account");

//     let named_keys = account.named_keys();
//     assert!(!named_keys.contains_key(BALANCES_KEY), "{:?}", named_keys);
//     assert!(!named_keys.contains_key(OPERATORS_KEY), "{:?}", named_keys);
// }

// #[test]
// fn should_transfer_account_to_account() {
//     let (mut builder, test_context) = setup();
//     let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient1 = Key::Account(*ACCOUNT_1_ADDR);
//     let sender2 = Key::Account(*ACCOUNT_1_ADDR);
//     let recipient2 = Key::Account(*ACCOUNT_2_ADDR);
//     let id = String::from("1");

//     test_erc1155_safe_transfer_from(
//         &mut builder,
//         &test_context,
//         sender1,
//         recipient1,
//         sender2,
//         recipient2,
//         id,
//     );
// }

// #[test]
// fn should_transfer_account_to_contract() {
//     let (mut builder, test_context) = setup();

//     let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient1 = Key::Account(*ACCOUNT_1_ADDR);
//     let sender2 = Key::Account(*ACCOUNT_1_ADDR);
//     let recipient2 = Key::Hash(test_context.test_contract.value());
//     let id = String::from("1");
//     test_erc1155_safe_transfer_from(
//         &mut builder,
//         &test_context,
//         sender1,
//         recipient1,
//         sender2,
//         recipient2,
//         id,
//     );
// }

// #[test]
// fn should_transfer_contract_to_contract() {
//     let (mut builder, test_context) = setup();
//     let TestContext {
//         erc1155_test_call, ..
//     } = test_context;

//     let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient1 = Key::Hash(erc1155_test_call.value());
//     let sender2 = Key::Hash(erc1155_test_call.value());
//     let recipient2 = Key::Hash([42; 32]);
//     let id = String::from("1");
//     test_erc1155_safe_transfer_from(
//         &mut builder,
//         &test_context,
//         sender1,
//         recipient1,
//         sender2,
//         recipient2,
//         id,
//     );
// }

// #[test]
// fn should_transfer_contract_to_account() {
//     let (mut builder, test_context) = setup();
//     let TestContext {
//         erc1155_test_call, ..
//     } = test_context;

//     let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient1 = Key::Hash(erc1155_test_call.value());

//     let sender2 = Key::Hash(erc1155_test_call.value());
//     let recipient2 = Key::Account(*ACCOUNT_1_ADDR);
//     let id = String::from("1");
//     test_erc1155_safe_transfer_from(
//         &mut builder,
//         &test_context,
//         sender1,
//         recipient1,
//         sender2,
//         recipient2,
//         id,
//     );
// }

// #[test]
// fn should_transfer_full_owned_amount() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();
//     let id = String::from("1");
//     let transfer_1_sender = *DEFAULT_ACCOUNT_ADDR;
//     let sender_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(transfer_1_sender), id);
//     let erc1155_transfer_1_args = runtime_args! {
//       ARG_RECIPIENT => Key::Account(*ACCOUNT_1_ADDR),
//         ARG_TOKEN_ID => id,
//         ARG_AMOUNT => sender_balance_before,
//     };
    
//     let account_1_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(*ACCOUNT_1_ADDR), id);
//     assert_eq!(account_1_balance_before, U256::zero());

//     let token_transfer_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         transfer_1_sender,
//         erc1155_token,
//         METHOD_SAFE_TRANSFER_FROM,
//         erc1155_transfer_1_args,
//     )
//     .build();

//     builder
//         .exec(token_transfer_request_1)
//         .expect_success()
//         .commit();

//     let account_1_balance_after =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(*ACCOUNT_1_ADDR), id);
//     assert_eq!(account_1_balance_after, sender_balance_before);

//     let sender_balance_after = erc1155_check_balance_of(
//         &mut builder,
//         &erc1155_token,
//         Key::Account(transfer_1_sender),
//         id,
//     );
//     assert_eq!(sender_balance_after, U256::zero());
// }

// #[test]
// fn should_not_transfer_more_than_owned_balance() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();
//     let id = String::from("1");
//     let sender_balance_before = erc1155_check_balance_of(
//         &mut builder,
//         &erc1155_token,
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//         id,
//     );
//     let transfer_amount = sender_balance_before + U256::one();

//     let transfer_1_sender = *DEFAULT_ACCOUNT_ADDR;
//     let transfer_1_recipient = *ACCOUNT_1_ADDR;

//     let erc1155_transfer_1_args = runtime_args! {
//         ARG_RECIPIENT => Key::Account(transfer_1_recipient),
//         ARG_TOKEN_ID => id,
//         ARG_AMOUNT => transfer_amount,
//     };

//     assert!(transfer_amount > sender_balance_before);

//     let account_1_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(*ACCOUNT_1_ADDR), id);
//     assert_eq!(account_1_balance_before, U256::zero());

//     let token_transfer_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         transfer_1_sender,
//         erc1155_token,
//         METHOD_SAFE_TRANSFER_FROM,
//         erc1155_transfer_1_args,
//     )
//     .build();

//     builder.exec(token_transfer_request_1).commit();

//     let error = builder.get_error().expect("should have error");
//     assert!(
//         matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_BALANCE),
//         "{:?}",
//         error
//     );

//     let account_1_balance_after = erc1155_check_balance_of(
//         &mut builder,
//         &erc1155_token,
//         Key::Account(transfer_1_recipient),
//         id,
//     );
//     assert_eq!(account_1_balance_after, account_1_balance_before);

//     let sender_balance_after =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(transfer_1_sender), id);
//     assert_eq!(sender_balance_after, sender_balance_before);

//}

// #[test]
// fn should_set_approval_for_all_account_to_account() {
//     let (mut builder, test_context) = setup();

//     test_set_approval_for_all(
//         &mut builder,
//         &test_context,
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//         Key::Account(*ACCOUNT_1_ADDR),
//     );
// }

// #[test]
// fn should_set_approval_for_all_account_to_contract() {
//     let (mut builder, test_context) = setup();
//     test_set_approval_for_all(
//         &mut builder,
//         &test_context,
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//         Key::Hash([42; 32]),
//     );
// }

// #[test]
// fn should_set_approval_for_all_contract_to_account() {
//     let (mut builder, test_context) = setup();
//     let TestContext {
//         erc1155_test_call, ..
//     } = test_context;

//     test_set_approval_for_all(
//         &mut builder,
//         &test_context,
//         Key::Hash(erc1155_test_call.value()),
//         Key::Hash(erc1155_test_call.value()),
//         Key::Account(*DEFAULT_ACCOUNT_ADDR),
//     );
// }

// #[test]
// fn should_set_approval_for_all_contract_to_contract() {
//     let (mut builder, test_context) = setup();
//     let TestContext {
//         erc1155_test_call, ..
//     } = test_context;

//     test_set_approval_for_all(
//         &mut builder,
//         &test_context,
//         Key::Hash(erc1155_test_call.value()),
//         Key::Hash(erc1155_test_call.value()),
//         Key::Hash([42; 32]),
//     );
//}

// #[test]
// fn should_have_correct_balance_after_own_transfer() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let id = String::from("1");
//     let transfer_amount = U256::from(TRANSFER_AMOUNT_1);

//     let sender_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, sender, id);
//     let recipient_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient, id);

//     assert_eq!(sender_balance_before, recipient_balance_before);

//     let token_transfer_request_1 =
//         make_erc1155_safe_transfer_from_request(sender, &erc1155_token, recipient, id, transfer_amount);

//     builder
//         .exec(token_transfer_request_1)
//         .expect_success()
//         .commit();

//     let sender_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, sender, id);
//     assert_eq!(sender_balance_before, sender_balance_after);

//     let recipient_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient, id);
//     assert_eq!(recipient_balance_before, recipient_balance_after);

//     assert_eq!(sender_balance_after, recipient_balance_after);
// }

// #[test]
// fn should_verify_zero_amount_transfer_is_noop() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient = Key::Account(*ACCOUNT_1_ADDR);
//     let id = String::from("1");
//     let transfer_amount = U256::zero();

//     let sender_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, sender, id);
//     let recipient_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient, id);

//     let token_transfer_request_1 =
//         make_erc1155_safe_transfer_from_request(sender, &erc1155_token, recipient, id, transfer_amount);

//     builder
//         .exec(token_transfer_request_1)
//         .expect_success()
//         .commit();

//     let sender_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, sender, id);
//     assert_eq!(sender_balance_before, sender_balance_after);

//     let recipient_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient, id);
//     assert_eq!(recipient_balance_before, recipient_balance_after);
// }


// #[test]
// fn should_not_transfer_from_without_enough_allowance() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
//     let transfer_from_amount_1 = allowance_amount_1 + U256::one();

//     let sender = *DEFAULT_ACCOUNT_ADDR;
//     let owner = sender;
//     let recipient = *ACCOUNT_1_ADDR;

//     let erc1155_approve_args = runtime_args! {
//         ARG_OWNER => Key::Account(owner),
//         ARG_SPENDER => Key::Account(recipient),
//         ARG_AMOUNT => allowance_amount_1,
//     };
//     let erc1155_transfer_from_args = runtime_args! {
//         ARG_OWNER => Key::Account(owner),
//         ARG_RECIPIENT => Key::Account(recipient),
//         ARG_AMOUNT => transfer_from_amount_1,
//     };

//     let spender_allowance_before =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
//     assert_eq!(spender_allowance_before, U256::zero());

//     let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         sender,
//         erc1155_token,
//         METHOD_APPROVE,
//         erc1155_approve_args,
//     )
//     .build();

//     let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         sender,
//         erc1155_token,
//         METHOD_TRANSFER_FROM,
//         erc1155_transfer_from_args,
//     )
//     .build();

//     builder.exec(approve_request_1).expect_success().commit();

//     let account_1_allowance_after =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
//     assert_eq!(account_1_allowance_after, allowance_amount_1);

//     builder.exec(transfer_from_request_1).commit();

//     let error = builder.get_error().expect("should have error");
//     assert!(
//         matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_ALLOWANCE),
//         "{:?}",
//         error
//     );
// }

// #[test]
// fn should_transfer_from_from_account_to_account() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
//     let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
//     let transfer_from_amount_1 = allowance_amount_1;

//     let owner = *DEFAULT_ACCOUNT_ADDR;
//     let spender = *ACCOUNT_1_ADDR;

//     let erc1155_approve_args = runtime_args! {
//         ARG_OWNER => Key::Account(owner),
//         ARG_SPENDER => Key::Account(spender),
//         ARG_AMOUNT => allowance_amount_1,
//     };
//     let erc1155_transfer_from_args = runtime_args! {
//         ARG_OWNER => Key::Account(owner),
//         ARG_RECIPIENT => Key::Account(spender),
//         ARG_AMOUNT => transfer_from_amount_1,
//     };

//     let spender_allowance_before =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
//     assert_eq!(spender_allowance_before, U256::zero());

//     let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         owner,
//         erc1155_token,
//         METHOD_APPROVE,
//         erc1155_approve_args,
//     )
//     .build();

//     let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         spender,
//         erc1155_token,
//         METHOD_TRANSFER_FROM,
//         erc1155_transfer_from_args,
//     )
//     .build();

//     builder.exec(approve_request_1).expect_success().commit();

//     let account_1_balance_before =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(owner));
//     assert_eq!(account_1_balance_before, initial_supply);

//     let account_1_allowance_before =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
//     assert_eq!(account_1_allowance_before, allowance_amount_1);

//     builder
//         .exec(transfer_from_request_1)
//         .expect_success()
//         .commit();

//     let account_1_allowance_after =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
//     assert_eq!(
//         account_1_allowance_after,
//         account_1_allowance_before - transfer_from_amount_1
//     );

//     let account_1_balance_after =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(owner));
//     assert_eq!(
//         account_1_balance_after,
//         account_1_balance_before - transfer_from_amount_1
//     );
// }

// #[test]
// fn should_transfer_from_account_by_contract() {
//     let (
//         mut builder,
//         TestContext {
//             erc1155_token,
//             erc1155_test_call,
//             ..
//         },
//     ) = setup();

//     let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
//     let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
//     let transfer_from_amount_1 = allowance_amount_1;

//     let owner = *DEFAULT_ACCOUNT_ADDR;

//     let spender = Key::Hash(erc1155_test_call.value());
//     let recipient = Key::Account(*ACCOUNT_1_ADDR);

//     let erc1155_approve_args = runtime_args! {
//         ARG_OWNER => Key::Account(owner),
//         ARG_SPENDER => spender,
//         ARG_AMOUNT => allowance_amount_1,
//     };
//     let erc1155_transfer_from_args = runtime_args! {
//         ARG_TOKEN_CONTRACT => erc1155_token,
//         ARG_OWNER => Key::Account(owner),
//         ARG_RECIPIENT => recipient,
//         ARG_AMOUNT => transfer_from_amount_1,
//     };

//     let spender_allowance_before =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), spender);
//     assert_eq!(spender_allowance_before, U256::zero());

//     let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
//         owner,
//         erc1155_token,
//         METHOD_APPROVE,
//         erc1155_approve_args,
//     )
//     .build();

//     let transfer_from_request_1 = ExecuteRequestBuilder::versioned_contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         erc1155_test_call,
//         None,
//         METHOD_FROM_AS_STORED_CONTRACT,
//         erc1155_transfer_from_args,
//     )
//     .build();

//     builder.exec(approve_request_1).expect_success().commit();

//     let owner_balance_before =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(owner));
//     assert_eq!(owner_balance_before, initial_supply);

//     let spender_allowance_before =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), spender);
//     assert_eq!(spender_allowance_before, allowance_amount_1);

//     builder
//         .exec(transfer_from_request_1)
//         .expect_success()
//         .commit();

//     let spender_allowance_after =
//         erc1155_check_allowance_of(&mut builder, Key::Account(owner), spender);
//     assert_eq!(
//         spender_allowance_after,
//         spender_allowance_before - transfer_from_amount_1
//     );

//     let owner_balance_after =
//         erc1155_check_balance_of(&mut builder, &erc1155_token, Key::Account(owner));
//     assert_eq!(
//         owner_balance_after,
//         owner_balance_before - transfer_from_amount_1
//     );
// }
// #[test]
// fn test_mint_and_burn_tokens() {
//     let mint_amount = U256::one();

//     let (mut builder, TestContext { test_contract, .. }) = setup();
//     assert_eq!(
//         erc1155_check_balance_of(
//             &mut builder,
//             &test_contract,
//             Key::Account(*DEFAULT_ACCOUNT_ADDR)
//         ),
//         U256::from(TOKEN_TOTAL_SUPPLY),
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
//         U256::from(TOKEN_OWNER_AMOUNT_1)
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
//         U256::from(TOKEN_OWNER_AMOUNT_2)
//     );
//     let total_supply_before_mint = erc1155_check_total_supply(&mut builder, &test_contract);

//     let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         test_contract,
//         METHOD_MINT,
//         runtime_args! {
//             ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
//             ARG_AMOUNT => mint_amount,
//         },
//     )
//     .build();

//     builder.exec(mint_request).expect_success().commit();

//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
//         U256::from(TOKEN_OWNER_AMOUNT_1) + mint_amount,
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
//         U256::from(TOKEN_OWNER_AMOUNT_2)
//     );

//     let total_supply_after_mint = erc1155_check_total_supply(&mut builder, &test_contract);
//     assert_eq!(
//         total_supply_after_mint,
//         total_supply_before_mint + mint_amount,
//     );
//     let total_supply_before_burn = total_supply_after_mint;

//     let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         test_contract,
//         METHOD_BURN,
//         runtime_args! {
//             ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
//             ARG_AMOUNT => mint_amount,
//         },
//     )
//     .build();

//     builder.exec(burn_request).expect_success().commit();

//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
//         U256::from(TOKEN_OWNER_AMOUNT_1),
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
//         U256::from(TOKEN_OWNER_AMOUNT_2)
//     );
//     let total_supply_after_burn = erc1155_check_total_supply(&mut builder, &test_contract);
//     assert_eq!(
//         total_supply_after_burn,
//         total_supply_before_burn - mint_amount,
//     );

//     assert_eq!(total_supply_after_burn, total_supply_before_mint);
// }
// #[test]
// fn test_should_not_burn_above_balance() {
//     let mint_amount = U256::MAX;

//     let (mut builder, TestContext { test_contract, .. }) = setup();
//     assert_eq!(
//         erc1155_check_balance_of(
//             &mut builder,
//             &test_contract,
//             Key::Account(*DEFAULT_ACCOUNT_ADDR)
//         ),
//         U256::from(TOKEN_TOTAL_SUPPLY),
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
//         U256::from(TOKEN_OWNER_AMOUNT_1)
//     );
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_2),
//         U256::from(TOKEN_OWNER_AMOUNT_2)
//     );

//     let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         test_contract,
//         METHOD_BURN,
//         runtime_args! {
//             ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
//             ARG_AMOUNT => mint_amount,
//         },
//     )
//     .build();

//     builder.exec(mint_request).commit();

//     let error = builder.get_error().expect("should have error");
//     assert!(
//         matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_BALANCE),
//         "{:?}",
//         error
//     );
// }

// #[test]
// fn test_should_not_mint_above_limits() {
//     let mint_amount = U256::MAX;

//     let (mut builder, TestContext { test_contract, .. }) = setup();
//     assert_eq!(
//         erc1155_check_balance_of(&mut builder, &test_contract, TOKEN_OWNER_ADDRESS_1),
//         U256::from(TOKEN_OWNER_AMOUNT_1)
//     );

//     let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
//         *DEFAULT_ACCOUNT_ADDR,
//         test_contract,
//         METHOD_MINT,
//         runtime_args! {
//             ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
//             ARG_AMOUNT => mint_amount,
//         },
//     )
//     .build();

//     builder.exec(mint_request).commit();

//     let error = builder.get_error().expect("should have error");
//     assert!(
//         matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_OVERFLOW),
//         "{:?}",
//         error
//     );
// }
// #[test]
// fn should_have_correct_balance_after_own_transfer_from() {
//     let (mut builder, TestContext { erc1155_token, .. }) = setup();

//     let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let spender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
//     let recipient = Key::Account(*DEFAULT_ACCOUNT_ADDR);

//     let allowance_amount = U256::from(ALLOWANCE_AMOUNT_1);
//     let transfer_amount = U256::from(TRANSFER_AMOUNT_1);

//     let approve_request =
//         make_erc1155_approve_request(sender, &erc1155_token, spender, allowance_amount);

//     builder.exec(approve_request).expect_success().commit();

//     let spender_allowance_before = erc1155_check_allowance_of(&mut builder, owner, spender);

//     let sender_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, sender);
//     let recipient_balance_before = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient);

//     assert_eq!(sender_balance_before, recipient_balance_before);

//     let transfer_from_request = {
//         let erc1155_transfer_from_args = runtime_args! {
//             ARG_OWNER => owner,
//             ARG_RECIPIENT => recipient,
//             ARG_AMOUNT => transfer_amount,
//         };
//         ExecuteRequestBuilder::contract_call_by_hash(
//             sender.into_account().unwrap(),
//             erc1155_token,
//             METHOD_TRANSFER_FROM,
//             erc1155_transfer_from_args,
//         )
//         .build()
//     };

//     builder
//         .exec(transfer_from_request)
//         .expect_success()
//         .commit();

//     let sender_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, sender);
//     assert_eq!(sender_balance_before, sender_balance_after);

//     let recipient_balance_after = erc1155_check_balance_of(&mut builder, &erc1155_token, recipient);
//     assert_eq!(recipient_balance_before, recipient_balance_after);

//     assert_eq!(sender_balance_after, recipient_balance_after);

//     let spender_allowance_after = erc1155_check_allowance_of(&mut builder, owner, spender);
//     assert_eq!(
//         spender_allowance_after,
//         spender_allowance_before - transfer_amount
//     );
// }