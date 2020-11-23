#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
    /// The API to verify barcode
    pub trait VerifyBarcodeApi<Hash> where
		Hash: codec::Codec, {
		/// Verify barcode
        fn is_valid_barcode(barcode: Hash) -> bool;
    }
}