// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{AccountId, BalancesConfig, RuntimeGenesisConfig, SudoConfig};
use alloc::{vec, vec::Vec};
use frame_support::build_struct_json_patch;
use serde_json::Value;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_genesis_builder::{self, PresetId};
use sp_core::crypto::AccountId32;
use sp_keyring::Sr25519Keyring;

// Returns the genesis config presets populated with given parameters.
fn testnet_genesis(
	initial_authorities: Vec<GrandpaId>,
	endowed_accounts: Vec<AccountId>,
	root: AccountId,
) -> Value {
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 1u128 << 60))
				.collect::<Vec<_>>(),
		},
		grandpa: pallet_grandpa::GenesisConfig {
			authorities: initial_authorities.iter().map(|x| (x.clone(), 1)).collect::<Vec<_>>(),
		},
		sudo: SudoConfig { key: Some(root) },
	})
}

/// Return the development genesis config.
pub fn development_config_genesis() -> Value {
	testnet_genesis(
		vec![sp_keyring::Ed25519Keyring::Alice.public().into()],
		vec![
			Sr25519Keyring::Alice.to_account_id(),
			Sr25519Keyring::Bob.to_account_id(),
			Sr25519Keyring::AliceStash.to_account_id(),
			Sr25519Keyring::BobStash.to_account_id(),
		],
		sp_keyring::Sr25519Keyring::Alice.to_account_id(),
	)
}

/// Return the local genesis config preset.
pub fn local_config_genesis() -> Value {
	testnet_genesis(
		vec![
			sp_keyring::Ed25519Keyring::Alice.public().into(),
			sp_keyring::Ed25519Keyring::Bob.public().into(),
		],
		Sr25519Keyring::iter()
			.filter(|v| v != &Sr25519Keyring::One && v != &Sr25519Keyring::Two)
			.map(|v| v.to_account_id())
			.collect::<Vec<_>>(),
		Sr25519Keyring::Alice.to_account_id(),
	)
}

/// Plicata Alpha genesis preset.
pub fn alpha_config_genesis() -> Value {
        const UNIT: u128 = 1_000_000_000_000;

        let grandpa_authority = GrandpaId::from(
                sp_core::ed25519::Public::from_raw([
                        0x2d, 0x69, 0x6b, 0x24, 0x4b, 0xc9, 0xfd, 0x5f,
                        0x8c, 0x53, 0xa4, 0xf5, 0x5f, 0x2b, 0x86, 0x0f,
                        0x69, 0xb3, 0x9f, 0xff, 0x24, 0xe1, 0x5d, 0xc1,
                        0x44, 0x36, 0xf7, 0x37, 0xc7, 0x7d, 0xf8, 0x44,
                ])
        );

        let seedbank: AccountId = AccountId32::new([
                0xd8, 0xd4, 0xf0, 0x4a, 0x4e, 0xb5, 0xbd, 0x3f,
                0x54, 0x9c, 0x73, 0x0b, 0x69, 0xb9, 0xb7, 0x0e,
                0x2b, 0x82, 0xa5, 0x5c, 0x4d, 0x23, 0x64, 0xfd,
                0xbe, 0xfe, 0x1f, 0x02, 0xc9, 0xe0, 0x60, 0x7b,
        ]).into();

        build_struct_json_patch!(RuntimeGenesisConfig {
                balances: BalancesConfig {
                        balances: vec![(seedbank.clone(), 1_000 * UNIT)],
                },
                grandpa: pallet_grandpa::GenesisConfig {
                        authorities: vec![(grandpa_authority, 1)],
                },
                sudo: SudoConfig {
                        key: Some(seedbank),
                },
        })
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::DEV_RUNTIME_PRESET => development_config_genesis(),
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => local_config_genesis(),
                "plicata_alpha" => alpha_config_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
                PresetId::from("plicata_alpha"),
	]
}
