use cycan_runtime::{
	AccountId, BabeConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	Signature, SudoConfig, SystemConfig, WASM_BINARY,StakerStatus,{opaque::SessionKeys as SessionKeys}, DOLLARS, SessionConfig, StakingConfig,
	ContractsConfig, ImOnlineConfig,wasm_binary_unwrap,IndicesConfig,CouncilConfig,DemocracyConfig,ElectionsConfig,TechnicalCommitteeConfig
};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public, H160, U256};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
use sp_runtime::{Perbill};
use pallet_rgrandpa::AuthorityId as RGrandpaId;
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sc_cli::NodeKeyType::Ed25519;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;
const BETA_PROPERTIES: &str = r#"
{
    "ss58Format": 42,
    "tokenDecimals": 12,
    "tokenSymbol": "CYN"
}"#;
const BETA_PROTOCOL_ID: &str = "cyn";

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate a crypto pair from seed.
pub fn my_get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate an account ID from seed.
pub fn my_get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
	where
		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(my_get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId, RGrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<RGrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

pub fn my_authority_keys_from_seed(index:&str,seed: &str) -> (AccountId, AccountId, BabeId, GrandpaId, RGrandpaId, ImOnlineId) {
	(
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}//stash", seed, index)),
		my_get_account_id_from_seed::<sr25519::Public>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<BabeId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<GrandpaId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<RGrandpaId>(&format!("{}//{}", seed, index)),
		my_get_from_seed::<ImOnlineId>(&format!("{}//{}", seed, index)),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					authority_keys_from_seed("Alice"),
					authority_keys_from_seed("Bob"),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}
pub fn beta_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Livenet wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"beta",
		// ID
		"beta",
		ChainType::Live,
		move || {
			beta_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					 my_authority_keys_from_seed("1","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("2","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("3","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("4","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("5","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("6","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("7","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
					 my_authority_keys_from_seed("8","conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				],
				// Sudo account
				my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth"),
				// Pre-funded accounts
				vec![
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//1//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//2//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//3//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//4//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//5//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//6//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//7//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//8//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//9//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//10//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//11//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//12//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//13//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//14//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//15//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//16//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//17//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//18//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//19//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//20//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//21//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//22//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//23//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//24//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//1"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//2"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//3"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//4"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//5"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//6"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//7"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//8"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//9"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//10"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//11"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//12"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//13"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//14"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//15"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//16"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//17"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//18"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//19"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//20"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//21"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//22"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//23"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//24"),
				],
				vec![
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//9//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//10//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//11//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//12//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//13//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//14//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//15//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//16//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//17//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//18//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//19//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//20//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//21//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//22//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//23//stash"),
					my_get_account_id_from_seed::<sr25519::Public>("conduct enforce source exhibit inform rescue exercise rubber jeans swarm crisp wealth//24//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![
			// "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp",
			// "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWHdiAxVd8uMQR1hGWXccidmfCwLqcMpGwR6QcTP6QRMuD",
			// "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWSCufgHzV4fCwRijfH2k3abrpAJxTKxEvN1FDuRXA2U9x",
			// "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWSsChzF81YDUKpe9Uk5AHV5oqAaXAcWNSPYgoLauUk4st",
		],
		// Telemetry
		None,
		// Protocol ID
		Some(BETA_PROTOCOL_ID),
		// Properties
		serde_json::from_str(BETA_PROPERTIES).unwrap(),
		// Extensions
		None,
	))
}

fn beta_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, RGrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	nominate_accounts: Vec<AccountId>,
	enable_println: bool,
) -> GenesisConfig {
	const INITIAL_STAKING: u128 = 10_000 * DOLLARS;
	const ENDOWMENT: u128 = 10_000 * DOLLARS;
	const STASH: u128 = ENDOWMENT / 1000;
	let num_nominate_accounts = nominate_accounts.len();
	let num_endowed_accounts = endowed_accounts.len();
	let mut tem_count = 0;
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 10_000_000 * DOLLARS))
				.collect(),
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_indices: Some(IndicesConfig { indices: vec![] }),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				log::info!("========================================{},{},{},{},{},{}",x.0.clone(),x.1.clone(),x.2.clone(),x.3.clone(),x.4.clone(),x.5.clone());

				(x.1.clone(),
				 x.0.clone(),
				 SessionKeys { babe:  x.2.clone(), grandpa: x.3.clone(), rgrandpa: x.4.clone(), im_online:x.5.clone()}
				)
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 4,
			stakers: initial_authorities
				.iter()
				.map(|x| {
					if tem_count < 4 {
						tem_count+=1;
						(x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Validator)
					} else {
						tem_count+=1;
						(x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Nominator(nominate_accounts.clone()))
					}
				})
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		pallet_evm: Some(EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		}),
		pallet_ethereum: Some(EthereumConfig {}),
		pallet_dynamic_fee: Some(Default::default()),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println,
				..Default::default()
			},
		}),
		pallet_treasury: Some(Default::default()),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig {
			members: nominate_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		}),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
	}
}



/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, RGrandpaId, ImOnlineId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool,
) -> GenesisConfig {
	const INITIAL_STAKING: u128 = 10_000 * DOLLARS;
	const ENDOWMENT: u128 = 10_000 * DOLLARS;
	const STASH: u128 = ENDOWMENT / 1000;

	let num_endowed_accounts = endowed_accounts.len();
	GenesisConfig {
		frame_system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 10_000_000 * DOLLARS))
				.collect(),
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				log::info!("========================================{},{},{},{},{}",x.0.clone(),x.1.clone(),x.2.clone(),x.3.clone(),x.4.clone());
				(x.1.clone(),
				 x.0.clone(),
				 SessionKeys { babe:  x.2.clone(), grandpa: x.3.clone(), rgrandpa: x.4.clone(), im_online: x.5.clone()}
				)
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), INITIAL_STAKING, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		pallet_sudo: Some(SudoConfig {
			key: root_key,
		}),
		pallet_evm: Some(EVMConfig {
			accounts: {
				let mut map = BTreeMap::new();
				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
					H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("1B191594ad9730eDE7cCe7801A1C853557Eb0315")
						.expect("internal H160 is valid; qed"),
					pallet_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
		}),
		pallet_ethereum: Some(EthereumConfig {}),
		pallet_dynamic_fee: Some(Default::default()),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		}),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		}),
	}
}