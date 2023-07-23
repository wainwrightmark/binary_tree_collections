use binary_tree_collections::map::BinaryMap;
use criterion::{BenchmarkId, Criterion};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::iter::Sum;

pub fn map_sum_keys_benchmark(c: &mut Criterion) {
    bench_maps_sum_keys::<u64>(c, "U64 Maps Sum Keys", |x| x);
    bench_maps_sum_keys::<u32>(c, "U32 Maps Sum Keys", |x| x as u32);
    bench_maps_sum_keys::<u16>(c, "U16 Maps Sum Keys", |x| x as u16);
    bench_maps_sum_keys::<u8>(c, "U8 Maps Sum Keys", |x| x as u8);
}

fn bench_maps_sum_keys<TKey: Clone + Eq + Hash + Ord + Sum>(
    c: &mut Criterion,
    name: &str,
    make_key: impl Fn(u64) -> TKey,
) {
    let mut group = c.benchmark_group(name);

    for n in [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        let pairs: Vec<_> = (0..n).map(|v| v * 2).map(|v| (make_key(v), v)).collect();


        let hash_map: HashMap<TKey, u64> = HashMap::from_iter(pairs.iter().cloned());
        let btree_map: BTreeMap<TKey, u64> = BTreeMap::from_iter(pairs.iter().cloned());
        let binary_map: BinaryMap<TKey, u64> = BinaryMap::from_iter(pairs.iter().cloned());
        let hashbrown_map: hashbrown::HashMap<TKey, u64> = hashbrown::HashMap::from_iter(pairs.iter().cloned());

        group.bench_function(BenchmarkId::new("Hashmap", n), |b| {
            b.iter(|| hashmap_sum_keys(&hash_map))
        });
        group.bench_function(BenchmarkId::new("BTreeMap", n), |b| {
            b.iter(|| btree_map_sum_keys(&btree_map))
        });
        group.bench_function(BenchmarkId::new("BinaryMap", n), |b| {
            b.iter(|| binary_map_sum_keys(&binary_map))
        });
        
        group.bench_function(BenchmarkId::new("Hashbrown", n), |b| {
            b.iter(|| hashbrown_map_sum_keys(&hashbrown_map))
        });
    }

    group.finish();
}

fn btree_map_sum_keys<TKey :  Sum + Clone, TValue>(map: &BTreeMap<TKey, TValue>)-> TKey{
    map.keys().cloned().sum()
}

fn binary_map_sum_keys<TKey : Sum + Clone, TValue>(map: &BinaryMap<TKey, TValue>)-> TKey{
    map.keys().iter().cloned().sum()
}

fn hashbrown_map_sum_keys<TKey : Sum + Clone, TValue>(map: &hashbrown::HashMap<TKey, TValue>)-> TKey{
    map.keys().cloned().sum()
}

fn hashmap_sum_keys<TKey : Sum + Clone, TValue>(map: &HashMap<TKey, TValue>)-> TKey{
    map.keys().cloned().sum()
}

