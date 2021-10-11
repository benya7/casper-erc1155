#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use crate::test_fixture::{Sender, TestFixture};
    use casper_types::{Key, U256};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_uri(), TestFixture::URI);
    }

    #[test]
    fn should_read_total_supply() {
        let mut fixture = TestFixture::install_contract();

        let id_1 = "1";
        let mint_amount_1 = U256::from(42);
        let total_supply_1_before = fixture.total_supply(id_1).unwrap_or_default();

        let id_2 = "2";
        let mint_amount_2 = U256::from(72);
        let total_supply_2_before = fixture.total_supply(id_1).unwrap_or_default();

        assert_eq!(total_supply_1_before, U256::zero());
        fixture.mint(
            Key::from(fixture.bob),
            id_1,
            mint_amount_1,
            Sender(fixture.ali),
        );
        let total_supply_1_after = fixture.total_supply(id_1).unwrap();
        assert_eq!(total_supply_1_after, mint_amount_1);

        assert_eq!(total_supply_2_before, U256::zero());
        fixture.mint(
            Key::from(fixture.joe),
            id_2,
            mint_amount_2,
            Sender(fixture.bob),
        );
        let total_supply_2_after = fixture.total_supply(id_2).unwrap();
        assert_eq!(total_supply_2_after, mint_amount_2);
    }

    #[test]
    fn should_read_balance() {
        let mut fixture = TestFixture::install_contract();

        let token_id = "1";
        let mint_amount_joe = U256::from(70);

        fixture.mint(
            Key::from(fixture.joe),
            token_id,
            mint_amount_joe,
            Sender(fixture.ali),
        );
        let to_balance_after = fixture
            .balance_of(Key::from(fixture.joe), token_id)
            .unwrap_or_default();
        assert_eq!(to_balance_after, mint_amount_joe);
    }
    #[test]
    fn should_read_balance_batch() {
        let mut fixture = TestFixture::install_contract();

        let token_i_1 = "1";
        let token_i_2 = "2";
        let mint_amount_ali = U256::from(26);
        let mint_amount_bob = U256::from(42);
        let mint_amount_joe = U256::from(70);

        fixture.mint(
            Key::from(fixture.ali),
            token_i_1,
            mint_amount_ali,
            Sender(fixture.ali),
        );
        fixture.mint(
            Key::from(fixture.bob),
            token_i_2,
            mint_amount_bob,
            Sender(fixture.ali),
        );
        fixture.mint(
            Key::from(fixture.joe),
            token_i_1,
            mint_amount_joe,
            Sender(fixture.ali),
        );

        let mut accounts: Vec<Key> = Vec::new();
        let mut ids: Vec<String> = Vec::new();

        accounts.push(Key::from(fixture.ali));
        accounts.push(Key::from(fixture.bob));
        accounts.push(Key::from(fixture.joe));

        ids.push(String::from("1"));
        ids.push(String::from("2"));
        ids.push(String::from("1"));

        let to_balance_after = fixture.balance_of_batch(accounts, ids).unwrap_or_default();
        let mut mint_amounts: Vec<U256> = Vec::new();
        mint_amounts.push(mint_amount_ali);
        mint_amounts.push(mint_amount_bob);
        mint_amounts.push(mint_amount_joe);
        assert_eq!(to_balance_after, mint_amounts);
    }

    #[test]
    fn should_set_approval_for_all() {
        let mut fixture = TestFixture::install_contract();
        //let id = "1";
        //let mint_amount = U256::from(42);
        // let transfer_amount = U256::from(15);
        //fixture.mint(Key::from(fixture.ali), id, mint_amount, Sender(fixture.ali));
        let approved_before = fixture
            .is_approval_for_all(Key::from(fixture.ali), Key::from(fixture.bob))
            .unwrap_or_default();
        assert_eq!(approved_before, false);

        fixture.set_approval_for_all(Key::from(fixture.bob), true, Sender(fixture.ali));
        let approved_after = fixture
            .is_approval_for_all(Key::from(fixture.ali), Key::from(fixture.bob))
            .unwrap();
        assert_eq!(approved_after, true);
        // fixture.safe_transfer_from(Key::from(fixture.ali), Key::from(fixture.joe), id, transfer_amount, Sender(fixture.ali));
        //let balance_before = fixture.balance_of(Key::from(fixture.ali), id).unwrap_or_default();
        //assert_eq!(balance_before, mint_amount);
    }

    #[test]
    fn should_safe_transfer_from() {
        let mut fixture = TestFixture::install_contract();
        let token_id = "1";
        //     assert_eq!(fixture.balance_of(Key::from(fixture.bob), String::from("1")), None);
        //     let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali), String::from("1")).unwrap();
        //     let transfer_amount_1 = U256::from(42);
        let mint_amount_joe = U256::from(100);
        let to_balance_before = fixture
            .balance_of(Key::from(fixture.joe), token_id)
            .unwrap_or_default();
        assert_eq!(to_balance_before, U256::zero());

        fixture.mint(
            Key::from(fixture.joe),
            token_id,
            mint_amount_joe,
            Sender(fixture.ali),
        );
        let to_balance_after = fixture
            .balance_of(Key::from(fixture.joe), token_id)
            .unwrap_or_default();
        assert_eq!(to_balance_after, mint_amount_joe);


    }
    //     
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.bob), String::from("1")),
    //         Some(transfer_amount_1)
    //     );
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.ali), String::from("1")),
    //         Some(initial_ali_balance - transfer_amount_1)
    //     );

    //     let transfer_amount_2 = U256::from(20);
    //     fixture.safe_transfer_from(
    //         Key::from(fixture.ali),
    //         String::from("1"),
    //         transfer_amount_2,
    //         Sender(fixture.bob),
    //     );
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.ali), String::from("1")),
    //         Some(initial_ali_balance - transfer_amount_1 + transfer_amount_2)
    //     );
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.bob), String::from("1")),
    //         Some(transfer_amount_1 - transfer_amount_2)
    //     );
    // }

    // #[test]
    // fn should_transfer_full_amount() {
    //     let mut fixture = TestFixture::install_contract();

    //     let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali), String::from("1")).unwrap();
    //     assert_eq!(fixture.balance_of(Key::from(fixture.bob), String::from("1")), None);

    //     fixture.safe_transfer_from(
    //         Key::from(fixture.bob),
    //         String::from("1"),
    //         initial_ali_balance,
    //         Sender(fixture.ali),
    //     );

    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.bob), String::from("1")),
    //         Some(initial_ali_balance)
    //     );
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.ali), String::from("1")),
    //         Some(U256::zero())
    //     );

    //     fixture.safe_transfer_from(
    //         Key::from(fixture.ali),
    //         String::from("1"),
    //         initial_ali_balance,
    //         Sender(fixture.bob),
    //     );

    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.bob), String::from("1")),
    //         Some(U256::zero())
    //     );
    //     assert_eq!(
    //         fixture.balance_of(Key::from(fixture.ali), String::from("1")),
    //         Some(initial_ali_balance)
    //     );
    // }

    // #[should_panic(expected = "ApiError::User(65534) [131070]")]
    // #[test]
    // fn should_not_transfer_with_insufficient_balance() {
    //     let mut fixture = TestFixture::install_contract();

    //     let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali), String::from("1")).unwrap();
    //     assert_eq!(fixture.balance_of(Key::from(fixture.bob), String::from("1")), None);

    //     fixture.safe_transfer_from(
    //         Key::from(fixture.bob),
    //         String::from("1"),
    //         initial_ali_balance + U256::one(),
    //         Sender(fixture.ali),
    //     );
    // }

    // fn should_set_operator() {

    //     let mut fixture = TestFixture::install_contract();
    //     let owner = fixture.ali;
    //     let operator = fixture.bob;
    //     fixture.set_approval_for_all(Key::from(operator), true, Sender(owner));
    //     assert_eq!(
    //         fixture.is_approval_for_all(Key::from(owner), Key::from(operator)),
    //         Some(true)
    //     );
    // }

    // #[test]
    //     #[should_panic(expected = "ApiError::MissingArgument")]
    //     fn should_error_on_missing_runtime_arg() {
    //         let secret_key = SecretKey::ed25519_from_bytes(MY_ACCOUNT).unwrap();
    //         let public_key = PublicKey::from(&secret_key);
    //         let account_addr = AccountHash::from(&public_key);

    //         let mut context = TestContextBuilder::new()
    //             .with_public_key(public_key, U512::from(500_000_000_000_000_000u64))
    //             .build();

    //         let session_code = Code::from(CONTRACT_WASM);
    //         let session_args = RuntimeArgs::new();
    //         let session = SessionBuilder::new(session_code, session_args)
    //             .with_address(account_addr)
    //             .with_authorization_keys(&[account_addr])
    //             .build();

    //         context.run(session);
    //     }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
