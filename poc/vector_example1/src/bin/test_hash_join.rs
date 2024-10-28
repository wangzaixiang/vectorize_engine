use std::collections::HashMap;
use vector_example1::timeit;

// select sku.sku_name, sum(order.amount) from
// order join sku on order.sku_id = sku.sku_id
fn main() {

    let skus = prepare_sku_data();
    let orders = prepare_data();

    let mut ht = HashMap::new();
    for i in 0..skus.sku_id.len() {
        ht.insert(skus.sku_id[i], skus.sku_name[i].clone());
    }

    let (grouped, time) =
        timeit("hash_join", || group_data(&orders, &ht) );

    let mut sorted: Vec<(&&str, &f64)> = grouped.iter().collect();
        sorted.sort_by(|a, b| a.0.cmp(b.0));

    println!("hash-join-hash-aggregate: {:?}", time);
    // println!("{:?}", sorted);
}

struct Skus {
    sku_id: Vec<u32>,
    sku_name: Vec<String>,
}
pub fn prepare_sku_data() -> Skus {
    let mut skus = Skus {
        sku_id: Vec::new(),
        sku_name: Vec::new(),
    };

    for i in 0..10_000 {
        let sku_id = i;
        let sku_name = format!("sku_{}", i);

        skus.sku_id.push(sku_id);
        skus.sku_name.push(sku_name);
    }

    skus
}

#[derive(PartialEq)]
struct Orders {
    order_id: Vec<u32>,
    sku_id: Vec<u32>,
    amount: Vec<f64>,
}
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

// 1. replace HashMap<String, f64> with HashMap<&str, f64> to avoid memory allocation, from 54s to 35s.
fn group_data<'a>(orders: &Orders, ht: &'a HashMap<u32, String>) -> HashMap<&'a str, f64> {
    let mut grouped: HashMap<&str, f64> = HashMap::new();
    for i in 0..orders.order_id.len() {
        let sku_id = orders.sku_id[i];
        let amount = orders.amount[i];

        let sku_name = ht.get(&sku_id).unwrap();
        let entry = grouped.entry(sku_name).or_insert(0.0);
        *entry += amount;
    }
    grouped
}