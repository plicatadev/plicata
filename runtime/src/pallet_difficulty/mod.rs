#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_core::U256;
    use frame_system::pallet_prelude::BlockNumberFor;
    use sp_runtime::{SaturatedConversion, Saturating};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type InitialTarget: Get<U256>;
    }

    #[pallet::storage]
    #[pallet::getter(fn current_target)]
    pub type CurrentTarget<T: Config> =
        StorageValue<_, U256, ValueQuery>;

    #[pallet::storage]
    pub type LastAdjustmentTimestamp<T: Config> =
        StorageValue<_, T::Moment, OptionQuery>;

    #[pallet::storage]
    pub type LastAdjustmentBlock<T: Config> =
        StorageValue<_, BlockNumberFor<T>, OptionQuery>;

    impl<T: Config> Pallet<T> {
        pub fn target() -> U256 {
            CurrentTarget::<T>::get()
        }
        pub fn current_timestamp() -> T::Moment {
            pallet_timestamp::Pallet::<T>::get()
        }


        pub fn set_target(target: U256) {
            CurrentTarget::<T>::put(target);
        }
    }

    
/// Calculate the next PoW target from the observed block time.
pub fn adjust_target(
    current_target: U256,
    actual_time_ms: u64,
    expected_time_ms: u64,
    max_multiplier: u32,
) -> U256 {
    if actual_time_ms == 0 || expected_time_ms == 0 || max_multiplier == 0 {
        return current_target;
    }

    let multiplier = U256::from(max_multiplier);
    let actual = U256::from(actual_time_ms);
    let expected = U256::from(expected_time_ms);

    let mut new_target = current_target.saturating_mul(actual) / expected;

    let min_target = current_target / multiplier;
    let max_target = current_target.saturating_mul(multiplier);

    if new_target < min_target {
        new_target = min_target;
    }

    if new_target > max_target {
        new_target = max_target;
    }

    new_target
}


    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            let current_block = frame_system::Pallet::<T>::block_number();
            let current_timestamp = Self::current_timestamp();

            let Some(last_block) = LastAdjustmentBlock::<T>::get() else {
                LastAdjustmentBlock::<T>::put(current_block);
                LastAdjustmentTimestamp::<T>::put(current_timestamp);
                return;
            };

            let Some(last_timestamp) = LastAdjustmentTimestamp::<T>::get() else {
                LastAdjustmentBlock::<T>::put(current_block);
                LastAdjustmentTimestamp::<T>::put(current_timestamp);
                return;
            };

            let blocks_elapsed: u64 =
                current_block.saturated_into::<u64>()
                    .saturating_sub(last_block.saturated_into::<u64>());

            if blocks_elapsed
                < crate::configs::DIFFICULTY_ADJUSTMENT_INTERVAL::get() as u64
            {
                return;
            }

            let actual_time_ms: u64 =
                current_timestamp.saturated_into();

            let last_time_ms: u64 =
                last_timestamp.saturated_into();

            let elapsed_time_ms =
                actual_time_ms.saturating_sub(last_time_ms);

            let expected_time_ms =
                crate::configs::DIFFICULTY_ADJUSTMENT_INTERVAL::get() as u64
                    * crate::configs::TARGET_BLOCK_TIME::get();

            let current_target = CurrentTarget::<T>::get();

            let new_target = super::adjust_target(
                current_target,
                elapsed_time_ms,
                expected_time_ms,
                crate::configs::MAX_DIFFICULTY_MULTIPLIER::get(),
            );

            CurrentTarget::<T>::put(new_target);
            LastAdjustmentBlock::<T>::put(current_block);
            LastAdjustmentTimestamp::<T>::put(current_timestamp);
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub initial_target: U256,
        pub _marker: PhantomData<T>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                initial_target: T::InitialTarget::get(),
                _marker: PhantomData,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            CurrentTarget::<T>::put(self.initial_target);
        }
    }
}

#[cfg(test)]
mod tests {
    use sp_core::U256;
    use super::*;

    #[test]
    fn target_stays_same_when_on_target() {
        let target = U256::from(1_000_000u64);

        let adjusted = adjust_target(
            target,
            600_000,
            600_000,
            2,
        );

        assert_eq!(adjusted, target);
    }

    #[test]
    fn target_gets_harder_when_blocks_are_fast() {
        let target = U256::from(1_000_000u64);

        let adjusted = adjust_target(
            target,
            300_000,
            600_000,
            2,
        );

        assert_eq!(adjusted, U256::from(500_000u64));
    }

    #[test]
    fn target_gets_easier_when_blocks_are_slow() {
        let target = U256::from(1_000_000u64);

        let adjusted = adjust_target(
            target,
            1_200_000,
            600_000,
            2,
        );

        assert_eq!(adjusted, U256::from(2_000_000u64));
    }
}
