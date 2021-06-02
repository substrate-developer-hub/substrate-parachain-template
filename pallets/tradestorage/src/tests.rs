use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::store_trade_map(Origin::signed(1), "42", 100, 200, 5, 12));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::trade_map(), Some("42"));
	});
}
