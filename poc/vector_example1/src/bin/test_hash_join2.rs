#[derive(Debug)]
struct BuildTable {
    count: usize,

    // bucketV: Vec<usize>,
    // groupIdV: Vec<usize>,

    // first[N] = the first groupId in bucket N
    first: Vec<i64>,

    // next[X] = the next groupId in the same bucket, or -1 if it's the last
    // next.length = groupIdV.length
    next: Vec<i64>,
}

impl BuildTable {

    fn new(bucket_size: usize) -> BuildTable {
        BuildTable {
            count: 0,
            first: vec![-1; bucket_size],
            next: Vec::new(),
        }
    }

    fn insert(&mut self, bucketV: &Vec<usize>) {
        for i in 0..bucketV.len() {
            self.next.push( self.first[bucketV[i]] ); // self.next[groupId] = self.first[bucketV[i]];
            // self.next[groupId] = self.first[bucketV[i]];
            self.first[bucketV[i]] = self.count as i64;
            self.count += 1;
        }
    }

}

const SKU_COUNT: usize = 100;
const BUCKET_SIZE: usize = 16;
const PROBE_SIZE: usize = 200;

struct ProbeTable {
    probe_count: usize,

    // probe_bucketV.len() == probe_count
    probe_bucketV: Vec<usize>,

    // probe_groupIdV.len() == probe_count
    group_idV: Vec<usize>,
    to_checkV: Vec<usize>,
}

fn main(){

    let skus = prepare_sku_data();
    let orders = prepare_order_data();
    println!("{:?}", orders);


    let mut build_table = BuildTable::new(BUCKET_SIZE);
    let mut build_bucketV = Vec::with_capacity(skus.sku_id.len());
    for i in 0..skus.sku_id.len() {
        build_bucketV.push(skus.sku_id[i] as usize % BUCKET_SIZE);
    }
    build_table.insert(&build_bucketV);
    println!("build_table {:?}", build_table);

    let len1 = orders.order_id.len() & (!0x7FF); // 2048


    let mut probe_bucketV = [0; PROBE_SIZE];
    let mut probe_groupIdV = [0; PROBE_SIZE];
    let mut probe_to_checkV = vec![0; PROBE_SIZE];

    let i = 0;
    // for i in (0..len1).step_by(2048) {
        // initial phase
        for j in 0..PROBE_SIZE {
            probe_bucketV[j] = orders.sku_id[i+j] as usize % BUCKET_SIZE;
        }
        for j in 0..PROBE_SIZE {
            let bucket = probe_bucketV[j];
            probe_groupIdV[j] = build_table.first[bucket];
            probe_to_checkV[j] = j;
        }

        // probe phase
        loop {
            if probe_to_checkV.len() == 0 {
                break;
            }

            let toCheck1: &[usize] = &probe_to_checkV;
            println!("toCheck1: {} {:?}\n", toCheck1.len(), toCheck1);

            let mut j = 0;
            while j < probe_to_checkV.len() {
                let toCheck = probe_to_checkV[j];

                let probeGroupId = probe_groupIdV[toCheck];
                if probeGroupId == -1 {
                    j += 1;
                    continue;
                }
                let value = orders.sku_id[i+toCheck];;
                let probeValue = skus.sku_id[probeGroupId as usize];
                if value == probeValue {
                    // print!("check:{toCheck} order_id: {}, sku_id: {}, amount: {}", orders.order_id[i+toCheck], orders.sku_id[i+toCheck], orders.amount[i+toCheck]);
                    // println!(" <==> sku_id: {}, sku_name: {}", skus.sku_id[probeGroupId as usize], skus.sku_name[probeGroupId as usize]);
                    probe_to_checkV.remove(j);
                    continue;
                }
                else {
                    // print!("check:{toCheck} order_id: {}, sku_id: {}, amount: {}", orders.order_id[i+toCheck], orders.sku_id[i+toCheck], orders.amount[i+toCheck]);
                    // println!(" !==! sku_id: {}, sku_name: {}, try Next", skus.sku_id[probeGroupId as usize], skus.sku_name[probeGroupId as usize]);
                    probe_groupIdV[toCheck] = build_table.next[probeGroupId as usize];
                    j += 1;
                    continue;
                }
            }

            // let toCheck: &[usize] = &probe_to_checkV;
            // println!("toCheck: {:?}", toCheck);
        }

        // for j in 0..PROBE_SIZE {
        //     let groupId = probe_groupIdV[j];
        //     if groupId != 0 {
        //     }
        // }

        // break;
    // }
    // for i in len1..orders.order_id.len() {
    //
    // }

}


struct Skus {
    sku_id: Vec<u32>,
    sku_name: Vec<String>,
}
fn prepare_sku_data() -> Skus {
    let mut skus = Skus {
        sku_id: Vec::new(),
        sku_name: Vec::new(),
    };

    for i in 0..SKU_COUNT {
        let sku_id = i;
        let sku_name = format!("sku_{}", i);

        skus.sku_id.push(sku_id as u32);
        skus.sku_name.push(sku_name);
    }

    skus
}

#[derive(PartialEq, Debug)]
struct Orders {
    order_id: Vec<u32>,
    sku_id: Vec<u32>,
    amount: Vec<f64>,
}
fn prepare_order_data() -> Orders {
    let mut order_ids = vec![];
    let mut sku_ids = vec![];   // 10000 based
    let mut amounts = vec![];

    // for i in 0u32..1_000_000_000 {
    for i in 0u32..PROBE_SIZE as u32 {
        order_ids.push(i);
        // generate a random number between 0 and 10000
        let r1 = rand::random::<u32>() % SKU_COUNT as u32;
        sku_ids.push(r1);

        let amount: f64 = (rand::random::<u32>() % 1000 + 10) as f64 / 10.0;
        amounts.push(amount);
    }
    Orders{ order_id: order_ids, sku_id: sku_ids, amount: amounts }
}

