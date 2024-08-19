#![cfg_attr(not(feature = "std"), no_std)]

pub use cloak_pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html
// https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.pallet.html#dev-mode-palletdev_mode
#[frame_support::pallet(dev_mode)]
pub mod cloak_pallet {
    use crate::*;
    use frame_support::pallet_prelude::{Encode, OptionQuery};
    use frame_support::traits::fungible::Inspect;
    use frame_support::traits::fungible::*;
    use frame_support::traits::tokens::{Precision, Preservation};
    use frame_support::StorageHasher;
    use frame_support::{
        dispatch::GetDispatchInfo, pallet_prelude::*, sp_runtime::traits::Zero, traits::fungible,
    };
    use frame_support::{ensure, Blake2_128Concat};
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_runtime::traits::Dispatchable;
    use sp_runtime::traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
    use sp_runtime::{PerThing, Percent, Rounding};
    use sp_std::prelude::*;

    /// The balance pallet's balance type.
    pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::composite_enum]
    pub enum HoldReason {
        #[codec(index = 0)]
        DataPoolStake,
    }

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        /// /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Type to access the Balances Pallet.
        type NativeBalance: fungible::Inspect<Self::AccountId>
            + fungible::Mutate<Self::AccountId>
            + fungible::hold::Inspect<Self::AccountId>
            + fungible::hold::Mutate<Self::AccountId, Reason = Self::RuntimeHoldReason>
            + fungible::freeze::Inspect<Self::AccountId>
            + fungible::freeze::Mutate<Self::AccountId>;

        /// A type representing all calls available in your runtime.
        /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html
        type RuntimeCall: Parameter
            + Dispatchable<
                RuntimeOrigin = Self::RuntimeOrigin,
                PostInfo = frame_support::dispatch::PostDispatchInfo,
            > + GetDispatchInfo;

        /// The reason type for holding tokens.
        type RuntimeHoldReason: From<HoldReason>;
    }

    #[pallet::genesis_config]
    #[derive(frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub initial_accounts: Vec<T::AccountId>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {}
    }

    /// Pallets use events to inform users when important changes are made.
    /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// We usually use passive tencse for events.
        TxSuccess,

        /// Transaction failed.
        TxFailed,
    }

    /// Errors inform users that something went wrong.
    /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#event-and-error
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        TxFailed,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html#dispatchables
    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    // #[pallet::hooks]
    // /// Implement runtime hooks.
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    //     #[cfg(feature = "try-runtime")]
    //     fn try_state(_: BlockNumberFor<T>) -> Result<(), sp_runtime::TryRuntimeError> {
    //         Self::do_try_state()
    //     }
    // }

    #[cfg(any(test, feature = "try-runtime"))]
    /// Implement runtime hooks.
    impl<T: Config> Pallet<T> {
        /// Try to access the state of the pallet.
        pub(crate) fn do_try_state() {
            use sp_std::collections::btree_set::BTreeSet;

            let (credits, holds) = AccountInfo::<T>::iter().fold(
                (
                    BTreeSet::<T::AccountId>::new(),
                    BTreeSet::<T::AccountId>::new(),
                ),
                |(mut credits, mut holds), (key, _)| {
                    credits.insert(key.clone());
                    holds.insert(key);
                    (credits, holds)
                },
            );

            assert_eq!(credits, holds);
        }
    }

    /// Implement the helper functions for the pallet.
    impl<T: Config> Pallet<T> {
        /// Get the weight of a call.
        pub fn call_weight(call: <T as Config>::RuntimeCall) -> Weight {
            call.get_dispatch_info().weight
        }
    }
}
