use oracle_layer::fri_query::fri_prove;
use oracle_layer::fri_protocol::Challenger;
use p3_baby_bear::{BabyBear, default_babybear_poseidon2_16};

type F = BabyBear;

#[test]
fn fri_prove_does_not_panic_and_returns_proof() {
    let n = 1024;
    let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
    let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

    let perm = default_babybear_poseidon2_16();
    let mut challenger = Challenger::new(perm);

    let proof = fri_prove(evals, domain, &mut challenger, 20);

    // In 0.6.1 FriProof has no Debug, so just check it exists
    // If we got here without panic, the API works
    std::hint::black_box(proof);
}
