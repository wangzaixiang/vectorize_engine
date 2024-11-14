#![feature(portable_simd)]

use std::fmt::Display;
use std::simd::{u32x16, u32x4, u32x8};
use std::simd::cmp::SimdOrd;
use rand::random;
use vector_example1::sort2::{debug_print, sort};
use vector_example1::timeit;

fn main() {

    test_merge_sort();
}

fn show(label: &str, v: &Vec<u32>, start: usize) {
    print!("{:10} = [", label);
    for i in 0..v.len() {

        if i >= start { print!("\x1b[0;34m"); }
        print!("{:3} ", v[i]);
        if i >= start { print!("\x1b[0m"); }

        if i % 4 == 3 { print!("    "); }
    }
    println!("]");
}

// TODO process the last 1, 2, 3 elements
unsafe fn merge_sort2(v1: &Vec<u32>, v2: &mut Vec<u32>) -> Vec<u32> {

    let mut result = Vec::with_capacity(v1.len() + v2.len());
    let (mut i, mut j, mut out) = (0, 0, 0);

    let mut vMin: u32x4 = u32x4::from_slice(&v1[i..]);  i+=4;
    let mut vMax = u32x4::from_slice(&v2[j..]); j+=4;

    while i < v1.len() && j < v2.len() {
        merge_sort_u32x4(vMin, vMax, &mut vMin, &mut vMax);
        result.extend_from_slice(vMin.as_array());

        if v1[i] < v2[j] {
            vMin = u32x4::from_slice(&v1[i..]);
            i += 4;
        } else {
            vMin = u32x4::from_slice(&v2[j..]);
            j += 4;
        }
    }

    while i < v1.len() {
        merge_sort_u32x4(vMin, vMax, &mut vMin, &mut vMax);
        result.extend_from_slice(vMin.as_array());
        vMin = u32x4::from_slice(&v1[i..]);
        i += 4;
    }
    while j < v2.len() {
        merge_sort_u32x4(vMin, vMax, &mut vMin, &mut vMax);
        result.extend_from_slice(vMin.as_array());
        vMin = u32x4::from_slice(&v2[j..]);
        j += 4;
    }

    merge_sort_u32x4(vMin, vMax, &mut vMin, &mut vMax);
    result.extend_from_slice(vMin.as_array());
    result.extend_from_slice(vMax.as_array());

    result

}

unsafe fn merge_sort(v1: &mut Vec<u32>, v2: &mut Vec<u32>) -> Vec<u32> {

    show("v1", v1, 0);
    show("v2", v2, 0);

    let len1 = v1.len() & !3;   // round down to multiple of 4
    let len2 = v2.len() & !3;   // round down to multiple of 4
    let mut result = Vec::with_capacity(v1.len() + v2.len());

    let (mut i, mut j) = (0, 0);

    while i + 1 < len1 && j + 1 < len2 {
        let a: &mut u32x4 = &mut *(&mut v1[i] as *mut u32 as *mut u32x4);
        let b: &mut u32x4 = &mut *(&mut v2[j] as *mut u32 as *mut u32x4);
        merge_sort_u32x4(*a, *b, a, b);

        result.extend_from_slice(a.as_array());

        if v1[i+4] < v2[j+4] {
            i += 4;
        }
        else {
            j += 4;
        }

        // print ascii blue
        println!(">>> ");
        show("v1", v1, i);
        show("v2", v2, j);
        show("result", &result, usize::MAX);
        println!();
    }

    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            result.push(v1[i]);
            i += 1;
        } else {
            result.push(v2[j]);
            j += 1;
        }
    }
    println!(">>3 result = {:?} i={i} j={j}", result);

    if i < v1.len() {
        result.extend_from_slice(&v1[i..]);
    }
    if j < v2.len() {
        result.extend_from_slice(&v2[j..]);
    }
    println!(">>4 result = {:?}", result);
    result
}

// port https://www.vldb.org/pvldb/vol8/p1274-inoue.pdf to std::simd
unsafe fn merge_sort_u32x4(vA: u32x4, vB: u32x4, vMin: &mut u32x4, vMax: &mut u32x4 ) {
    *vMin = vA.simd_min(vB);
    *vMax = vA.simd_max(vB);
    let mut vTemp = vMin.rotate_elements_left::<1>();

    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();


    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    *vMin = vMin.rotate_elements_left::<1>();
}

#[cfg(target_feature = "avx2")]
unsafe fn merge_sort_u32x8( vA: u32x8, vB: u32x8, vMin: &mut u32x8, vMax: &mut u32x8 ) {

    // round 0
    *vMin = vA.simd_min(vB);
    *vMax = vA.simd_max(vB);
    let mut vTemp = vMin.rotate_elements_left::<1>();

    // round 1
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 2
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 3
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 4
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 5
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 6
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 7
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    *vMin = vMin.rotate_elements_left::<1>();
}

#[cfg(not(target_feature = "avx2"))]
unsafe fn merge_sort_u32x8( vA: u32x8, vB: u32x8, vMin: &mut u32x8, vMax: &mut u32x8 ) {
    *vMin = vA;
    *vMax = vB;
    let vA0: &mut u32x4 = &mut *( &mut vMin[0] as *mut u32 as *mut u32x4);
    let vA1: &mut u32x4 = &mut *( &mut vMin[4] as *mut u32 as *mut u32x4);
    let vB0: &mut u32x4 = &mut *( &mut vMax[0] as *mut u32 as *mut u32x4);
    let vB1: &mut u32x4 = &mut *( &mut vMax[4] as *mut u32 as *mut u32x4);

    merge_sort_u32x4(*vA0, *vB0, vA0, vB0);
    merge_sort_u32x4(*vA1, *vB1, vA1, vB1);
    merge_sort_u32x4(*vA1, *vB0, vA1, vB0);
}

#[cfg(target_feature = "avx512f")]
unsafe fn merge_sort_u32x16( vA: u32x16, vB: u32x16, vMin: &mut u32x16, vMax: &mut u32x16) {
    // round 0
    *vMin = vA.simd_min(vB);
    *vMax = vA.simd_max(vB);
    let mut vTemp = vMin.rotate_elements_left::<1>();

    // round 1
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 2
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 3
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 4
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 5
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 6
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 7
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 8
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 9
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 10
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 11
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 12
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 13
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 14
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    vTemp = vMin.rotate_elements_left::<1>();

    // round 15
    *vMin = vTemp.simd_min(*vMax);
    *vMax = vTemp.simd_max(*vMax);
    *vMin = vMin.rotate_elements_left::<1>();
}

#[cfg(not(target_feature = "avx512f"))]
unsafe fn merge_sort_u32x16( vA: u32x16, vB: u32x16, vMin: &mut u32x16, vMax: &mut u32x16) {
    *vMin = vA;
    *vMax = vB;
    let vA0: &mut u32x8 = &mut *( &mut vMin[0] as *mut u32 as *mut u32x8);
    let vA1: &mut u32x8 = &mut *( &mut vMin[8] as *mut u32 as *mut u32x8);
    let vB0: &mut u32x8 = &mut *( &mut vMax[0] as *mut u32 as *mut u32x8);
    let vB1: &mut u32x8 = &mut *( &mut vMax[8] as *mut u32 as *mut u32x8);

    merge_sort_u32x8(*vA0, *vB0, vA0, vB0);
    merge_sort_u32x8(*vA1, *vB1, vA1, vB1);
    merge_sort_u32x8(*vA1, *vB0, vA1, vB0);
}


#[test]
fn test_merge_sort_u32x4(){
    let zero = rand::random::<u32>() % 2;   // either 0 or 1
    unsafe {
        let mut vA = u32x4::from_array([zero, 20, 30, 40]);
        let mut vB = u32x4::from_array([2, 8, 25, 45]);

        // concat vA and vB
        let mut vec = Vec::new();
        vA.as_array().iter().for_each(|x| vec.push(*x));
        vB.as_array().iter().for_each(|x| vec.push(*x));
        vec.sort();

        merge_sort_u32x4(vA, vB, &mut vA, &mut vB );
        assert_eq!(vA.as_array(), &vec[0..4]);
        assert_eq!(vB.as_array(), &vec[4..8]);
        println!("min: {:?} max: {:?}", vA, vB);
    }
}

#[test]
fn test_merge_sort_u32x8(){
    for _ in 0..256 {
        unsafe {
            let mut a1: [u32; 8] = [0u32; 8];
            let mut a2 = [0u32; 8];
            let mut all = Vec::new();

            // generate 8 random u32
            for i in 0..8 {
                a1[i] = rand::random::<u32>() % 1000;
                a2[i] = rand::random::<u32>() % 1000;
                all.push(a1[i]);
                all.push(a2[i]);
            }
            a1.sort();
            a2.sort();
            all.sort();

            let mut vA = u32x8::from_array(a1);
            let mut vB = u32x8::from_array(a2);

            merge_sort_u32x8(vA, vB, &mut vA, &mut vB);
            assert_eq!(vA.as_array(), &all[0..8]);
            assert_eq!(vB.as_array(), &all[8..16]);
        }
    }
}

#[test]
fn test_merge_sort_u32x16() {
   for _ in 0..256 {
         unsafe {
              let mut a1: [u32; 16] = [0u32; 16];
              let mut a2 = [0u32; 16];
              let mut all = Vec::new();

              // generate 16 random u32
              for i in 0..16 {
                a1[i] = rand::random::<u32>() % 1000;
                a2[i] = rand::random::<u32>() % 1000;
                all.push(a1[i]);
                all.push(a2[i]);
              }
              a1.sort();
              a2.sort();
              all.sort();

              let mut vA = u32x16::from_array(a1);
              let mut vB = u32x16::from_array(a2);

              merge_sort_u32x16(vA, vB, &mut vA, &mut vB);
              assert_eq!(vA.as_array(), &all[0..16]);
              assert_eq!(vB.as_array(), &all[16..32]);
         }
   }
}


fn test_merge_sort() {

    for _ in 0..256 {
        let mut a1: Vec<u32> = Vec::new();
        let mut a2: Vec<u32> = Vec::new();
        let mut all: Vec<u32> = Vec::new();

        // generate 16 random u32
        for _ in 0..16 { // 4 * 4 + 2
            a1.push(rand::random::<u32>() % 1000);
            a2.push(rand::random::<u32>() % 1000);
        }
        for _ in 0..4 {
            let r = rand::random::<u32>() % 2;
            if r == 0 {
                a1.push(rand::random::<u32>() % 1000);
                a1.push(rand::random::<u32>() % 1000);
                a1.push(rand::random::<u32>() % 1000);
                a1.push(rand::random::<u32>() % 1000);
            } else {
                a2.push(rand::random::<u32>() % 1000);
                a2.push(rand::random::<u32>() % 1000);
                a2.push(rand::random::<u32>() % 1000);
                a2.push(rand::random::<u32>() % 1000);
            }
        }

        all.extend_from_slice(&a1);
        all.extend_from_slice(&a2);
        a1.sort();
        a2.sort();
        all.sort();

        let result = unsafe { merge_sort2(&mut a1, &mut a2) };
        assert_eq!(result, all);

        println!("a1 = {:?}\na2 = {:?}\n", a1, a2);
    }

}

#[test]
fn test_sort(){

    for len in 1..1000 {

        for _ in 0..2 {    // each len test 64 times
            let LEN: usize = 16 * len;
            let vec: Vec<u32> = (0..LEN).map(|_| random::<u32>() % 1_000_000).collect();
            let mut expected = vec.clone();
            expected.sort();

            let mut result = vec.clone();
            sort(&mut result);

            if result != expected {
                println!("vec = {:?}", vec);
                debug_print("vec", &vec);
                debug_print("result", &result);
                debug_print("expect", &expected);
            }
            assert_eq!(result, expected);

        }
    }
}