#![feature(portable_simd)]

use std::simd::{u32x16, u32x32, u32x8};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::random;
use vector_example1::sort2::{sort_u32x16, sort_u32x32, sort_u32x8};

pub fn criterion_benchmark(c: &mut Criterion) {

    let rand_nums: Vec<u32> = (0..8).map( |_ | random::<u32>() ).collect();
    let mut vec = u32x8::from_slice(&rand_nums);

    c.bench_function("sort_u32x8", |b| b.iter(|| sort_u32x8(black_box(&mut vec))));

}



pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("sort");
    {
        let rand_nums: Vec<u32> = (0..8).map(|_| random::<u32>()).collect();
        let mut vec2 = rand_nums.clone();
        let mut vec = u32x8::from_slice(&rand_nums);

        group.bench_with_input(BenchmarkId::new("simd sort u32x8", 1), &1,
                               |b, i| b.iter(|| {
                                   vec = u32x8::from_slice(&rand_nums);
                                   vec[0] = *i as u32;
                                   sort_u32x8(black_box(&mut vec))
                               }));

        group.bench_with_input(BenchmarkId::new("simple sort u32x8", 1), &1,
                               |b, i| b.iter(|| {
                                   vec2.clear();
                                   vec2.extend_from_slice(&rand_nums);
                                   vec2[0] = *i as u32;
                                   vec2.sort();
                               } ));
    }

    {
        let rand_nums: Vec<u32> = (0..16).map(|_| random::<u32>()).collect();
        let mut vec2 = rand_nums.clone();
        let mut vec = u32x16::from_slice(&rand_nums);

        group.bench_with_input(BenchmarkId::new("simd sort u32x16", 1), &1,
                               |b, i| b.iter(|| {
                                   vec = u32x16::from_slice(&rand_nums);
                                   vec[0] = *i as u32;
                                   sort_u32x16(black_box(&mut vec))
                               }));

        group.bench_with_input(BenchmarkId::new("simple sort u32x16", 1), &1,
                               |b, i| b.iter(|| {
                                   vec2.clear();
                                   vec2.extend_from_slice(&rand_nums);
                                   vec2[0] = *i as u32;
                                   vec2.sort();
                               } ));
    }

    {
        let rand_nums: Vec<u32> = (0..32).map(|_| random::<u32>()).collect();
        let mut vec2 = rand_nums.clone();
        let mut vec = u32x32::from_slice(&rand_nums);

        group.bench_with_input(BenchmarkId::new("simd sort u32x32", 1), &1,
                               |b, i| b.iter(|| {
                                   vec = u32x32::from_slice(&rand_nums);
                                   vec[0] = *i as u32;
                                   sort_u32x32(black_box(&mut vec))
                               }));

        group.bench_with_input(BenchmarkId::new("simple sort u32x32", 1), &1,
                               |b, i| b.iter(|| {
                                   vec2.clear();
                                   vec2.extend_from_slice(&rand_nums);
                                   vec2[0] = *i as u32;
                                   vec2.sort();
                               } ));
    }

}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, compare);
criterion_main!(benches);