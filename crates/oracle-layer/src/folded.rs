use crate::oracle::{evaluate_mle, MleOracle};
use p3_field::Field;

/// Typestate: challenge not yet absorbed
pub struct NeedsChallenge;
/// Typestate: challenge absorbed, ready to bind prefix
pub struct ReadyToBind<F: Field> {
    pub alpha: F,
}

pub struct FoldedOracleBuilder<F: Field, State> {
    pub oracles: Vec<Vec<F>>, // raw column values
    pub n_vars: usize,
    pub state: State,
    _f: std::marker::PhantomData<F>,
}

impl<F: Field> FoldedOracleBuilder<F, NeedsChallenge> {
    pub fn new(oracles: Vec<Vec<F>>, n_vars: usize) -> Self {
        Self {
            oracles,
            n_vars,
            state: NeedsChallenge,
            _f: Default::default(),
        }
    }

    /// Must absorb challenge before binding — enforced by typestate
    pub fn absorb_challenge(self, alpha: F) -> FoldedOracleBuilder<F, ReadyToBind<F>> {
        FoldedOracleBuilder {
            oracles: self.oracles,
            n_vars: self.n_vars,
            state: ReadyToBind { alpha },
            _f: Default::default(),
        }
    }
}

impl<F: Field> FoldedOracleBuilder<F, ReadyToBind<F>> {
    pub fn build(self) -> FoldedOracle<F> {
        let alpha = self.state.alpha;
        let n = self.oracles[0].len();
        let mut folded = vec![F::ZERO; n];
        let mut coeff = F::ONE;
        for oracle in &self.oracles {
            for (i, v) in oracle.iter().enumerate() {
                folded[i] += coeff * *v;
            }
            coeff *= alpha;
        }
        FoldedOracle {
            values: folded,
            n_vars: self.n_vars,
            alpha,
        }
    }
}

pub struct FoldedOracle<F: Field> {
    pub values: Vec<F>,
    pub n_vars: usize,
    pub alpha: F,
}

impl<F: Field> FoldedOracle<F> {
    /// bind(fold(α; Rs), ρ) — prefix binding after folding
    pub fn bind_prefix(&self, prefix: &[F]) -> Vec<F> {
        let remaining = self.n_vars - prefix.len();
        let size = 1usize << remaining;
        (0..size)
            .map(|idx| {
                let mut full = prefix.to_vec();
                let bits: Vec<F> = (0..remaining)
                    .map(|i| if (idx >> i) & 1 == 1 { F::ONE } else { F::ZERO })
                    .collect();
                full.extend(bits);
                evaluate_mle(&self.values, &full)
            })
            .collect()
    }

    /// Commutation law test: bind(fold(α;Rs), ρ) = fold(α; bind(Rᵢ, ρ))
    #[cfg(test)]
    pub fn test_commutation_law(oracles: &[Vec<F>], alpha: F, prefix: &[F], n_vars: usize) -> bool
    where
        F: PartialEq,
    {
        // LHS: fold then bind
        let builder = FoldedOracleBuilder::new(oracles.to_vec(), n_vars);
        let lhs = builder.absorb_challenge(alpha).build().bind_prefix(prefix);

        // RHS: bind each oracle then fold
        let bound: Vec<Vec<F>> = oracles
            .iter()
            .map(|o| {
                let remaining = n_vars - prefix.len();
                let size = 1usize << remaining;
                (0..size)
                    .map(|idx| {
                        let mut full = prefix.to_vec();
                        let bits: Vec<F> = (0..remaining)
                            .map(|i| if (idx >> i) & 1 == 1 { F::ONE } else { F::ZERO })
                            .collect();
                        full.extend(bits);
                        evaluate_mle(o, &full)
                    })
                    .collect()
            })
            .collect();

        let mut rhs = vec![F::ZERO; lhs.len()];
        let mut coeff = F::ONE;
        for b in &bound {
            for (i, v) in b.iter().enumerate() {
                rhs[i] += coeff * *v;
            }
            coeff *= alpha;
        }
        lhs == rhs
    }
}

impl<F: Field> MleOracle<F> for FoldedOracle<F> {
    fn n_vars(&self) -> usize {
        self.n_vars
    }
    fn eval(&self, point: &[F]) -> F {
        evaluate_mle(&self.values, point)
    }
}
