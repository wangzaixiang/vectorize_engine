#![feature(portable_simd)]

use std::hint::black_box;
use rand::random;
use vector_example1::sort2::{debug_print, sort};
use vector_example1::timeit;

fn main() {

    test_sort();
    // test_sort2();
    // test_sort_2();
}


#[test]
fn test_mergeu32x16x2(){
    use std::simd::u32x16;

    let vec1 = vec![11, 11, 14, 18, 19, 21, 27, 41, 42, 44, 67, 70, 82, 90, 96, 98];
    let vec2 = vec! [6, 16, 21, 29, 41, 46, 53, 54, 55, 56, 63, 64, 66, 92, 93, 97];

    let mut min = u32x16::from_slice(&vec1);
    let mut max = u32x16::from_slice(&vec2);

    vector_example1::sort2::merge_sort_u32x16x2(&mut min, &mut max);

    println!("min = {:?}", min);
    println!("max = {:?}", max);

    let mut expect = Vec::new();
    expect.extend_from_slice(&vec1);
    expect.extend_from_slice(&vec2);
    expect.sort();

    println!("expect: {:?}", expect);

}



fn test_sort(){

    for order in 0..10 {
        let (mut rust, mut simd) = (0u64, 0u64);
        let len: usize = 32<<order;
        const LOOP: u64 = 100_000;
        for _ in 0..LOOP {
            let vec: Vec<u32> = (0..len).map(|_| random::<u32>()).collect();
            let (_, time1) = timeit("rust sort", || {
                let mut expected = black_box(&vec).clone();
                expected.sort();
            });

            let (_, time2) = timeit("simd sort", || {
                let mut vec = black_box(&vec).clone();
                sort(black_box(&mut vec));
            });

            rust += time1.as_nanos() as u64;
            simd += time2.as_nanos() as u64;
        }
        println!("len = {:04} rust = {:8}ns  simd = {:8}ns", len, rust/LOOP, simd/LOOP);
    }
}

fn _test_sort_2(){

    // for order in 0..10 {
        let (mut rust, mut simd) = (0u64, 0u64);
        let len: usize = 256 * 256 * 16;    // = 1M
        const LOOP: u64 = 1000;
        for _ in 0..LOOP {
            let vec: Vec<u32> = (0..len).map(|_| random::<u32>()).collect();
            let (_, time1) = timeit("rust sort", || {
                let mut expected = black_box(&vec).clone();
                expected.sort();
            });

            let (_, time2) = timeit("simd sort", || {
                let mut vec = black_box(&vec).clone();
                sort(black_box(&mut vec));
            });

            rust += time1.as_micros() as u64;
            simd += time2.as_micros() as u64;
        }
        println!("len = {:04} rust = {:8}us  simd = {:8}us", len, rust/LOOP, simd/LOOP);
    // }
}


fn _test_sort2() {
    const LEN: usize = 16 * 4;
        let vec: Vec<u32> = (0..LEN).map(|_| random::<u32>() % 1_000).collect();
        let mut expected = vec.clone();
        println!("vec = {:?}", vec);

        let (_, _) = timeit("rust sort", || expected.sort() );

        let mut result = vec.clone();

        let (_, _) = timeit("simd sort", || sort(&mut result) );

        if result != expected {
            println!("vec = {:?}", vec);
            debug_print("vec", &vec);
            debug_print("result", &result);
            debug_print("expect", &expected);
        }
        assert_eq!(result, expected);
}


fn _test1(){
    // let mut vec = vec! [409, 212, 141, 930, 582, 992, 65, 918, 530, 76, 331, 317, 857, 899, 743, 72, 513, 744, 450, 65, 280, 366, 593, 142, 908, 259, 668, 211, 677, 600, 797, 395, 550, 410, 905, 699, 822, 516, 829, 655, 324, 392, 962, 231, 101, 157, 448, 910, 611, 396, 494, 692, 216, 817, 953, 1, 966, 727, 877, 982, 368, 504, 759, 890, 737, 373, 581, 198, 427, 361, 21, 462, 850, 26, 470, 185, 980, 102, 870, 791, 667, 94, 949, 54, 91, 288, 554, 962, 866, 864, 3, 115, 342, 723, 663, 768, 55, 141, 517, 124, 441, 656, 913, 121, 196, 651, 966, 445, 718, 13, 590, 897, 893, 692, 435, 890, 353, 340, 524, 354, 207, 460, 449, 39, 715, 518, 698, 756];
    let mut vec = vec! [524, 497, 73, 21, 680, 712, 634, 680, 591, 823, 114, 734, 404, 572, 233, 305, 924, 709, 681, 958, 927, 483, 401, 693, 256, 560, 981, 924, 880, 398, 300, 394, 98, 568, 141, 24, 939, 176, 809, 207, 940, 198, 945, 935, 33, 744, 49, 603, 377, 463, 172, 20, 933, 746, 692, 871, 141, 793, 102, 627, 558, 507, 601, 56];
    let mut expected = vec.clone();
    expected.sort();

    // debug_print("before", &vec);
    //
    let _ = timeit("sort", || {
        for _ in 0..8192*128 {
            sort(black_box(&mut vec));
        }
    });
    let time2 = timeit("sort", || {
        for _ in 0..8192*128 {
            sort(black_box(&mut vec));
        }
    });

    // debug_print("afer", &vec);

    assert_eq!(vec, expected);

    println!("time = {}", time2.1.as_nanos()/8192/128);
}