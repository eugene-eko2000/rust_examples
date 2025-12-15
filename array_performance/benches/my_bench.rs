use criterion::{criterion_group, criterion_main, Criterion};
use core::hint::black_box;

#[inline(never)]
pub fn sum_array_1(arr: &[u64]) -> u64 {
    // if arr.len() < 100 {
    //     return 0;
    // }
    let mut result = 0u64;
    result += arr[0];
    result += arr[1];
    result += arr[2];
    result += arr[3];
    result += arr[4];
    result += arr[5];
    result += arr[6];
    result += arr[7];
    result += arr[8];
    result += arr[9];
    result
}

#[inline(never)]
pub fn sum_array_2(arr: &[u64]) -> u64 {
    if arr.len() < 10 {
        return 0;
    }
    let mut result = 0u64;
    result += arr[0];
    result += arr[1];
    result += arr[2];
    result += arr[3];
    result += arr[4];
    result += arr[5];
    result += arr[6];
    result += arr[7];
    result += arr[8];
    result += arr[9];
    result
}

fn work1(arr: &[u64]) -> u64 {
    sum_array_1(&arr)
}

fn work2(arr: &[u64]) -> u64 {
    sum_array_2(&arr)
}

fn bench_work(c: &mut Criterion) {
    let arr = (0..10u64).collect::<Vec<u64>>();
    c.bench_function("work1", |b| {
        b.iter(|| work1(black_box(&arr)))
    });
    c.bench_function("work2", |b| {
        b.iter(|| work2(black_box(&arr)))
    });
}

criterion_group!(benches, bench_work);
criterion_main!(benches);
