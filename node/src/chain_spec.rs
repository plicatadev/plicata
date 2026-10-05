use sc_service::{config::MultiaddrWithPeerId, ChainType};
use plicata_runtime::WASM_BINARY;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec;

pub fn development_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_preset_name(sp_genesis_builder::DEV_RUNTIME_PRESET)
	.build())
}

pub fn local_chain_spec() -> Result<ChainSpec, String> {
	Ok(ChainSpec::builder(
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?,
		None,
	)
	.with_name("Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_preset_name(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET)
	.build())
}

pub fn alpha_chain_spec() -> Result<ChainSpec, String> {
        Ok(ChainSpec::builder(
                WASM_BINARY.ok_or_else(|| "Plicata Alpha wasm not available".to_string())?,
                None,
        )
        .with_name("Plicata Alpha")
        .with_id("plicata_alpha")
        .with_chain_type(ChainType::Live)
        .with_boot_nodes(vec![
                "/ip4/151.145.33.210/tcp/30333/p2p/12D3KooWJvPV8ThXEX6Sf7HLYGLozfb8w5a2Ymhic1fxE3vhF9wv"
                        .parse::<MultiaddrWithPeerId>()
                        .map_err(|e| format!("Invalid Plicata Alpha bootnode: {e}"))?,
        ])
        .with_genesis_config_preset_name("plicata_alpha")
        .build())
}
