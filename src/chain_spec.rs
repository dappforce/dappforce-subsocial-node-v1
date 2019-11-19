// Copyright 2019 Joystream Contributors
// This file is part of Joystream node.

// Joystream node is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Joystream node is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Joystream node.  If not, see <http://www.gnu.org/licenses/>.

use hex_literal::{hex, hex_impl};
use subsocial_runtime::{
    AccountId, BalancesConfig, ConsensusConfig,
    GenesisConfig, GrandpaConfig, IndicesConfig, Perbill,
    SessionConfig, StakerStatus, StakingConfig, SudoConfig, TimestampConfig,
};
use primitives::{crypto::UncheckedInto, ed25519, sr25519, Pair};
use substrate_service;
use substrate_telemetry::TelemetryEndpoints;

use ed25519::Public as AuthorityId;

// Note this is the URL for the telemetry server
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialised `ChainSpec`. This is a specialisation of the general Substrate ChainSpec type.
pub type ChainSpec = substrate_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
    /// Whatever the current runtime is, with just Alice as an auth.
    Development,
    /// Whatever the current runtime is, with simple Alice/Bob auths.
    LocalTestnet,
    /// Staging testnet
    StagingTestnet,
}

fn authority_key(s: &str) -> AuthorityId {
    ed25519::Pair::from_string(&format!("//{}", s), None)
        .expect("static values are valid; qed")
        .public()
}

fn account_key(s: &str) -> AccountId {
	sr25519::Pair::from_string(&format!("//{}", s), None)
		.expect("static values are valid; qed")
		.public()
}

impl Alternative {
    /// Get an actual chain config from one of the alternatives.
    pub(crate) fn load(self) -> Result<ChainSpec, String> {
        Ok(match self {
            Alternative::Development => ChainSpec::from_genesis(
                "Development",
                "dev",
                || {
                    testnet_genesis(
                        vec![
                            // stash, controller, authority
                            (
                                account_key("Alice//stash"),
                                account_key("Alice"),
                                authority_key("Alice"),
                            ),
                        ],
                        vec![
                            // endowed account
                            account_key("Alice"),
                            account_key("Bob"),
                            account_key("Charlie"),
                            account_key("Dave"),
                            account_key("Eve"),
                            account_key("Ferdie"),
                        ],
                        // sudo key
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
            Alternative::LocalTestnet => ChainSpec::from_genesis(
                "Local Testnet",
                "local_testnet",
                || {
                    testnet_genesis(
                        vec![
                            (
                                account_key("Alice//stash"),
                                account_key("Alice"),
                                authority_key("Alice"),
                            ),
                            (
                                account_key("Bob//stash"),
                                account_key("Bob"),
                                authority_key("Bob"),
                            ),
                        ],
                        vec![
                            account_key("Alice"),
                        ],
                        account_key("Alice"),
                    )
                },
                vec![],
                None,
                None,
                None,
                None,
            ),
            Alternative::StagingTestnet => staging_testnet_config(),
        })
    }

    pub(crate) fn from(s: &str) -> Option<Self> {
        match s {
            "dev" => Some(Alternative::Development),
            "local" => Some(Alternative::LocalTestnet),
            "staging" => Some(Alternative::StagingTestnet),
            _ => None,
        }
    }
}

/// Staging testnet config
pub fn staging_testnet_config() -> ChainSpec {
    let boot_nodes = vec![];
    ChainSpec::from_genesis(
        "Subsocial Testnet",
        "subsocial_testnet_m3",
        staging_testnet_config_genesis,
        boot_nodes,
        Some(TelemetryEndpoints::new(vec![(
            STAGING_TELEMETRY_URL.to_string(),
            0,
        )])),
        None,
        None,
        None,
    )
}

fn staging_testnet_config_genesis() -> GenesisConfig {
    let initial_authorities: Vec<(AccountId, AccountId, AuthorityId)> = vec![(
        hex!["0610d1a2b1d704723e588c842a934737491688b18b052baae1286f12e96adb65"].unchecked_into(), // stash
        hex!["609cee3edd9900e69be44bcbf7a1892cad10408840a2d72d563811d72d9bb339"].unchecked_into(), // controller
        hex!["65179fd9c39ec301457d1ee47a13f3bb0fef65812a57b6c93212e609b10d35d2"].unchecked_into(), // session key
    )];
    let endowed_accounts = vec![hex![
        "0ae55282e669fc55cb9529c0b12b989f2c5bf636d0de7630b5a4850055ed9c30"
    ]
    .unchecked_into()];

    const CENTS: u128 = 1;
    const DOLLARS: u128 = 100 * CENTS;

    const SECS_PER_BLOCK: u64 = 4;
    const MINUTES: u64 = 60 / SECS_PER_BLOCK;
    // const HOURS: u64 = MINUTES * 60;
    // const DAYS: u64 = HOURS * 24;
    const STASH: u128 = 50 * DOLLARS;
    const ENDOWMENT: u128 = 100_000_000 * DOLLARS;

    GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../dappforce-subsocial-runtime/wasm/target/wasm32-unknown-unknown/release/subsocial_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			minimum_period: SECS_PER_BLOCK / 2, // due to the nature of aura the slots are 2*period
		}),
		indices: Some(IndicesConfig {
			ids: vec![],
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			existential_deposit: 0,
			transfer_fee: 0,
			creation_fee: 0,
			vesting: vec![],
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		}),
		sudo: Some(SudoConfig {
			key: endowed_accounts[0].clone(),
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			session_length: 10 * MINUTES,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			offline_slash: Perbill::from_millionths(10_000),  // 1/ 100 => 1%
			session_reward: Perbill::from_millionths(1_000),  // 1/1000 => 0.1% (min stake -> 1000 units for reward to be GT 0)
			current_session_reward: 0,
			validator_count: 20,
			sessions_per_era: 6,
			bonding_duration: 1, // Number of ERAs
			offline_slash_grace: 4,
			minimum_validator_count: 1,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		})
	}
}

fn testnet_genesis(
    initial_authorities: Vec<(AccountId, AccountId, AuthorityId)>,
    endowed_accounts: Vec<AccountId>,
    root_key: AccountId,
) -> GenesisConfig {
    const STASH: u128 = 100;
    const ENDOWMENT: u128 = 100_000_000;

    GenesisConfig {
		consensus: Some(ConsensusConfig {
			code: include_bytes!("../dappforce-subsocial-runtime/wasm/target/wasm32-unknown-unknown/release/subsocial_runtime_wasm.compact.wasm").to_vec(),
			authorities: initial_authorities.iter().map(|x| x.2.clone()).collect(),
		}),
		system: None,
		timestamp: Some(TimestampConfig {
			minimum_period: 1,                    // 3*2=6 second block time.
		}),
		indices: Some(IndicesConfig {
			ids: vec![]
		}),
		balances: Some(BalancesConfig {
			existential_deposit: 0,
			transfer_fee: 0,
			creation_fee: 0,
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
			transaction_base_fee: 1,
			transaction_byte_fee: 0,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		session: Some(SessionConfig {
			validators: initial_authorities.iter().map(|x| x.1.clone()).collect(),
			session_length: 10,
			keys: initial_authorities.iter().map(|x| (x.1.clone(), x.2.clone())).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			minimum_validator_count: 1,
			validator_count: 2,
			sessions_per_era: 5,
			bonding_duration: 1, // Number of Eras
			offline_slash: Perbill::zero(),
			session_reward: Perbill::zero(),
			current_session_reward: 0,
			offline_slash_grace: 0,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.1.clone()).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.2.clone(), 1)).collect(),
		})
	}
}
