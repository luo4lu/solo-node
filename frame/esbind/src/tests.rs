use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use frame_system::InitKind;
use sp_core::crypto::AccountId32;
use pallet_evm::{AddressMapping, Config};
use sp_core::ecdsa;

pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);

#[test]
fn e2sbind_get_correct_balance() {
	ExtBuilder::default().set_accounts(|| {
		let mut accounts = BTreeMap::new();
		accounts.insert(
			H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap(),
			GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1000000),
				storage: Default::default(),
				code: vec![
				],
			},
		);
	accounts}
	).build().execute_with(|| {
		let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let substrate_addr = <Test as Config>::AddressMapping::into_account_id(eth_addr);

		//assert_eq!(format!("{}",substrate_addr), "5E1Q88ndNYvQ2yuNR9JxVZutTrCvkotjasw3i9CkNufamqqW");
		assert_eq!(substrate_addr.to_string(), "5E1Q88ndNYvQ2yuNR9JxVZutTrCvkotjasw3i9CkNufamqqW");
		assert_eq!(Balances::free_balance(&substrate_addr), 1000000);
	});
}
#[test]
fn e2sbind_get_correct_address() {
	ExtBuilder::default().set_accounts(|| {
		let mut accounts = BTreeMap::new();
		accounts.insert(
			H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap(),
			GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1000000),
				storage: Default::default(),
				code: vec![
				],
			},
		);
		accounts}
	).set_map(
		vec![
			(H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap(),
			 get_accountid_from_str("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")),
		],
		vec![],
	).build().execute_with(|| {
		let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let substrate_addr = <Test as Config>::AddressMapping::into_account_id(eth_addr);

		assert_eq!(substrate_addr.to_string(), "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
		assert_ne!(Balances::free_balance(&substrate_addr), 1000000);
	});
}

#[test]
fn e2sbind_eth_sig_and_recover() {
	ExtBuilder::default().build().execute_with(|| {
		System::initialize(
			&1,
			&[3u8; 32].into(),
			&Default::default(),
			InitKind::Full,
		);
		//the secrect is random
		let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
		let sig = ESBind::eth_sign(&secrect);
		let result = ESBind::eth_recover(&sig);
		assert_eq!(result,Some(H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap()));
	});
}
#[test]
fn e2sbind_test_error_param() {
	ExtBuilder::default().build().execute_with(|| {
		let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
		//The eth address is error!
		let eth_addr = H160::from_str("2B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let sig = ESBind::eth_sign(&secrect);
		assert_noop!(ESBind::bind_account(Origin::signed(ALICE), eth_addr, sig), Error::<Test>::ParamNotMatch);
	});
}

#[test]
fn e2sbind_test_bad_signature() {
	ExtBuilder::default().build().execute_with(|| {
		let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
		let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let mut sig:ecdsa::Signature = ESBind::eth_sign(&secrect);
		sig.0[64] = 0xFF;
		assert_noop!(ESBind::bind_account(Origin::signed(ALICE), eth_addr, sig), Error::<Test>::SignatureError);
	});
}

#[test]
fn e2sbind_test_already_bind() {
	ExtBuilder::default().set_map(
		vec![
			(H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap(),
			 get_accountid_from_str("5GuiPHn4eeiFEd9a1uB6gVwy9dXc5X4qeX5TM2CpJ69pSZzx")),
		],
		vec![],
	).build().execute_with(|| {
		let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
		let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let sig = ESBind::eth_sign(&secrect);
		assert_noop!(ESBind::bind_account(Origin::signed(ALICE), eth_addr, sig), Error::<Test>::OtherBindAlreday);
	});
}
#[test]
fn e2sbind_test_bind_and_transfer() {
	ExtBuilder::default().set_accounts(|| {
		let mut accounts = BTreeMap::new();
		accounts.insert(
			H160::from_str("fc16585898a0e5c7cae3c373e7085f3072ecc582").unwrap(),
			GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1000000),
				storage: Default::default(),
				code: vec![
				],
			},
		);
		accounts}
	).build().execute_with(|| {
		let secrect = [0x47,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];
		let eth_addr = H160::from_str("fc16585898a0e5c7cae3c373e7085f3072ecc582").unwrap();
		let sig = ESBind::eth_sign(&secrect);
		assert_eq!(Balances::free_balance(ALICE), 0);
		assert_ok!(ESBind::bind_account(Origin::signed(ALICE), eth_addr, sig));
		assert_eq!(Balances::free_balance(ALICE), 1000000);
	});
}
#[test]
fn e2sbind_send_to_sub_addr() {
	ExtBuilder::default().set_accounts(|| {
		let mut accounts = BTreeMap::new();
		accounts.insert(
			H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap(),
			GenesisAccount {
				nonce: U256::from(1),
				balance: U256::from(1000000),
				storage: Default::default(),
				code: vec![
				],
			},
		);
		accounts}
	).build().execute_with(|| {
		let eth_addr = H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315").unwrap();
		let except_addr = "5E1Q88ndNYvQ2yuNR9JxVZutTrCvkotjasw3i9CkNufamqqW";
		let secrect = [0x46,0x43,0x1a,0x5e,0xe6,0x2b,0x48,0x0e,0x0e,0x51,0xdb,0xf7,0xf4,0xee,0x48,0xb4,0xd7,0xf2,0xf4,0xbe,0x3b,0x65,0x01,0xb3,0x58,0x2a,0x21,0x89,0xa7,0xfe,0x57,0xb9];

		let substrate_addr = <Test as Config>::AddressMapping::into_account_id(eth_addr);
		System::initialize(
			&1,
			&[3u8; 32].into(),
			&Default::default(),
			InitKind::Full,
		);
		let sig = ESBind::eth_sign(&secrect);

		assert_eq!(substrate_addr.to_string(), except_addr);
		assert_eq!(Balances::free_balance(&substrate_addr), 1000000);

		assert_ok!(ESBind::bind_account(Origin::signed(ALICE), eth_addr, sig));
		assert_eq!(Balances::free_balance(ALICE), 1000000);
	});
}