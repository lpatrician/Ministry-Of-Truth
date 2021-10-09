pub fn content_scoring(claim_bools: Vec<bool>) -> f64 {
    let mut true_count = 0;
    let iter_true_claims = claim_bools.iter();
    for claim in iter_true_claims {
        if *claim == true {
            true_count = true_count + 1
        };
    };
    let total_claims = claim_bools.len();
    let score = (true_count as f64 / total_claims as f64) * 100f64;
    return score as f64;
}