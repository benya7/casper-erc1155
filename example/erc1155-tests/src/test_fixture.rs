use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc1155::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};

const CONTRACT_ERC1155_TOKEN: &str = "erc1155_token.wasm";
const CONTRACT_KEY_NAME: &str = "erc1155_token_contract";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}
impl TestFixture {
    pub const URI: &'static str = "https://myuri-example.com";

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_ERC1155_TOKEN);
        let session_args = runtime_args! {
          consts::URI_RUNTIME_ARG_NAME => TestFixture::URI,
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn uri(&self) -> String {
        self.query_contract(consts::URI_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn total_supply(&self, id: &str) -> Option<U256> {
        let item_key = format!("total_supply_{}", id);

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::TOTAL_SUPPLY_KEY_NAME.to_string()),
                item_key,
            )
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn balance_of(&self, account: Key, id: &str) -> Option<U256> {
        let mut preimage = Vec::new();

        preimage.append(&mut id.to_bytes().unwrap());
        preimage.append(&mut account.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let balance_key = base64::encode(key_bytes);

        let key = Key::Hash(self.contract_hash().value());
        let balance = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::BALANCES_KEY_NAME.to_string()),
                balance_key,
            )
            .ok()?;

        Some(balance.into_t::<U256>().unwrap_or_default())
    }

    pub fn balance_of_batch(&self, accounts: Vec<Key>, ids: Vec<String>) -> Option<Vec<U256>> {
        let mut balances = Vec::new();
        let key = Key::Hash(self.contract_hash().value());
        for (i, _) in accounts.iter().enumerate() {
            let mut preimage = Vec::new();
            preimage.append(&mut ids[i].to_bytes().unwrap());
            preimage.append(&mut accounts[i].to_bytes().unwrap());
            let key_bytes = blake2b256(&preimage);
            let balance_key = base64::encode(key_bytes);
            let balance = self
                .context
                .query_dictionary_item(
                    key,
                    Some(consts::BALANCES_KEY_NAME.to_string()),
                    balance_key,
                )
                .ok()?;
            balances.push(balance.into_t::<U256>().unwrap());
        }
        Some(balances)
    }

    pub fn set_approval_for_all(&mut self, operator: Key, approved: bool, sender: Sender) {
        self.call(
            sender,
            consts::SET_APPROVAL_FOR_ALL_ENTRY_POINT_NAME,
            runtime_args! {
              consts::OPERATOR_RUNTIME_ARG_NAME => operator,
              consts::APPROVED_RUNTIME_ARG_NAME => approved,
            },
        )
    }

    pub fn is_approval_for_all(&self, account: Key, operator: Key) -> Option<bool> {
        let mut preimage = Vec::new();
        preimage.append(&mut account.to_bytes().unwrap());
        preimage.append(&mut operator.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let approved_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash().value());

        let approved = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::OPERATORS_KEY_NAME.to_string()),
                approved_item_key,
            )
            .ok()?;
        Some(approved.into_t::<bool>().unwrap())
    }

    pub fn safe_transfer_from(
        &mut self,
        from: Key,
        to: Key,
        id: &str,
        amount: U256,
        sender: Sender,
    ) {
        self.call(
            sender,
            consts::SAFE_TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::FROM_RUNTIME_ARG_NAME => from,
                consts::RECIPIENT_RUNTIME_ARG_NAME => to,
                consts::TOKEN_ID_RUNTIME_ARG_NAME => id,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn safe_batch_transfer_from(
        &mut self,
        from: Key,
        to: Key,
        ids: Vec<String>,
        amounts: Vec<U256>,
        sender: Sender,
    ) {
        self.call(
            sender,
            consts::SAFE_BATCH_TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::FROM_RUNTIME_ARG_NAME => from,
                consts::RECIPIENT_RUNTIME_ARG_NAME => to,
                consts::TOKEN_IDS_RUNTIME_ARG_NAME => ids,
                consts::AMOUNTS_RUNTIME_ARG_NAME => amounts
            },
        );
    }

    pub fn mint(&mut self, to: Key, id: &str, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::MINT_ENTRY_POINT_NAME,
            runtime_args! {
              consts::RECIPIENT_RUNTIME_ARG_NAME => to,
                consts::TOKEN_ID_RUNTIME_ARG_NAME => id,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn burn(&mut self, owner: Key, id: &str, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::BURN_ENTRY_POINT_NAME,
            runtime_args! {
              consts::OWNER_RUNTIME_ARG_NAME => owner,
                consts::TOKEN_ID_RUNTIME_ARG_NAME => id,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }
}
