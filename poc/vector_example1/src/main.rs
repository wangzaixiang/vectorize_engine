#![feature(portable_simd)]
#![feature(extend_one_unchecked)]

use std::collections::HashMap;
/// example1 demonstrates the use of:
/// 1. flat vectors
/// 2. do some filters
/// 3. do hash group and aggregations
///

use std::f64;
use std::ops::BitAnd;
use std::simd::{f64x16, mask8x16, u32x16, Mask};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};

// mod vector;


#[derive(PartialEq)]
struct Orders {
    order_id: Vec<u32>,
    sku_id: Vec<u32>,
    amount: Vec<f64>,
}

fn timeit<F, T>(_name: &str, mut f: F) -> (T, usize) where F: FnMut()-> T {
    let start = std::time::Instant::now();
    let t = f();
    let elapsed = start.elapsed();
    // println!("{}: {}.{:03} seconds", name, elapsed.as_secs(), elapsed.subsec_millis());
    (t, elapsed.as_millis() as usize)
}

fn main() {

    // fn group_row(ht: &mut HashMap<u32, (f64, u32)>, sku_id: u32, amount: f64) {
    let (orders, _) = timeit("prepare_data", || prepare_data() );

    for i in 0..16 {
        let (filtered, filter_data_time) = timeit("filter_data_loop", || filter_data(&orders));
        let (expected, group_by_time) = timeit("group-by", || group_data(&filtered));
        // expected.insert(0, (0.0, 0));

        println!("filtered: {}ms, grouped: {}ms, total is {}ms", filter_data_time, group_by_time, filter_data_time + group_by_time);

        {
            let (result, filter_aggregate_time) = timeit("filter_data_simd", || unsafe { filter_aggregate(&orders) });

            if result != expected {
                println!("expect: {:?}", expected);
                println!("filter_aggregate result: {:?}", result);
                panic!("expect != result");
            }
            println!("filter_aggregate {}ms", filter_aggregate_time);
        }

        {
            let (filtered, filter_simd_time) = timeit("filter_simd", || unsafe { filter_simd(&orders) });
            let (result, group_by_time) = timeit("group-by", || group_data(&filtered));

            if result != expected {
                println!("expect: {:?}", expected);
                println!("filter_simd result: {:?}", result);
                panic!("expect != result");
            }
            println!("filter_simd: {}ms, grouped: {}ms, total is {}ms", filter_simd_time, group_by_time, filter_simd_time + group_by_time);
        }

        {
            let (result, filter_simd_aggregate_time) = timeit("filter_simd_aggregate", || unsafe { filter_simd_aggregate(&orders) });

            if result != expected {
                println!("expect: {:?}", expected);
                println!("filter_simd_aggregate result: {:?}", result);
                panic!("expect != result");
            }
            println!("filter_simd_aggregate {}ms", filter_simd_aggregate_time);
        }

        println!()

    }


}

// order_id, sku_id, amount
fn prepare_data() -> Orders {
    let mut order_ids = vec![];
    let mut sku_ids = vec![];
    let mut amounts = vec![];

    for i in 0u32..1_000_000_000 {
    // for i in 0u32..1_000_000 {
        order_ids.push(i);
        // generate a random number between 0 and 10000
        let r1 = rand::random::<u32>() % 100;
        let r2 = rand::random::<u32>() % 10000;
        let r = if r1 < 90 { r1 } else { r2 };
        sku_ids.push(r);

        let amount: f64 = (rand::random::<u32>() % 1000) as f64 / 10.0;
        amounts.push(amount);
    }
    Orders{ order_id: order_ids, sku_id: sku_ids, amount: amounts }
}

fn filter_data(orders: &Orders) -> Orders {
    let mut result = Orders{ order_id: vec![], sku_id: vec![], amount: vec![] };
    for i in 0..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            result.order_id.push(orders.order_id[i]);
            result.sku_id.push(orders.sku_id[i]);
            result.amount.push(orders.amount[i]);
        }
    }
    result
}

fn filter_aggregate(orders: &Orders) -> HashMap<u32, (f64, u32)> {
    let mut result = HashMap::new();
    for i in 0..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            let entry = result.entry(orders.sku_id[i]).or_insert((0.0, 0));
            entry.0 += orders.amount[i];
            entry.1 += 1;
        }
    }
    result
}

unsafe fn filter_simd(orders: &Orders) -> Orders  {
    let mut result = Orders{ order_id: vec![], sku_id: vec![], amount: vec![] };
    let length = orders.order_id.len() & (!0x0F); // 32

    let a: u32x16 = u32x16::splat(1000);
    let b: f64x16 = f64x16::splat(20.0);
    let zero = u32x16::splat(0);

    for i in (0..length).step_by(16) {
        let sku_id: u32x16 = u32x16::from_slice(&orders.sku_id[i..]);
        let order_id: u32x16 = u32x16::from_slice(&orders.order_id[i..]);
        let amount: f64x16 = f64x16::from_slice(&orders.amount[i..]);

        // compute a mask of sku_id % 1000 == 0 && amount > 50.0
        let b1: mask8x16 = Mask::from((sku_id % a).simd_eq(zero));  // sku_id % 1000 == 0
        let b2 = Mask::from(amount.simd_gt(b));         // amount > 20.0
        let mut b = b1.bitand(b2);                      // combine the two masks

        result.order_id.reserve(16);
        result.sku_id.reserve(16);
        result.amount.reserve(16);

        while let Some(idx) = b.first_set() {
            result.order_id.extend_one_unchecked(order_id[idx]);
            result.sku_id.extend_one_unchecked(sku_id[idx]);
            result.amount.extend_one_unchecked(amount[idx]);
            b.set(idx, false);
        }
    }
    for i in length..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            result.order_id.push(orders.order_id[i]);
            result.sku_id.push(orders.sku_id[i]);
            result.amount.push(orders.amount[i]);
        }
    }
    result
}


unsafe fn filter_simd_aggregate(orders: &Orders) -> HashMap<u32, (f64, u32)>  {
    let mut ht = HashMap::new();
    let length = orders.order_id.len() & (!0x0F); // 32

    let a: u32x16 = u32x16::splat(1000);
    let b: f64x16 = f64x16::splat(20.0);
    let zero = u32x16::splat(0);

    for i in (0..length).step_by(16) {
        let sku_id: u32x16 = u32x16::from_slice(&orders.sku_id[i..]);
        let order_id: u32x16 = u32x16::from_slice(&orders.order_id[i..]);
        let amount: f64x16 = f64x16::from_slice(&orders.amount[i..]);

        // compute a mask of sku_id % 1000 == 0 && amount > 50.0
        let b1: mask8x16 = Mask::from((sku_id % a).simd_eq(zero));
        let b2 = Mask::from(amount.simd_gt(b));
        let mut b = b1.bitand(b2);

        while let Some(idx) = b.first_set() {
            let entry = ht.entry(sku_id[idx]).or_insert((0.0, 0));
            entry.0 += amount[idx];
            entry.1 += 1;
            b.set(idx, false);
        }
    }
    for i in length..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            let entry = ht.entry(orders.sku_id[i]).or_insert((0.0, 0));
            entry.0 += orders.amount[i];
            entry.1 += 1;
        }
    }
    ht
}

#[inline(never)]
fn group_data(orders: &Orders) -> std::collections::HashMap<u32, (f64, u32)> {
    assert!(orders.sku_id.len() == orders.order_id.len());
    assert!(orders.amount.len() == orders.order_id.len());
    let mut grouped = std::collections::HashMap::new();
    for i in 0..orders.order_id.len() {
        let sku_id = orders.sku_id[i];
        let amount = orders.amount[i];
        let entry = grouped.entry(sku_id).or_insert((0.0, 0));
        entry.0 += amount;
        entry.1 += 1;
    }

    grouped
}