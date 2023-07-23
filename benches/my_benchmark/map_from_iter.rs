use binary_tree_collections::map::BinaryMap;
use cc_traits::Len;
use criterion::{BenchmarkId, Criterion};
use hashbrown::hash_map::DefaultHashBuilder;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub fn map_from_iter_benchmark(c: &mut Criterion) {
    bench_maps_from_iter::<u64>(c, "U64 Maps From Iter", |x| x);
    bench_maps_from_iter::<u32>(c, "U32 Maps From Iter", |x| x as u32);
    bench_maps_from_iter::<u16>(c, "U16 Maps From Iter", |x| x as u16);
    bench_maps_from_iter::<u8>(c, "U8 Maps From Iter", |x| x as u8);
    bench_maps_from_iter::<String>(c, "String Maps From Iter", |x| x.to_string());
}

fn bench_maps_from_iter<TKey: Clone + Eq + Hash + Ord>(
    c: &mut Criterion,
    name: &str,
    make_key: impl Fn(u64) -> TKey,
) {
    let mut group = c.benchmark_group(name);

    for n in [1u64, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096] {
        group.throughput(criterion::Throughput::Elements(n as u64));
        let pairs: Vec<_> = (0..n).map(|v| v * 2).map(|v| (make_key(v), v)).collect();

        group.bench_function(BenchmarkId::new("Hashmap", n), |b| {
            b.iter(|| bench_map_from_iter::<TKey, u64, HashMap<_, _>>(&pairs))
        });
        group.bench_function(BenchmarkId::new("BTreeMap", n), |b| {
            b.iter(|| bench_map_from_iter::<TKey, u64, BTreeMap<_, _>>(&pairs))
        });
        group.bench_function(BenchmarkId::new("BinaryMap", n), |b| {
            b.iter(|| bench_map_from_iter::<TKey, u64, BinaryMap<_, _>>(&pairs))
        });
        
        
        group.bench_function(BenchmarkId::new("Hashbrown HashMap", n), |b| {
            b.iter(|| bench_hasbrown_from_iter::<TKey, u64>(&pairs))
        });
    }

    group.finish();
}

fn bench_map_from_iter<TKey: Clone, TValue: Clone, TMap: FromIterator<(TKey, TValue)> + Len>(
    slice: &[(TKey, TValue)],
) -> usize {
    let map = TMap::from_iter(slice.iter().cloned());
    map.len()
}

fn bench_hasbrown_from_iter<TKey: Clone + Eq +Hash, TValue: Clone>(
    slice: &[(TKey, TValue)],
) -> usize {
    let map: hashbrown::HashMap<TKey, TValue, DefaultHashBuilder> = hashbrown::HashMap::from_iter(slice.iter().cloned());
    map.len()
}
