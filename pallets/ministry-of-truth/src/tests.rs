use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_stores_articles() {
	new_test_ext().execute_with(|| {
		assert_ok!(MinistryOfTruth::store_article(Origin::signed(1), vec![1, 2], vec![2, 3]));
		assert_eq!(MinistryOfTruth::next_class_id(), 1)
	});
}

#[test]
fn it_stores_claims() {
	new_test_ext().execute_with(|| {
		assert_ok!(MinistryOfTruth::store_article(Origin::signed(1), vec![1, 2], vec![2, 3]));

		assert_ok!(MinistryOfTruth::store_claim_for_article(
			Origin::signed(1),
			vec![1, 2],
			0,
			false
		));
		assert_eq!(MinistryOfTruth::next_claim_id(), 1);
		assert_eq!(
			MinistryOfTruth::get_claims(0, 0),
			Claim { claim_text_cid: [1, 2].to_vec(), is_rejected: false }
		);
	});
}

#[test]
fn it_fails_if_article_nonexistent() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			MinistryOfTruth::store_claim_for_article(Origin::signed(1), vec![1, 2], 0, false),
			Error::<Test>::NonExistentArticle
		);
	});
}
