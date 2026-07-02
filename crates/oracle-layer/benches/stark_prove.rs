use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use oracle_layer::fri_protocol::Challenger;
use oracle_layer::fri_query::fri_prove;
use p3_baby_bear::{default_babybear_poseidon2_16, BabyBear};

type F = BabyBear;

fn bench_fri_prove(c: &mut Criterion) {
    let mut group = c.benchmark_group("fri_prove_scaling");

    for &n in &[1024usize, 4096, 16384] {
        let evals: Vec<F> = (1..=n).map(|i| F::new(i as u32)).collect();
        let domain: Vec<F> = (1..=n).map(|i| F::new((i * 5 + 1) as u32)).collect();

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, _| {
            b.iter(|| {
                let perm = default_babybear_poseidon2_16();
                let mut challenger = Challenger::new(perm);
                let _ = fri_prove(evals.clone(), domain.clone(), &mut challenger, 20);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fri_prove);
criterion_main!(benches);
