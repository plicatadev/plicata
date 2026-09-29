use sc_consensus_pow::{Error, PowAlgorithm};
use sp_core::U256;
use sp_runtime::traits::{Block as BlockT, Hash as HashT, BlakeTwo256};
use std::{thread, time::Duration};

/// Initial PoW target for the Plicata development network.
/// Higher targets make cultivation easier; lower targets make it harder.
pub fn initial_target() -> U256 { U256::MAX / 1_000_000 }

#[derive(Clone)]
pub struct PlicataPow {
    difficulty: U256,
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
    fn cultivator_finds_nonce() {
        let target = U256::MAX / 1_000_000;
        let pow = PlicataPow::new(target);

        let pre_hash = <Block as BlockT>::Hash::from([1u8; 32]);

        let seal = pow.cultivate::<Block>(&pre_hash, target, || false);

        assert!(seal.is_some());

        println!("Found seal: {:?}", seal.unwrap());
    }
}
