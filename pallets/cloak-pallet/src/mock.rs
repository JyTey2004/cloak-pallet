#[warn(non_camel_case_types)]
use crate as pallet_cloak;
use frame_support::{
    derive_impl,
    traits::{ConstU128, ConstU16, ConstU32, ConstU64},
};
use sp_core::H256;
use sp_io::TestExternalities;
use sp_runtime::{
    // generic,
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

// type SignedExtra = ();
// type Signature = ();
type Block = frame_system::mocking::MockBlock<Test>;
// type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
type Balance = u128;
type AccountId = u64;
// type UncheckedExtrinsic =
//     generic::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;

// Configure a mock runtime to test the pallet. We use the simpler syntax here.
frame_support::construct_runtime! {
    pub struct Test {
        System: frame_system,
        Balances: pallet_balances,
        Pallet_Cloak: pallet_cloak,
    }
}

// Feel free to remove more items from this, as they are the same as
// `frame_system::config_preludes::TestDefaultConfig`. We have only listed the full `type` list here
// for verbosity. Same for `pallet_balances::Config`.
// https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.derive_impl.html
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU128<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<10>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = RuntimeHoldReason;
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<10>;
}

impl pallet_cloak::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type NativeBalance = Balances;
    type RuntimeCall = RuntimeCall;
    type RuntimeHoldReason = RuntimeHoldReason;
}

pub struct StateBuilder {
    pub initial_accounts: Vec<AccountId>,
    pub initial_balances: Vec<(AccountId, Balance)>,
}

impl Default for StateBuilder {
    fn default() -> Self {
        Self {
            initial_accounts: vec![1, 2],
            initial_balances: vec![(1, 10_000_000), (2, 5_000_000), (3, 100)],
        }
    }
}

impl StateBuilder {
    pub(crate) fn with_balance(mut self, acc: AccountId, balance: Balance) -> Self {
        self.initial_balances.push((acc, balance));
        self
    }

    pub(crate) fn build_and_execute(self, test: impl FnOnce() -> ()) {
        let system = frame_system::GenesisConfig::<Test>::default();
        let balances = pallet_balances::GenesisConfig::<Test> {
            balances: self.initial_balances,
        };
        let pallet_cloak = pallet_cloak::GenesisConfig::<Test> {
            initial_accounts: self.initial_accounts,
        };

        let mut ext: TestExternalities = RuntimeGenesisConfig {
            system,
            balances,
            pallet_cloak,
        }
        .build_storage()
        .unwrap()
        .into();

        ext.execute_with(|| {
            test();
            Pallet_Cloak::do_try_state();
        });
    }
}

// pub fn new_test_ext() -> sp_io::TestExternalities {
//     let mut t = frame_system::GenesisConfig::<Test>::default()
//         .build_storage()
//         .unwrap();

//     pallet_balances::GenesisConfig::<Test> {
//         balances: vec![(1, 100000), (2, 50000)],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(1));
//     ext
// }

// pub fn new_test_ext_with_little_balance() -> sp_io::TestExternalities {
//     let mut t: sp_runtime::Storage = frame_system::GenesisConfig::<Test>::default()
//         .build_storage()
//         .unwrap();

//     pallet_balances::GenesisConfig::<Test> {
//         balances: vec![(1, 1000), (2, 500)],
//     }
//     .assimilate_storage(&mut t)
//     .unwrap();

//     let mut ext = sp_io::TestExternalities::new(t);
//     ext.execute_with(|| System::set_block_number(1));
//     ext
// }

// pub fn mock_extrinsic(account_id: AccountId, call: RuntimeCall) -> UncheckedExtrinsic {
//     let extra: SignedExtra = (
// 		// TODO: Add whatever signed extensions you want, such as:
// 		// pallet_transaction_payment::ChargeTransactionPayment::<Test>::from(0),
// 	);
//     let raw_payload = SignedPayload::new(call, extra).unwrap();
//     let (call, extra, _) = raw_payload.deconstruct();
//     UncheckedExtrinsic::new_signed(call, account_id.into(), (), extra)
// }
