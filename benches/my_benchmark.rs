use binaryset::{map::BinaryMap, set::*};
use cc_traits::{Len, Map};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

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

fn extend_with_inserts(set: &mut BinarySet<usize>, vec: &Vec<usize>) -> usize {
    set.extend_with_inserts(vec.iter().cloned());
    set.len()
}

fn extend_with_rotates(set: &mut BinarySet<usize>, vec: &Vec<usize>) -> usize {
    set.extend_with_rotates(vec.iter().cloned());
    set.len()
}

fn bench_extend(
    c: &mut Criterion,
    name: &str,
    get_args: impl Fn(usize) -> (BinarySet<usize>, Vec<usize>),
) {
    let mut group = c.benchmark_group(name);

    for n in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        let args = get_args(n);
        group.bench_function(BenchmarkId::new("extend normal", n), |b| {
            b.iter_batched_ref(
                || args.0.clone(),
                |a| extend(a, &args.1),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("extend with inserts", n), |b| {
            b.iter_batched_ref(
                || args.0.clone(),
                |a| extend_with_inserts(a, &args.1),
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_function(BenchmarkId::new("extend with rotates", n), |b| {
            b.iter_batched_ref(
                || args.0.clone(),
                |a| extend_with_rotates(a, &args.1),
                criterion::BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

fn set_extend_benchmark(c: &mut Criterion) {
    bench_extend(c, "interleave", setup_interleave);
    bench_extend(c, "prepend", setup_prepend);
    bench_extend(c, "postpend", setup_postpend);
}

fn map_get_benchmark(c: &mut Criterion) {
    bench_maps_get::<u64>(c, "U64 Maps", |x| x);
    bench_maps_get::<u32>(c, "U32 Maps", |x| x as u32);
    bench_maps_get::<u16>(c, "U16 Maps", |x| x as u16);
    bench_maps_get::<u8>(c, "U8 Maps", |x| x as u8);
    bench_maps_get::<String>(c, "String Maps", |x| x.to_string());
}

criterion_group!(benches, map_get_benchmark, set_extend_benchmark);
criterion_main!(benches);

fn bench_maps_get<TKey: Clone + Eq + Hash + Ord>(
    c: &mut Criterion,
    name: &str,
    make_key: impl Fn(u64) -> TKey,
) {
    let mut group = c.benchmark_group(name);

    for n in [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        let pairs: Vec<_> = (0..n).map(|v| v * 2).map(|v| (make_key(v), v)).collect();

        let keys_to_search: Vec<TKey> = (0..(n * 2)).map(|v| make_key(v)).collect();

        let hash_map: HashMap<TKey, u64> = HashMap::from_iter(pairs.iter().cloned());
        let btree_map: BTreeMap<TKey, u64> = BTreeMap::from_iter(pairs.iter().cloned());
        let binary_map: BinaryMap<TKey, u64> = BinaryMap::from_iter(pairs.iter().cloned());

        group.bench_function(BenchmarkId::new("Hashmap", n), |b| {
            b.iter(|| bench_map_get(&hash_map, &keys_to_search))
        });
        group.bench_function(BenchmarkId::new("BTreeMap", n), |b| {
            b.iter(|| bench_map_get(&btree_map, &keys_to_search))
        });
        group.bench_function(BenchmarkId::new("BinaryMap", n), |b| {
            b.iter(|| bench_map_get(&binary_map, &keys_to_search))
        });
    }

    group.finish();
}



fn bench_map_get<'a, TKey, TMap: 'a + Map<TKey, u64, ItemRef<'a> = &'a u64>>(
    map: &'a TMap,
    keys: &[TKey],
) -> u64 {
    keys.iter().flat_map(|key| map.get(key)).cloned().sum()
}
