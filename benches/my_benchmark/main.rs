pub mod map_from_iter;
pub mod map_get;
pub mod set_extend;
mod map_sum_keys;

use criterion::{criterion_group, criterion_main};
use map_from_iter::map_from_iter_benchmark;
use map_get::map_get_benchmark;
use set_extend::set_extend_benchmark;
use map_sum_keys::map_sum_keys_benchmark;

criterion_group!(
    benches,
    map_sum_keys_benchmark,
    map_from_iter_benchmark,
    map_get_benchmark,
    set_extend_benchmark
);
criterion_main!(benches);
