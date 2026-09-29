#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_core::U256;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type InitialTarget: Get<U256>;
    }

    #[pallet::storage]
    #[pallet::getter(fn current_target)]
    pub type CurrentTarget<T: Config> =
        StorageValue<_, U256, ValueQuery>;

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
