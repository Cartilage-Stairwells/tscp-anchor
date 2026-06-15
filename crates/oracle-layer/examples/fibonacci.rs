use oracle_layer::{oracle::{ColumnOracle, MleOracle}, folded::{FoldedOracleBuilder, NeedsChallenge}};
use p3_baby_bear::BabyBear;
use p3_matrix::dense::RowMajorMatrix;

type F = BabyBear;

fn main() {
    // Build a Fibonacci trace: [a, b, a+b, b+(a+b), ...]
    let n = 8usize;
    let mut row0 = vec![F::ZERO; n];
    let mut row1 = vec![F::ZERO; n];
    row0[0] = F::ONE;
    row1[0] = F::ONE;
    for i in 1..n {
        row0[i] = row0[i-1] + row1[i-1];
        row1[i] = row1[i-1] + row0[i];
    }

    println!("Fibonacci trace (col 0): {:?}", &row0);

    // Lift to column oracle
    let col0 = ColumnOracle { values: row0.clone(), n_vars: n.ilog2() as usize };
    let col1 = ColumnOracle { values: row1.clone(), n_vars: n.ilog2() as usize };

    // Fold with random alpha (in production: from Fiat-Shamir transcript)
    let alpha = F::from_canonical_u32(42);
    let builder = FoldedOracleBuilder::new(
        vec![col0.values.clone(), col1.values.clone()],
        n.ilog2() as usize,
    );
    // Typestate enforces: must absorb challenge before binding
    let folded = builder.absorb_challenge(alpha).build();

    // Eval at a point
    let point: Vec<F> = vec![
        F::from_canonical_u32(2),
        F::from_canonical_u32(3),
        F::from_canonical_u32(1),
    ];
    println!("FoldedOracle eval at point: {:?}", folded.eval(&point));

    // Commutation law test
    let prefix = vec![F::from_canonical_u32(1)];
    let ok = oracle_layer::folded::FoldedOracle::<F>::test_commutation_law(
        &[col0.values, col1.values],
        alpha,
        &prefix,
        n.ilog2() as usize,
    );
    println!("Commutation law holds: {}", ok);
}
