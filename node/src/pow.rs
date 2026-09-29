use sc_consensus_pow::{Error, PowAlgorithm};
use sp_core::U256;
use sp_runtime::traits::{Block as BlockT, Hash as HashT, BlakeTwo256};
use std::{thread, time::Duration};

/// Initial PoW target for the Plicata development network.
/// Higher targets make cultivation easier; lower targets make it harder.
pub fn initial_target() -> U256 { U256::MAX / U256::from(plicata_runtime::configs::POW_TARGET_DIVISOR::get()) }

#[derive(Clone)]
pub struct PlicataPow {
    difficulty: U256,
}

/// Adjust the PoW target based on how long the last difficulty window took.
///
/// A larger target makes cultivation easier; a smaller target makes it harder.
pub fn adjust_target(
    current_target: U256,
    actual_time_ms: u64,
    expected_time_ms: u64,
) -> U256 {
    if actual_time_ms == 0 || expected_time_ms == 0 {
        return current_target;
    }

    let max_multiplier = U256::from(
        plicata_runtime::configs::MAX_DIFFICULTY_MULTIPLIER::get(),
    );

    let actual = U256::from(actual_time_ms);
    let expected = U256::from(expected_time_ms);

    let mut new_target = if actual > expected {
        current_target.saturating_mul(actual) / expected
    } else {
        current_target.saturating_mul(actual) / expected
    };

    let min_target = current_target / max_multiplier;
    let max_target = current_target.saturating_mul(max_multiplier);

    if new_target < min_target {
        new_target = min_target;
    }

    if new_target > max_target {
        new_target = max_target;
    }

    new_target
}

impl PlicataPow {
    pub fn new(difficulty: U256) -> Self {
        Self { difficulty }
    }

    pub fn cultivate<B: BlockT>(
        &self,
        pre_hash: &B::Hash,
	difficulty: U256,
	should_stop: impl Fn() -> bool,
    ) -> Option<Vec<u8>> {
        let mut nonce: u64 = 0;

        loop {
            if should_stop() {
    return None;
}

            let mut input = pre_hash.as_ref().to_vec();
            input.extend_from_slice(&nonce.to_le_bytes());

            let hash = BlakeTwo256::hash(&input);
            let hash_value = U256::from_big_endian(hash.as_bytes());

            if hash_value <= difficulty {
                return Some(nonce.to_le_bytes().to_vec());
            }

            nonce = nonce.wrapping_add(1);
        }
    }
}

impl<B: BlockT> PowAlgorithm<B> for PlicataPow {
    type Difficulty = U256;

    fn difficulty(&self, _parent: B::Hash) -> Result<Self::Difficulty, Error<B>> {
        Ok(self.difficulty)
    }

fn verify(
    &self,
    _parent: &sp_runtime::generic::BlockId<B>,
    pre_hash: &B::Hash,
    _pre_digest: Option<&[u8]>,
    seal: &Vec<u8>,
    _difficulty: Self::Difficulty,
) -> Result<bool, Error<B>> {
    let mut input = pre_hash.as_ref().to_vec();
    input.extend_from_slice(seal);

    let hash = BlakeTwo256::hash(&input);
let hash_value = U256::from_big_endian(hash.as_bytes());



    Ok(hash_value <= _difficulty)
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use plicata_runtime::opaque::Block;

    #[test]
    fn difficulty_stays_same_at_target_time() {
        let target = initial_target();

        let adjusted = adjust_target(target, 60_000, 60_000);

        assert_eq!(adjusted, target);
    }

    #[test]
    fn difficulty_gets_harder_when_blocks_are_fast() {
        let target = initial_target();

        let adjusted = adjust_target(target, 30_000, 60_000);

        assert_eq!(adjusted, target / U256::from(2));
    }

    #[test]
    fn difficulty_gets_easier_when_blocks_are_slow() {
        let target = initial_target();

        let adjusted = adjust_target(target, 120_000, 60_000);

        assert_eq!(adjusted, target * U256::from(2));
    }

    #[test]
    fn cultivator_finds_nonce() {
        let target = U256::MAX / U256::from(plicata_runtime::configs::POW_TARGET_DIVISOR::get());
        let pow = PlicataPow::new(target);

        let pre_hash = <Block as BlockT>::Hash::from([1u8; 32]);

        let seal = pow.cultivate::<Block>(&pre_hash, target, || false);

        assert!(seal.is_some());

        println!("Found seal: {:?}", seal.unwrap());
    }
}
