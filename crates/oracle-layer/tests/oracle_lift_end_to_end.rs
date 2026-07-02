use oracle_layer::folded::FoldedOracleBuilder;
use oracle_layer::oracle::MleOracle;
use p3_baby_bear::BabyBear;
use p3_field::PrimeCharacteristicRing;

type F = BabyBear;

#[test]
fn oracle_lift_end_to_end() {
    let col0 = vec![F::ZERO, F::ONE, F::ZERO, F::ONE];
    let col1 = vec![F::ZERO, F::ONE, F::ONE, F::ZERO];

    let n_vars = 2;
    let alpha = F::from_u32(5);
    let folded = FoldedOracleBuilder::new(vec![col0, col1], n_vars)
        .absorb_challenge(alpha)
        .build();

    let claim = (0..4)
        .map(|i| {
            let pt: Vec<F> = (0..2)
                .map(|b| if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO })
                .collect();
            folded.eval(&pt)
        })
        .fold(F::ZERO, |a, b| a + b);

    assert!(claim != F::ZERO);
}
