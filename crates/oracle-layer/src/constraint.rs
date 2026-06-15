use p3_field::Field;

pub trait ConstraintOracle<F: Field> {
    fn n_vars(&self) -> usize;
    fn eval(&self, point: &[F]) -> F;
}

pub struct LinearCombination<F: Field> {
    pub oracles: Vec<Box<dyn ConstraintOracle<F>>>,
    pub coeffs: Vec<F>,
}

impl<F: Field> ConstraintOracle<F> for LinearCombination<F> {
    fn n_vars(&self) -> usize { self.oracles[0].n_vars() }
    fn eval(&self, point: &[F]) -> F {
        self.oracles.iter().zip(&self.coeffs)
            .map(|(o, &c)| c * o.eval(point))
            .fold(F::ZERO, |a, b| a + b)
    }
}
