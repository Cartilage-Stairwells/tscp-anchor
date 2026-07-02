use batch_merkle::{new_batch_merkle, F};
use p3_commit::Mmcs;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;

#[test]
fn commit_open_verify_mixed_matrices() {
    let mmcs = new_batch_merkle();

    let col0 = RowMajorMatrix::new(vec![F::new(0), F::new(1), F::new(0), F::new(1)], 1);
    let col1 = RowMajorMatrix::new(vec![F::new(0), F::new(1)], 1);
    let wide = RowMajorMatrix::new(
        vec![
            F::new(1),
            F::new(2),
            F::new(1),
            F::new(2),
            F::new(0),
            F::new(1),
            F::new(0),
            F::new(1),
        ],
        2,
    );

    let dims = vec![col0.dimensions(), col1.dimensions(), wide.dimensions()];
    let (commit, prover_data) = mmcs.commit(vec![col0, col1, wide]);

    let opening = mmcs.open_batch(1, &prover_data);
    mmcs.verify_batch(&commit, &dims, 1, (&opening).into())
        .expect("valid batch proof should verify");
}

#[test]
fn tampered_proof_fails_verification() {
    let mmcs = new_batch_merkle();

    let col0 = RowMajorMatrix::new(vec![F::new(0), F::new(1), F::new(2), F::new(1)], 1);
    let col1 = RowMajorMatrix::new(vec![F::new(1), F::new(0), F::new(1), F::new(2)], 1);

    let dims = vec![col0.dimensions(), col1.dimensions()];
    let (commit, prover_data) = mmcs.commit(vec![col0, col1]);

    let mut opening = mmcs.open_batch(2, &prover_data);
    opening.opening_proof[0][0] += F::new(1); // corrupt the proof

    let result = mmcs.verify_batch(&commit, &dims, 2, (&opening).into());
    assert!(result.is_err(), "tampered proof should fail verification");
}
