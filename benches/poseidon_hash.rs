use poseidon_rust::poseidon::{Fr, Poseidon};
use criterion::{criterion_group, criterion_main, Criterion};
use ff::PrimeField;

pub fn criterion_benchmark(c: &mut Criterion) {
    let fr: Fr =
        Fr::from_str("7048349338611591849303202358367150636722185719999099439061630806").unwrap();

    let mut arr: Vec<Fr> = Vec::new();
    arr.push(fr.clone());
    let poseidon = Poseidon::default();

    c.bench_function("poseidon hash", |b| {
        b.iter(|| poseidon.hash(arr.as_slice()).unwrap())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
