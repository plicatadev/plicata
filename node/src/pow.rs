use sc_consensus_pow::{Error, PowAlgorithm};
use sp_core::U256;
use sp_runtime::traits::{Block as BlockT, Hash as HashT, BlakeTwo256};

pub struct PlicataPow {
    difficulty: U256,
}

impl PlicataPow {
    pub fn new(difficulty: U256) -> Self {
        Self { difficulty }
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

println!("PoW hash as U256: {}", hash_value);

    println!("PoW test hash: {:?}", hash);

    Ok(hash_value <= _difficulty)
}
}
