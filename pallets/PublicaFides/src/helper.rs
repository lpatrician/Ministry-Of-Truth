use crate::pallet;
use sp_arithmetic::{per_things::Permill};

pub fn score_claims(claims: pallet::ResolvedClaims) -> Permill {
    let mut true_count = Permill::zero();
    let iter_true_claims = claims.claims.iter();
    // claims should be max 10
    for claim in iter_true_claims {
        if claim.is_accepted == true {
            // true_count = true_count + 1
            true_count = true_count + Permill::one();
        };
    };
    let total_claims = claims.claims.len() as u8;
    true_count / total_claims
}
