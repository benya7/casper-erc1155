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

        let id_2 = "2";
        let mint_amount_2 = U256::from(72);

        assert_eq!(fixture.total_supply(id_1), None);
        assert_eq!(fixture.total_supply(id_2), None);

        fixture.mint(
            Key::from(fixture.bob),
            id_1,
            mint_amount_1,
            Sender(fixture.ali),
        );

        assert_eq!(fixture.total_supply(id_1), Some(mint_amount_1));

        fixture.mint(
            Key::from(fixture.joe),
            id_2,
            mint_amount_2,
            Sender(fixture.bob),
        );

        assert_eq!(fixture.total_supply(id_2), Some(mint_amount_2));
    }

    #[test]
    fn should_read_balance() {
        let mut fixture = TestFixture::install_contract();
        let id = "1";
        let total_supply = U256::from(100);
        fixture.mint(
            Key::from(fixture.ali),
            id,
            total_supply,
            Sender(fixture.ali),
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob), id), None);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id),
            Some(total_supply)
        );
        let transfer_amount = U256::from(35);
        fixture.safe_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.bob),
            id,
            transfer_amount,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob), id),
            Some(transfer_amount)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id),
            Some(total_supply - transfer_amount)
        );
    }

    #[test]
    fn should_read_balance_batch() {
        let mut fixture = TestFixture::install_contract();

        let token_id_1 = "1";
        let token_id_2 = "2";

        let mint_amount_ali = U256::from(26);
        let mint_amount_bob = U256::from(42);
        let mint_amount_joe = U256::from(70);

        let mut accounts: Vec<Key> = Vec::new();
        let mut ids: Vec<String> = Vec::new();

        fixture.mint(
            Key::from(fixture.ali),
            token_id_1,
            mint_amount_ali,
            Sender(fixture.ali),
        );
        fixture.mint(
            Key::from(fixture.bob),
            token_id_2,
            mint_amount_bob,
            Sender(fixture.ali),
        );
        fixture.mint(
            Key::from(fixture.joe),
            token_id_1,
            mint_amount_joe,
            Sender(fixture.ali),
        );

        accounts.push(Key::from(fixture.ali));
        accounts.push(Key::from(fixture.bob));
        accounts.push(Key::from(fixture.joe));

        ids.push(String::from("1"));
        ids.push(String::from("2"));
        ids.push(String::from("1"));

        let mint_amounts = vec![mint_amount_ali, mint_amount_bob, mint_amount_joe];
        assert_eq!(fixture.balance_of_batch(accounts, ids), Some(mint_amounts));
    }

    #[test]
    fn should_is_approval_for_all() {
        let mut fixture = TestFixture::install_contract();
        let id = "1";
        let mint_amount = U256::from(100);
        let transfer_amount = U256::from(35);
        fixture.mint(Key::from(fixture.ali), id, mint_amount, Sender(fixture.ali));
        let approved_before = fixture
            .is_approval_for_all(Key::from(fixture.ali), Key::from(fixture.bob))
            .unwrap_or_default();
        assert_eq!(approved_before, false);

        fixture.set_approval_for_all(Key::from(fixture.bob), true, Sender(fixture.ali));

        let approved_after = fixture
            .is_approval_for_all(Key::from(fixture.ali), Key::from(fixture.bob))
            .unwrap();
        assert_eq!(approved_after, true);

        fixture.safe_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.joe),
            id,
            transfer_amount,
            Sender(fixture.bob),
        );
        let from_balance_after = fixture
            .balance_of(Key::from(fixture.ali), id)
            .unwrap_or_default();
        let to_balance_after = fixture
            .balance_of(Key::from(fixture.joe), id)
            .unwrap_or_default();
        assert_eq!(from_balance_after, mint_amount - transfer_amount);
        assert_eq!(to_balance_after, transfer_amount);
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
    fn should_mint_and_read_total_supply() {
        let mut fixture = TestFixture::install_contract();
        let id = "1";
        let mint_amount = U256::from(100);

        fixture.mint(Key::from(fixture.joe), id, mint_amount, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.joe), id),
            Some(mint_amount)
        );
        assert_eq!(fixture.total_supply(id), Some(mint_amount));
    }

    #[test]
    fn should_burn_and_read_total_supply() {
        let mut fixture = TestFixture::install_contract();
        let id = "1";
        let mint_amount = U256::from(100);

        fixture.mint(Key::from(fixture.joe), id, mint_amount, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.joe), id),
            Some(mint_amount)
        );
        assert_eq!(fixture.total_supply(id), Some(mint_amount));

        fixture.burn(
            Key::from(fixture.joe),
            id,
            mint_amount - U256::from(1),
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.joe), id),
            Some(U256::from(1))
        );
        assert_eq!(fixture.total_supply(id), Some(U256::from(1)));
    }

    #[test]
    fn should_safe_transfer_from() {
        let mut fixture = TestFixture::install_contract();
        let mint_amount = U256::from(10000);
        let id = "1";
        fixture.mint(Key::from(fixture.ali), id, mint_amount, Sender(fixture.ali));
        //let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali), "1").unwrap();
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id),
            Some(mint_amount)
        );
        let transfer_amount = U256::from(35);
        fixture.safe_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.joe),
            id,
            transfer_amount,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.joe), id),
            Some(transfer_amount)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id),
            Some(mint_amount - transfer_amount)
        );
    }

    #[test]
    fn should_safe_batch_transfer_from() {
        let mut fixture = TestFixture::install_contract();
        let mint_amount = U256::from(10000);
        let id_1 = "1";
        let id_2 = "2";

        fixture.mint(
            Key::from(fixture.ali),
            id_1,
            mint_amount,
            Sender(fixture.ali),
        );
        fixture.mint(
            Key::from(fixture.ali),
            id_2,
            mint_amount,
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id_1),
            Some(mint_amount)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id_2),
            Some(mint_amount)
        );

        let transfer_amount = U256::from(35);
        let mut ids: Vec<String> = Vec::new();
        let mut amounts: Vec<U256> = Vec::new();

        ids.push(String::from("1"));
        ids.push(String::from("2"));
        amounts.push(transfer_amount);
        amounts.push(transfer_amount);

        fixture.safe_batch_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.bob),
            ids,
            amounts,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob), id_1),
            Some(transfer_amount)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob), id_2),
            Some(transfer_amount)
        );
    }

    #[test]
    fn should_not_safe_transfer_from_operator() {
        let mut fixture = TestFixture::install_contract();
        let mint_amount = U256::from(10000);
        let transfer_amount = U256::from(20);
        let id = "1";
        fixture.mint(Key::from(fixture.ali), id, mint_amount, Sender(fixture.ali));
        //let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali), "1").unwrap();
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali), id),
            Some(mint_amount)
        );
        assert_eq!(
            fixture.is_approval_for_all(Key::from(fixture.ali), Key::from(fixture.bob)),
            None
        );
        fixture.safe_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.joe),
            id,
            transfer_amount,
            Sender(fixture.bob),
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.joe), id), None);
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_not_transfer_with_insufficient_balance() {
        let mut fixture = TestFixture::install_contract();
        let id = "1";
        assert_eq!(fixture.balance_of(Key::from(fixture.ali), id), None);

        fixture.safe_transfer_from(
            Key::from(fixture.ali),
            Key::from(fixture.bob),
            id,
            U256::from(20),
            Sender(fixture.ali),
        );
    }
}
fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
