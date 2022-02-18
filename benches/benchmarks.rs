use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};
use rand::{Rng, SeedableRng};
use rust_base32::{Engine, NaiveEngine};

fn do_naive_encode_bench(b: &mut Bencher, &size: &usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    fill(&mut v);
    let mut output: Vec<u8> = Vec::with_capacity(size * 2);
    fill(&mut output);
    b.iter(|| {
        let e = NaiveEngine::encode(&v, &mut output);
        black_box(&e);
    });
}

const BYTE_SIZES: [usize; 5] = [3, 50, 100, 500, 3 * 1024];

// Benchmarks over these byte sizes take longer so we will run fewer samples to
// keep the benchmark runtime reasonable.
const LARGE_BYTE_SIZES: [usize; 3] = [3 * 1024 * 1024, 10 * 1024 * 1024, 30 * 1024 * 1024];

fn encode_benchmarks(c: &mut Criterion, label: &str, byte_sizes: &[usize]) {
    let mut group = c.benchmark_group(label);
    group
        .warm_up_time(std::time::Duration::from_millis(500))
        .measurement_time(std::time::Duration::from_secs(3));

    for size in byte_sizes {
        group
            .throughput(Throughput::Bytes(*size as u64))
            .bench_with_input(
                BenchmarkId::new("encode_naive", size),
                size,
                do_naive_encode_bench,
            );
    }

    group.finish();
}

fn fill(v: &mut Vec<u8>) {
    let cap = v.capacity();
    // weak randomness is plenty; we just want to not be completely friendly to the branch predictor
    let mut r = rand::rngs::SmallRng::from_entropy();
    while v.len() < cap {
        v.push(r.gen::<u8>());
    }
}

fn bench(c: &mut Criterion) {
    encode_benchmarks(c, "encode_small_input", &BYTE_SIZES[..]);
    encode_benchmarks(c, "encode_large_input", &LARGE_BYTE_SIZES[..]);
}

criterion_group!(benches, bench);
criterion_main!(benches);
