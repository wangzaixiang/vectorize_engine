#![feature(portable_simd)]
#![feature(extend_one_unchecked)]
#![feature(allocator_api)]

use std::collections::HashMap;
/// example1 demonstrates the use of:
/// 1. flat vectors
/// 2. do some filters
/// 3. do hash group and aggregations
///

use std::f64;
use std::ops::BitAnd;
use std::simd::{f64x16, f64x8, mask8x16, u32x16, u32x8, Mask};
use std::simd::cmp::{SimdPartialEq, SimdPartialOrd};
use std::simd::num::SimdFloat;
use rand::random;
use vector_example1::sort2::sort_u32x8;
use vector_example1::timeit;
// mod vector;


#[derive(PartialEq)]
struct Orders {
    order_id: Vec<u32>,
    sku_id: Vec<u32>,
    amount: Vec<f64>,
}


fn main(){
    let rand_nums: Vec<u32> = (0..8).map( |_ | random::<u32>() ).collect();

    let mut vec = u32x8::from_slice(&rand_nums);
    let (_, time1) =timeit("sort_u32x8", || for i in 0..1_000_000 {
            vec[0] = i;
            sort_u32x8(&mut vec);
         }
    );

    let mut vec2 = Vec::with_capacity(8);
    let (_, time2) = timeit("simple sort", || for i in 0..1_000_000 {
        vec2.clear();
        vec2.extend_from_slice(&rand_nums);
        vec2[0] = i;
        vec2.sort();
    });

    println!("time1 = {:?} time2 = {:?} vec = {:?}, vec2 = {:?}", time1, time2, vec, vec2);

}

// 目前来看，group-aggregation 未能有效加速
// TODO 待测试: Hash-Join SIMD 加速
// TODO 待测试：Sorted-Join SIMD 加速
// TODO 排序加速
fn main0() {

    // prepare 1B records
    let (orders, _) = timeit("prepare_data", || prepare_data() );

    for _ in 0..16 {

        // Test1: filter_data |> group-by
        let mut filtered = Orders{ order_id: vec![], sku_id: vec![], amount: vec![] };
        let (_, filter_data_0_time) = timeit("filter_data_0", || filter_data(&orders, &mut |order_id, sku_id, amount| {
            filtered.order_id.push(order_id);
            filtered.sku_id.push(sku_id);
            filtered.amount.push(amount);
        }));
        let filtered = filtered;
        let (expected, group_by_time) = timeit("group-by", || group_data(&filtered));
        println!("filter_data_0: {}ms, grouped: {}ms, total is {}ms", filter_data_0_time.as_millis(), group_by_time.as_millis(), filter_data_0_time.as_millis() + group_by_time.as_millis());


        // Test2: bind filter + group-by
        {
            let label = "bind filter and group-by";
            let mut ht = HashMap::new();
            let mut each = |_order_id: u32, sku_id: u32, amount: f64| {
                let entry = ht.entry(sku_id).or_insert((0.0, 0));
                entry.0 += amount;
                entry.1 += 1;
            };
            let (_, filter_data_0_time) = timeit(label, || filter_data(&orders, &mut each));

            if ht != expected {
                println!("expect: {:?}", expected);
                println!("{label} result: {:?}", ht);
                panic!("expect != result");
            }
            println!("{label} {}ms", filter_data_0_time.as_millis());
        }

        // Test3: filter_simd |> group-by
        {
            let label = "filter_simd |> group-by";
            let mut filtered = Orders{ order_id: vec![], sku_id: vec![], amount: vec![] };
            let mut each = |order_id: u32, sku_id: u32, amount: f64| {
                filtered.order_id.push(order_id);
                filtered.sku_id.push(sku_id);
                filtered.amount.push(amount);
            };
            let (_, filter_simd_time) = timeit(label, || unsafe { filter_simd(&orders, &mut each) });
            let (result, group_by_time) = timeit(label, || group_data(&filtered));

            if result != expected {
                println!("expect: {:?}", expected);
                println!("${label} result: {:?}", result);
                panic!("expect != result");
            }
            println!("filter_simd: {}ms, grouped: {}ms, total is {}ms", filter_simd_time.as_millis(), group_by_time.as_millis(), filter_simd_time.as_millis() + group_by_time.as_millis());
        }

        // Test4: bind filter_simd + group-by
        {
            let label = "bind filter_simd and group-by";
            let mut ht = HashMap::new();
            let mut each = |_order_id: u32, sku_id: u32, amount: f64| {
                let entry = ht.entry(sku_id).or_insert((0.0, 0));
                entry.0 += amount;
                entry.1 += 1;
            };
            let (_, total_time) = timeit("filter_simd_0", || unsafe { filter_simd(&orders, &mut each) });

            if ht != expected {
                println!("expect: {:?}", expected);
                println!("{label} result: {:?}", ht);
                panic!("expect != result");
            }
            println!("{label} {}ms", total_time.as_millis());
        }

        // Test5: group-by

        let label = "group-by only";
        let (grouped_result, group_by_time) = timeit(label, || group_data(&orders));
        println!("{label} {}ms", group_by_time.as_millis());


        // Test6: group_data_using_array
        {
            let label = "sorted-group-by";
            let (result, group_by_time) = timeit(label, ||  group_data_using_array(&orders) ) ;

            if result != grouped_result {
                println!("expect: {:?}", grouped_result);
                println!("{label} result: {:?}", result);
                panic!("expect != result");
            }

            println!("{label} {}ms", group_by_time.as_millis());
        }

        // Test 7: aggregate_data
        {
            let label = "aggregate_data";
            let (result, group_by_time) = timeit(label, || aggregate_data(&orders) ) ;
            println!("{label} {}ms, result = ({:?})", group_by_time.as_millis(), result);
        }

        // Test 8: aggregate_data_simd
        {
            let label = "aggregate_data_simd";
            let (result, group_by_time) = timeit(label, || aggregate_data_simd(&orders) ) ;
            println!("{label} {}ms, result = ({:?})", group_by_time.as_millis(), result);
        }

        println!()

    }


}

// order_id, sku_id, amount
fn prepare_data() -> Orders {
    let mut order_ids = vec![];
    let mut sku_ids = vec![];   // 10000 based
    let mut amounts = vec![];

    for i in 0u32..1_000_000_000 {
    // for i in 0u32..10_000_000 {
        order_ids.push(i);
        // generate a random number between 0 and 10000
        let r1 = rand::random::<u32>() % 100;
        let r2 = rand::random::<u32>() % 10000;
        let r = if r1 < 90 { r1 } else { r2 };
        sku_ids.push(r);

        let amount: f64 = (rand::random::<u32>() % 1000 + 10) as f64 / 10.0;
        amounts.push(amount);
    }
    Orders{ order_id: order_ids, sku_id: sku_ids, amount: amounts }
}

// 1B 记录，filter 进行数据复制时，耗时 1360ms + 独立 group-aggregation 61ms = 1422ms
// 1B 记录，filter 进行流式处理时，耗时 1405 (附加了 group-aggregation 操作)
fn filter_data<F>(orders: &Orders, f: &mut F) where F: FnMut(u32, u32, f64) -> () {
    for i in 0..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            f(orders.order_id[i], orders.sku_id[i], orders.amount[i]);
        }
    }
}

// 1B 记录，filter 进行数据复制时，耗时 530ms + 独立 group-aggregation 61ms = 591ms
// 1B 记录，filter 进行流式处理时，耗时 495ms (附加了 group-aggregation 操作), filter 部分预计提升3倍+
unsafe fn filter_simd<F>(orders: &Orders, f: &mut F) where F: FnMut(u32, u32, f64) -> () {
    let length = orders.order_id.len() & (!0x0F); // 16

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

        while let Some(idx) = b.first_set() {                // 在 x86 上等效于 tzcnt 指令
            f(order_id[idx], sku_id[idx], amount[idx]);
            b.set(idx, false);
        }
    }
    for i in length..orders.order_id.len() {
        if orders.sku_id[i] % 1000 == 0 && orders.amount[i] > 20.0 {
            f(orders.order_id[i], orders.sku_id[i], orders.amount[i]);
        }
    }
}

// 基于 hashmap 的 group-by，在 1B 记录上，耗时为 7.4s，比 array 版本慢了 7倍。
fn group_data(orders: &Orders) -> std::collections::HashMap<u32, (f64, u32)> {
    assert!(orders.sku_id.len() == orders.order_id.len());
    assert!(orders.amount.len() == orders.order_id.len());
    let mut grouped = HashMap::new();
    for i in 0..orders.order_id.len() {
        let sku_id = orders.sku_id[i];
        let amount = orders.amount[i];
        let entry = grouped.entry(sku_id) // // 绝大部分时间都消耗在这里
            .or_insert((0.0, 0));
        entry.0 += amount;
        entry.1 += 1;
    }

    grouped
}

// group-by 的性能基准，直接使用硬编码的 array 替代 hashmap, 在 1B 数据上，耗时为 1.01s
fn group_data_using_array(orders: &Orders) -> std::collections::HashMap<u32, (f64, u32)> {
    assert!(orders.sku_id.len() == orders.order_id.len());
    assert!(orders.amount.len() == orders.order_id.len());

    struct Record {
        sku_id: u32,
        amount: f64,
        count: u32,
    }
    let mut records: Vec<Record> = Vec::with_capacity_in(10000, std::alloc::Global);
    for i in 0..10000 {
        records.push(Record{ sku_id: i, amount: 0.0, count: 0 });
    }

    for i in 0..orders.order_id.len() {
        let sku_id = orders.sku_id[i];
        let amount = orders.amount[i];
        records[sku_id as usize].amount += amount;
        records[sku_id as usize].count += 1;
    }

    let mut ht = HashMap::new();
    for i in 0..10000 {
        if records[i].count > 0 {
            ht.insert(i as u32, (records[i].amount, records[i].count));
        }
    }
    ht
}


// 未能有效向量化，1B 记录，耗时 937ms 速度比 simd 版本(260ms) 慢了 3.6 倍 M1Max
fn aggregate_data(orders: &Orders) -> (f64, u32) {
    let mut total_amount = 0.0;
    let mut count = 0;
    for i in 0..orders.order_id.len() {
        total_amount += orders.amount[i];
        count += 1;
    }
    (total_amount, count)
}


// 1G 数据，耗时 260ms
fn aggregate_data_simd(orders: &Orders) -> (f64, u32) {
    let mut total_amount = 0.0;
    let mut count = 0;

    let length = orders.order_id.len() & (!0x0F); // 16 is better than 32, same as 8
    for i in (0..length).step_by(16) {
        let amount= f64x16::from_slice(&orders.amount[i..]);
        let zero = f64x16::splat(0.0);
        total_amount += amount.reduce_sum();        // x86 上 reduce_sum 不支持向量化，还是多次累加
        count += amount.simd_ne(zero).to_bitmask().count_ones(); // x86 有 popcnt 指令
    }

    for i in length..orders.order_id.len() {
        total_amount += orders.amount[i];
        count += 1;
    }
    (total_amount, count)
}