#![feature(portable_simd)]

use std::simd::{u32x16, u32x32, u32x8};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::random;
use vector_example1::sort2::{merge_sort_u32x16x2, sort, sort_u32x16, sort_u32x32, sort_u32x8};

pub fn criterion_benchmark(c: &mut Criterion) {

    let rand_nums32: Vec<u32> = (0..32).map( |_ | random::<u32>() ).collect();
    let mut rand_nums16_1: Vec<u32> = (0..16).map( |_ | random::<u32>() ).collect();
    let mut rand_nums16_2: Vec<u32> = (0..16).map( |_ | random::<u32>() ).collect();

    let mut vec = u32x32::from_slice(&rand_nums32);
    c.bench_function("sort_u32x32", |b| b.iter(|| sort_u32x32(black_box(&mut vec))));

    rand_nums16_1.sort();
    rand_nums16_2.sort();
    let num1: &mut u32x16 = unsafe { &mut *(&mut rand_nums16_1[0] as *mut u32 as *mut u32x16) };
    let num2: &mut u32x16 = unsafe { &mut *(&mut rand_nums16_2[0] as *mut u32 as *mut u32x16) };
    c.bench_function("merge_sort_u32x16x2", |b| b.iter(|| merge_sort_u32x16x2(num1, num2)));

    // {
    //     let mut rand_nums: Vec<u32> = (0..64).map(|_| random::<u32>()).collect();
    //     c.bench_function("sort 16x4", |b| b.iter(|| sort(&mut rand_nums)));
    // }
    //
    // {
    //     let mut rand_nums: Vec<u32> = (0..128).map(|_| random::<u32>()).collect();
    //     c.bench_function("sort 16x8", |b| b.iter(|| sort(&mut rand_nums)));
    // }
    // {
    //     let mut rand_nums: Vec<u32> = (0..256).map(|_| random::<u32>()).collect();
    //     c.bench_function("sort 16x16", |b| b.iter(|| sort(&mut rand_nums)));
    // }
    // {
    //     let rand_nums1: Vec<u32> = (0..8192*8).map(|_| random::<u32>()).collect();
    //     let rand_nums2: Vec<u32> = rand_nums1.clone();
    //     let rand_nums3: Vec<u32> = rand_nums2.clone();
    //
    //     c.bench_function("clone 16x512", |b| b.iter(|| black_box(&rand_nums1).clone() ));
    //     c.bench_function("sort 16x512 SIMD", |b| b.iter(|| sort(&mut black_box(&rand_nums2).clone())));
    //     c.bench_function("sort 16x512 rust", |b| b.iter(|| black_box(&rand_nums3).clone().sort() ));
    // }
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

pub fn compare2(c: &mut Criterion) {
    // 32, 64, 128, 256, 512, 1024, 2048, 4096
    let mut group = c.benchmark_group("sort");

    let mut rand_nums32: Vec<u32> = (0..32).map( |_ | random::<u32>() ).collect();
    let mut rand_nums16_1: Vec<u32> = (0..16).map( |_ | random::<u32>() ).collect();
    let mut rand_nums16_2: Vec<u32> = (0..16).map( |_ | random::<u32>() ).collect();

    rand_nums16_1.sort();
    rand_nums16_2.sort();
    let num1: &mut u32x16 = unsafe { &mut *(&mut rand_nums16_1[0] as *mut u32 as *mut u32x16) };
    let num2: &mut u32x16 = unsafe { &mut *(&mut rand_nums16_2[0] as *mut u32 as *mut u32x16) };

    group.bench_with_input(BenchmarkId::new("sort_u32x32", ""), &1, |b, _| b.iter( || {
        let vec: &mut u32x32 = unsafe { &mut *(&mut rand_nums32[0] as *mut u32 as *mut u32x32) };
        sort_u32x32(black_box(vec));
    }));
    group.bench_with_input(BenchmarkId::new("merge_sort_u32x16x2", ""), &1, |b, _| b.iter( || {
        // merge_sort_u32x16x2(black_box(num1), black_box(num2));
        merge_sort_u32x16x2(num1, num2);
    }));

    for i in 0..8 {
        let len = 32 << i;
        let vec = (0..len).map(|_| random::<u32>()).collect::<Vec<u32>>();

        group.bench_with_input(BenchmarkId::new("simd", len ), &1, |b, _| b.iter(|| {
            let mut vec = black_box(&vec).clone();
            sort(&mut vec);
        }));
        group.bench_with_input(BenchmarkId::new("rust", len ), &1, |b, _| b.iter(||{
            let mut vec = black_box(&vec).clone();
            vec.sort();
        }));
    }

}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, compare2);
criterion_main!(benches);