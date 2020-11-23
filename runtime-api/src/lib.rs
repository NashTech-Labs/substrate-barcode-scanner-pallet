#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

sp_api::decl_runtime_apis! {
    pub trait VerifyBarcodeApi<Hash> where
		Hash: codec::Codec, {
        fn is_valid_barcode(barcode: Hash) -> bool;
    }
}