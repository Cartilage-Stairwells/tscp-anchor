use oracle_layer::oracle::MleOracle;
use oracle_layer::folded::FoldedOracleBuilder;
use p3_baby_bear::BabyBear;
use p3_field::PrimeCharacteristicRing;

type F = BabyBear;

fn verify_sumcheck_round(claimed_sum: F, g: [F; 2], r: F) -> F {
    assert_eq!(g[0] + g[1], claimed_sum,
        "Round failed: g(0)+g(1)={} != claimed={}", g[0]+g[1], claimed_sum);
    g[0] + r * (g[1] - g[0])
}

fn hypercube_sum(oracle: &impl MleOracle<F>, n_vars: usize) -> F {
    (0..(1usize << n_vars)).map(|i| {
        let pt: Vec<F> = (0..n_vars)
            .map(|b| if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO })
            .collect();
        oracle.eval(&pt)
    }).fold(F::ZERO, |a, b| a + b)
}

fn sumcheck_prover_round(oracle: &impl MleOracle<F>, prefix: &[F]) -> [F; 2] {
    let n_vars = oracle.n_vars();
    let remaining = n_vars - prefix.len() - 1;
    let half = 1usize << remaining;

    let sum_with_bit = |bit: F| -> F {
        (0..half).map(|i| {
            let mut pt = prefix.to_vec();
            pt.push(bit);
            for b in 0..remaining {
                pt.push(if (i >> b) & 1 == 1 { F::ONE } else { F::ZERO });
            }
            oracle.eval(&pt)
        }).fold(F::ZERO, |a, b| a + b)
    };

    [sum_with_bit(F::ZERO), sum_with_bit(F::ONE)]
}

fn main() {
    println!("=== Fibonacci AIR Full Sumcheck Demo ===");

    let n = 8usize;
    let n_vars = n.ilog2() as usize;

    let mut col0 = vec![F::ZERO; n];
    let mut col1 = vec![F::ZERO; n];
    col0[0] = F::ONE; col1[0] = F::ONE;
    for i in 1..n {
        col0[i] = col0[i-1] + col1[i-1];
        col1[i] = col1[i-1] + col0[i];
    }
    println!("col0: {:?}", &col0);

    let alpha = BabyBear::new(7);
    let folded = FoldedOracleBuilder::new(
        vec![col0.clone(), col1.clone()], n_vars,
    ).absorb_challenge(alpha).build();

    // Initial claim = actual sum over hypercube
    let mut claim = hypercube_sum(&folded, n_vars);
    println!("Initial claim (hypercube sum): {}", claim);

    let mut prefix: Vec<F> = Vec::new();
    for round in 0..n_vars {
        let g = sumcheck_prover_round(&folded, &prefix);
        let r = BabyBear::new(3 + round as u32);
        claim = verify_sumcheck_round(claim, g, r);
        prefix.push(r);
        println!("Round {}: g=[{},{}] r={} new_claim={}", round, g[0], g[1], r, claim);
    }

    let terminal = folded.eval(&prefix);
    println!("\nTerminal oracle eval: {}", terminal);
    println!("Final claim:          {}", claim);
    assert_eq!(terminal, claim, "Terminal check FAILED");
    println!("✅ Terminal check passed.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_verifier_accepts_valid_transcript() {
        let g = [BabyBear::new(10), BabyBear::new(6)];
        let r = BabyBear::new(2);
        let new_claim = verify_sumcheck_round(BabyBear::new(16), g, r);
        assert_eq!(new_claim, BabyBear::new(2));
    }
}
