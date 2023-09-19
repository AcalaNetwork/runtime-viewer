#![feature(prelude_import)]
#![no_std]
#![cfg(not(feature = "std"))]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
extern crate compiler_builtins as _;
use polkadot_runtime::{Runtime, Block};
use sp_api::{decl_runtime_apis, impl_runtime_apis};
use sp_runtime::{traits::StaticLookup, DispatchError};
type AccountId = <<Runtime as frame_system::Config>::Lookup as sp_runtime::traits::StaticLookup>::Source;
type Balance = <Runtime as pallet_balances::Config>::Balance;
#[doc(hidden)]
#[allow(dead_code)]
#[allow(deprecated)]
pub mod runtime_decl_for_polkadot_api {
    pub use super::*;
    pub trait PolkadotApiV1<Block: sp_api::BlockT> {
        fn get_balance(account: AccountId) -> Result<Balance, DispatchError>;
    }
    pub use PolkadotApiV1 as PolkadotApi;
    #[inline(always)]
    pub fn runtime_metadata<Block: sp_api::BlockT>() -> sp_api::metadata_ir::RuntimeApiMetadataIR
    where
        AccountId: sp_api::scale_info::TypeInfo + 'static,
        Result<Balance, DispatchError>: sp_api::scale_info::TypeInfo + 'static,
    {
        sp_api::metadata_ir::RuntimeApiMetadataIR {
            name: "PolkadotApi",
            methods: <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([
                    sp_api::metadata_ir::RuntimeApiMethodMetadataIR {
                        name: "get_balance",
                        inputs: <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                sp_api::metadata_ir::RuntimeApiMethodParamMetadataIR {
                                    name: "account",
                                    ty: sp_api::scale_info::meta_type::<AccountId>(),
                                },
                            ]),
                        ),
                        output: sp_api::scale_info::meta_type::<
                            Result<Balance, DispatchError>,
                        >(),
                        docs: ::alloc::vec::Vec::new(),
                    },
                ]),
            ),
            docs: ::alloc::vec::Vec::new(),
        }
    }
    pub const VERSION: u32 = 1u32;
    pub const ID: [u8; 8] = [54u8, 157u8, 202u8, 65u8, 101u8, 115u8, 172u8, 35u8];
}
pub struct RuntimeApi {}
impl runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::PolkadotApi<Block>
for Runtime {
    fn get_balance(account: AccountId) -> Result<Balance, DispatchError> {
        let acc = <Runtime as frame_system::Config>::Lookup::lookup(account)?;
        Ok(pallet_balances::Pallet::<Runtime>::free_balance(&acc))
    }
}
const RUNTIME_API_VERSIONS: sp_api::ApisVec = ::sp_version::sp_std::borrow::Cow::Borrowed(
    &[
        (
            runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::ID,
            runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::VERSION,
        ),
    ],
);
const _: () = {
    #[link_section = "runtime_apis"]
    static SECTION_CONTENTS: [u8; 12] = sp_api::serialize_runtime_api_info(
        runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::ID,
        runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::VERSION,
    );
};
#[doc(hidden)]
trait InternalImplRuntimeApis {
    #[inline(always)]
    fn runtime_metadata(
        &self,
    ) -> sp_api::vec::Vec<sp_api::metadata_ir::RuntimeApiMetadataIR> {
        <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::runtime_metadata::<
                    Block,
                >(),
            ]),
        )
    }
}
#[doc(hidden)]
impl InternalImplRuntimeApis for Runtime {}
pub mod api {
    use super::*;
    #[no_mangle]
    pub unsafe fn PolkadotApi_get_balance(input_data: *mut u8, input_len: usize) -> u64 {
        let mut input = if input_len == 0 {
            &[0u8; 0]
        } else {
            unsafe { sp_api::slice::from_raw_parts(input_data, input_len) }
        };
        sp_api::init_runtime_logger();
        let output = (move || {
            let account: AccountId = match sp_api::DecodeLimit::decode_all_with_depth_limit(
                sp_api::MAX_EXTRINSIC_DEPTH,
                &mut input,
            ) {
                Ok(res) => res,
                Err(e) => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Bad input data provided to {0}: {1}", "get_balance", e
                        ),
                    );
                }
            };
            #[allow(deprecated)]
            <Runtime as runtime_decl_for_polkadot_api::runtime_decl_for_polkadot_api::PolkadotApi<
                Block,
            >>::get_balance(account)
        })();
        sp_api::to_substrate_wasm_fn_return_value(&output)
    }
}
