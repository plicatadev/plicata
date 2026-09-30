#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::tokens::fungible::Mutate,
    };
    use sp_core::Decode;
    use sp_runtime::generic::DigestItem;
    use frame_system::pallet_prelude::BlockNumberFor;

    const POW_ENGINE_ID: [u8; 4] = *b"pow_";

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// PLIC balance implementation.
        type Currency: Mutate<Self::AccountId, Balance = u128>;

        /// Number of base units awarded for each mined Growth Ring.
        #[pallet::constant]
        type HarvestReward: Get<u128>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A Cultivator received the Harvest for a Growth Ring.
        HarvestPaid {
            cultivator: T::AccountId,
            amount: u128,
        },
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
            if let Some(cultivator) = Self::cultivator() {
                let reward = T::HarvestReward::get();

                if T::Currency::mint_into(&cultivator, reward).is_ok() {
                    Self::deposit_event(Event::HarvestPaid {
                        cultivator,
                        amount: reward,
                    });
                }
            }

            Weight::zero()
        }
    }

    impl<T: Config> Pallet<T> {
        /// Read the Cultivator Plot committed to the current Growth Ring.
        pub fn cultivator() -> Option<T::AccountId> {
            let digest = frame_system::Pallet::<T>::digest();

            for log in digest.logs() {
                if let DigestItem::PreRuntime(engine_id, data) = log {
                    if engine_id == &POW_ENGINE_ID {
                        let mut bytes = &data[..];

                        if let Ok(account) = T::AccountId::decode(&mut bytes) {
                            return Some(account);
                        }
                    }
                }
            }

            None
        }
    }
}
