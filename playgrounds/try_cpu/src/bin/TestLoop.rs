use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use memmap2::{Mmap, MmapOptions};
use rand::RngCore;

/// test IPC for LOOP

fn main() {
    let test = std::env::args().nth(1).unwrap_or("test1".to_string());

    let tm0 = std::time::Instant::now();

    let (sum, loops) =   if test == "test1" {
        TestCase::new("test1", 1_000_000_000, generate_data1)
            .execute()
    } else if test == "test2" {
        TestCase::new("test2", 1_000_000_000, generate_data2).execute()   // Fast
    } else if test == "test3" {
        TestCase::new("test3", 300_000_000, generate_data3)
            .execute()
    } else {
        println!("Usage: {} [test1|test2|test3]", std::env::args().nth(0).unwrap());
        return;
    };

    let tm1 = std::time::Instant::now();
    let elapsed = tm1.duration_since(tm0).as_secs_f64();

    println!("case: {}, sum: {}, loops: {}, elapsed: {:.3}sec, avg: {:.3} ns/iter", test, sum, loops, elapsed, tm1.duration_since(tm0).as_nanos() as f64 / loops as f64);
}

struct TestCase {
    name: String,   // same as mmap file
    numbers: usize, // number of elements
    mmap: Mmap
}

impl TestCase {
    fn new<F: Fn(&mut [u64])>(name: &str, numbers: usize, f: F) -> TestCase {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("out/{}.map", name))
            .unwrap();

        if file.metadata().unwrap().len() as usize == numbers * 8 {     // file exists, load it
            let mmap = unsafe { MmapOptions::new().map(&file).unwrap() }; // load the file
            TestCase {
                name: name.to_string(),
                numbers,
                mmap
            }
        }
        else {  // create a new file and generate data
            file.set_len(numbers as u64 * 8).unwrap();
            let mut mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
            let slice: &mut[u64] = unsafe { std::slice::from_raw_parts_mut(mmap.as_mut_ptr() as *mut u64, mmap.len() / 8) };
            f(slice);
            mmap.flush().unwrap(); // flush the file
            println!("generate file: out/{}.map", name);

            let mmap = unsafe { MmapOptions::new().map(&file).unwrap() }; // load the file

            TestCase {
                name: name.to_string(),
                numbers,
                mmap
            }
        }
    }

    fn execute(&self) -> (u64, u64) {
        let mut sum = 0u64;
        let mut loops = 0u64;
        let numbers = unsafe { std::slice::from_raw_parts(self.mmap.as_ptr() as *const u64, self.numbers) };

        numbers.iter().for_each( |&n| {
            let mut n = n;
            while n > 0 {
                let bit = n.trailing_zeros();
                n &= !(1 << bit);
                sum += bit as u64;
                loops += 1;
            }
        });

        (sum, loops)
    }

}

fn generate_data3(slice: &mut [u64]) {
    let mut random = rand::thread_rng();
    (0..slice.len()).for_each( |n| slice[n] = random.next_u64());
}

fn generate_data1(slice: &mut [u64]) {
    (0..slice.len()).for_each( |n| slice[n] = n as u64);
}

fn generate_data2(slice: &mut [u64]) {
    let nums = [0xFFF7u64, 0xFFF70, 0xFFF700, 0xFFF7000, 0xFFF70000, 0xFFF700000, 0xFFF7000000, 0xFFF70000000];
    (0..slice.len()).for_each( |n| slice[n] = nums[ n & 0x7 ]);
}
