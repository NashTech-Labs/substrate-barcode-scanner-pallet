use frame_support::assert_ok;
use sp_runtime::traits::Hash;

use crate::{mock::*, Product};

use super::*;

#[test]
fn check_product_storage() {
    new_test_ext().execute_with(|| {
        let barcode = create_hash_data(&1u32);
        let id = create_hash_data(&2u32);
        let owner: u64 = 6;
        let product = Product {
            id,
            name: "TEST".encode(),
            manufacturer: owner,
        };
        assert_ok!(TemplateModule::add_product(
            Origin::signed(owner),
            barcode.clone(),
            product.clone()
        ));
        assert_eq!(TemplateModule::product_information(barcode), product);
    });
}

#[test]
fn verify_barcode_scanner_for_fake_product() {
    new_test_ext().execute_with(|| {
        create_test_product();
        assert_eq!(
            TemplateModule::is_valid_barcode(create_hash_data(&2u32)),
            false
        );
    });
}

fn create_hash_data(data: &u32) -> <mock::Test as frame_system::Trait>::Hash {
    data.using_encoded(<Test as frame_system::Trait>::Hashing::hash)
}

fn create_test_product() -> Product<
    <mock::Test as frame_system::Trait>::AccountId,
    <mock::Test as frame_system::Trait>::Hash,
> {
    let id = create_hash_data(&2u32);
    let owner: u64 = 6;
    Product {
        id,
        name: "TEST".encode(),
        manufacturer: owner,
    }
}
