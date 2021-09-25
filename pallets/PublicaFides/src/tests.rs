use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_stores_contents() {
	new_test_ext().execute_with(|| {
		assert_ok!(PublicaFides::store_content(Origin::signed(1), vec![1, 2], vec![2, 3]));
		assert_eq!(PublicaFides::next_class_id(), 1)
	});
}

#[test]
fn it_stores_claims() {
	new_test_ext().execute_with(|| {
		assert_ok!(PublicaFides::store_content(Origin::signed(1), vec![1, 2], vec![2, 3]));

		assert_ok!(PublicaFides::store_claim_for_content(
			Origin::signed(1),
			vec![1, 2],
			0,
			false
		));
		assert_eq!(PublicaFides::next_claim_id(), 1);
		assert_eq!(
			PublicaFides::get_claims(0, 0),
			Claim { claim_text_cid: [1, 2].to_vec(), is_accepted: true }
		);
	});
}

#[test]
fn it_fails_if_content_nonexistent() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			PublicaFides::store_claim_for_content(Origin::signed(1), vec![1, 2], 0, false),
			Error::<Test>::NonExistentContent
		);
	});
}
