use oracle_layer::oracle::{ColumnOracle, MleOracle};
use oracle_layer::folded::FoldedOracleBuilder;
use p3_baby_bear::BabyBear;
use p3_field::PrimeCharacteristicRing;

type F = BabyBear;

fn main() {
    let n = 8usize;
    let mut col0 = vec![F::ZERO; n];
    let mut col1 = vec![F::ZERO; n];
    col0[0] = F::ONE;
    col1[0] = F::ONE;
    for i in 1..n {
        col0[i] = col0[i-1] + col1[i-1];
        col1[i] = col1[i-1] + col0[i];
    }
    println!("Fibonacci col0: {:?}", &col0);

    let alpha = BabyBear::new(42);
    let folded = FoldedOracleBuilder::new(
        vec![col0.clone(), col1.clone()],
        n.ilog2() as usize,
    ).absorb_challenge(alpha).build();

    let point = vec![BabyBear::new(2), BabyBear::new(3), BabyBear::new(1)];
    println!("FoldedOracle eval: {:?}", folded.eval(&point));

    // Commutation law: fold-then-bind == bind-then-fold
    let prefix = vec![BabyBear::new(1)];
    let lhs = folded.bind_prefix(&prefix);

    let mut rhs = vec![F::ZERO; lhs.len()];
    let mut coeff = F::ONE;
    for oracle_vals in [col0.clone(), col1.clone()] {
        let bound = FoldedOracleBuilder::new(vec![oracle_vals], n.ilog2() as usize)
            .absorb_challenge(F::ONE)
            .build()
            .bind_prefix(&prefix);
        for (i, v) in bound.iter().enumerate() {
            rhs[i] += coeff * *v;
        }
        coeff *= alpha;
    }
    println!("Commutation law holds: {}", lhs == rhs);
}
