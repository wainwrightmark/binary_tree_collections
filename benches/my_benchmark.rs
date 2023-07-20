use binaryset::*;
use cc_traits::Len;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn setup_interleave(n: usize) -> (BinarySet<usize>, Vec<usize>) {
    let set = BinarySet::from_iter((0..n).map(|x| x * 2));
    let vec = (0..n).map(|x| (x * 2) + 1).collect();
    (set, vec)
}

fn setup_prepend(n: usize) -> (BinarySet<usize>, Vec<usize>) {
    let set = BinarySet::from_iter(n..(n + n));
    let vec = (0..n).collect();
    (set, vec)
}

fn setup_postpend(n: usize) -> (BinarySet<usize>, Vec<usize>) {
    let set = BinarySet::from_iter(0..n);
    let vec = (n..(n + n)).collect();
    (set, vec)
}

fn extend(set: &mut BinarySet<usize>, vec: &Vec<usize>) -> usize {
    set.extend(vec.iter().cloned());
    set.len()
}

fn bench_extend(c: &mut Criterion, name: &str, get_args: impl Fn(usize)->(BinarySet<usize>, Vec<usize>)){
    let mut group = c.benchmark_group(name);

    for n in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        let args = get_args(n);
        group.bench_function(BenchmarkId::from_parameter(n), |b| {
            b.iter_batched_ref(
                || args.0.clone(),
                |a| extend(a, &args.1),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_extend(c, "interleave", setup_interleave);
    bench_extend(c, "prepend", setup_prepend);
    bench_extend(c, "postpend", setup_postpend);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
