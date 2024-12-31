#![feature(portable_simd)]

use std::hint::black_box;
use clap::{Parser, Subcommand};
use rand::random;
use vector_example1::sort2::sort;
use vector_example1::timeit;

#[derive(Parser)]
#[command(version, author, about)]
struct Cli {

    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Compare,
    Drift {
        /// size of the vector, will rounded up to the next power of 16, default 1M
        #[arg(short, long, default_value = "1048576")]
        size: usize,

        /// number of loops, default 256
        #[arg(short, long, default_value = "256")]
        loops: usize
    },
    Simd {
        /// size of the vector, will rounded up to the next power of 16, default 1M
        #[arg(short, long, default_value = "1048576")]
        size: usize,

        /// number of loops, default 256
        #[arg(short, long, default_value = "256")]
        loops: usize
    }
}

fn main() {

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Compare) => compare_sort_alg(),
        Some(Commands::Drift{size,loops}) => run_drift((size+15)/16*16, loops),
        Some(Commands::Simd{size, loops}) => run_simd( (size+15)/16*16, loops),
        None => println!("No command specified")
    }
}


fn compare_sort_alg(){

    // compare 32, 64, 128, 256, ... 32M
    println!("len,stable sort-ns,unstable sort-ns,simd sort-ns");
    for order in 0..20 {
        let (mut stable,mut unstable, mut simd) = (0u64, 0u64, 0u64);
        let len: usize = 32<<order;
        let loops: u64 = match order {
            0..4 => 1_000_000,
            4..8 => 100_000,
            8..12 => 10_000,
            _ => 1000
        };
        for _ in 0..loops {
            let vec: Vec<u32> = (0..len).map(|_| random::<u32>()).collect();
            let (_, time1) = timeit("rust stable sort", || {
                let mut expected = black_box(&vec).clone();
                expected.sort();
            });

            let (_, time2) = timeit("rust unstable sort", || {
                let mut expected = black_box(&vec).clone();
                expected.sort_unstable();
            });

            let (_, time3) = timeit("simd sort", || {
                let mut vec = black_box(&vec).clone();
                sort(black_box(&mut vec));
            });

            stable += time1.as_nanos() as u64;
            unstable += time2.as_nanos() as u64;
            simd += time3.as_nanos() as u64;
        }
        println!("{},{},{},{}", len, stable/ loops, unstable/ loops, simd/ loops);
    }
}

fn run_drift(size: usize, loops: usize){

    let vec = (0..size).map(|_| rand::random() ).collect::<Vec<u32>>();

    let time = timeit("run_drift", || for _ in 0..loops {
            let mut vec2 = black_box(&vec).clone();
            vec2.sort()
        } );

    println!("drift sort time = {:?}us/iteration", time.1.as_micros() / loops as u128);

}

fn run_simd(size: usize, loops: usize){
    let vec = (0..size).map(|_| random() ).collect::<Vec<u32>>();

    let time = timeit("run_drift", || for _ in 0..loops {
        let mut vec2 = black_box(&vec).clone();
        sort(&mut vec2)
    } );

    println!("simd sort time = {:?}us/iteration", time.1.as_micros() / loops as u128);

}

// step debug for the quicksort method
#[test]
fn _tmp_test_driftsoft(){
    let mut vec = (0..65536*16).map(|_| random::<u8>() % 10 + '0' as u8 ).collect::<Vec<u8>>();
    vec.sort();
    println!("vec = {:?}", vec);
}