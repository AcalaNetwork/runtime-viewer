#![no_std]
#![cfg(not(feature = "std"))]

use polkadot_runtime::{Runtime, Block};
use sp_api::{decl_runtime_apis, impl_runtime_apis};
use sp_runtime::{traits::StaticLookup, DispatchError};

type AccountId = <<Runtime as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source;
type Balance = <Runtime as pallet_balances::Config>::Balance;

decl_runtime_apis! {
    pub trait PolkadotApi {
        fn get_balance(account: AccountId) -> Result<Balance, DispatchError>;
    }
}

impl_runtime_apis! {
    impl runtime_decl_for_polkadot_api::PolkadotApi<Block> for Runtime {
        fn get_balance(account: AccountId) -> Result<Balance, DispatchError> {
            let acc = <Runtime as frame_system::Config>::Lookup::lookup(account)?;
            Ok(pallet_balances::Pallet::<Runtime>::free_balance(&acc))
        }
    }
}