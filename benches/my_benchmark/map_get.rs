use binary_tree_collections::map::BinaryMap;
use cc_traits::Map;
use criterion::{BenchmarkId, Criterion};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

pub fn map_get_benchmark(c: &mut Criterion) {
    bench_maps_get::<u64>(c, "U64 Maps Get", |x| x);
    bench_maps_get::<u32>(c, "U32 Maps Get", |x| x as u32);
    bench_maps_get::<u16>(c, "U16 Maps Get", |x| x as u16);
    bench_maps_get::<u8>(c, "U8 Maps Get", |x| x as u8);
    bench_maps_get::<String>(c, "String Maps Get", |x| x.to_string());
}

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
        let hashbrown_map: hashbrown::HashMap<TKey, u64> = hashbrown::HashMap::from_iter(pairs.iter().cloned());

        group.bench_function(BenchmarkId::new("Hashmap", n), |b| {
            b.iter(|| bench_map_get(&hash_map, &keys_to_search))
        });
        group.bench_function(BenchmarkId::new("BTreeMap", n), |b| {
            b.iter(|| bench_map_get(&btree_map, &keys_to_search))
        });
        group.bench_function(BenchmarkId::new("BinaryMap", n), |b| {
            b.iter(|| bench_map_get(&binary_map, &keys_to_search))
        });
        
        group.bench_function(BenchmarkId::new("Hashbrown", n), |b| {
            b.iter(|| bench_hashbrown_get(&hashbrown_map, &keys_to_search))
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

fn bench_hashbrown_get<'a, TKey : Eq +Hash>(
    map: &'a hashbrown::HashMap<TKey, u64>,
    keys: &[TKey],
) -> u64 {
    keys.iter().flat_map(|key| map.get(key)).cloned().sum()
}
