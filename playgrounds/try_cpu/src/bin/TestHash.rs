use std::collections::{BTreeMap, HashSet};
use std::hash::{DefaultHasher, Hasher};
use ahash::RandomState;
use rand::RngCore;

const ELEMENTS: u64 = 1 << 20;

fn main() {
    
    // generate 1M u64
    let mut rand = rand::rng();
    let nums = (0..ELEMENTS).map( |_| rand.next_u64()) .collect::<Vec<u64>>();  // ~ 30
    // let nums = (0..1 << 20).map( |i| i ) .collect::<Vec<u64>>();  // ~30

    build_hash_map(&nums);
}

struct HashEntry {
    h2:  u16,        // 0x00 - 0x7F, 0xFF for empty, 0xFE for others
    count: usize,    // {count} elements in this entry
}

struct MyHashMap<const SCALE_N: u64, const SCALE_M: u64, const GROUP_SIZE: u64, const H2_BITS: u64> {
    // groups: u64,
    // group_size: u64, // 16 or 32
    slots: Vec<HashEntry>,
    history: HashSet<u64>
}

impl <const SCALE_N: u64, const SCALE_M: u64, const GROUP_SIZE: u64, const H2_BITS: u64> MyHashMap<SCALE_N, SCALE_M, GROUP_SIZE, H2_BITS> {

    const GROUPS: u64 = (ELEMENTS as f64 * Self::SCALE) as u64 / Self::GROUP_SIZE;
    const SCALE: f64 = SCALE_N as f64 / SCALE_M as f64;
    const GROUP_SIZE: u64 = GROUP_SIZE;  // either 16/32

    // const H2_BITS: u64 = 15;
    // const H2_MASK: u16 = 0x8000;
    // const EMPTY_TAG: u16 = 0xFFFF;
    // const OTHER_TAG: u16 = 0xFFFE;

    const H2_BITS: u64 = H2_BITS; // either 7/15
    const H2_MASK: u16 = if Self::H2_BITS == 7 { 0x80 } else { 0x8000 };
    const EMPTY_TAG: u16 = if Self::H2_BITS == 7 { 0xFF } else { 0xFFFF };
    const OTHER_TAG: u16 = if Self::H2_BITS == 7 { 0xFE } else { 0xFFFE };

    fn new() -> MyHashMap<SCALE_N, SCALE_M, GROUP_SIZE, H2_BITS> {
        MyHashMap {
            slots: (0 .. Self::GROUPS * Self::GROUP_SIZE).map( |_| HashEntry{ h2: Self::EMPTY_TAG, count: 0}).collect(),
            history: HashSet::new()
        }
    }

    fn put(&mut self, value:u64) {
        if self.history.contains(&value) {
            return;
        }
        else {
            self.history.insert(value);

            // let mut hasher = DefaultHasher::new();
            // hasher.write_u64(value);
            // let hash_code = hasher.finish();
            // let random_state = RandomState::with_seeds(0, 0, 0, 0);
            let random_state = RandomState::new();
            let hash_code = random_state.hash_one(value);

            let group_no = hash_code % Self::GROUPS;
            let h2 = (hash_code >> (64 - Self::H2_BITS)) as u16;

            let from = group_no * Self::GROUP_SIZE;
            let end = (group_no + 1) * Self::GROUP_SIZE;

            let group_slots = &mut self.slots[from as usize..end as usize];
            for (seq, entry) in group_slots.iter_mut().enumerate() {
                if entry.h2 == Self::EMPTY_TAG {
                    if seq < Self::GROUP_SIZE as usize - 1 {
                        entry.h2 = h2;
                    } else {
                        entry.h2 = Self::OTHER_TAG;
                    }
                    entry.count = 1;
                    break;
                } else if entry.h2 == h2 {
                    entry.count += 1;
                    break;
                } else if entry.h2 == Self::OTHER_TAG {
                    entry.count += 1;
                }
            }
        }

    }

    fn run_test(numbers: &[u64]) {
        let mut ht = Self::new();

        numbers.iter().for_each( |&num| { ht.put(num)} );

        let mut dupicated_entries = 0;
        let mut dupicated_count = 0;
        let mut other_entries = 0;
        let mut other_count = 0;
        let mut total = 0;
        let unique = ht.history.len();
        ht.slots.iter().for_each( |entry| {
            total += entry.count;
            if entry.h2 < Self::H2_MASK && entry.count > 1 {
                dupicated_entries += 1;
                dupicated_count += entry.count;
            }
            else if entry.h2 == Self::OTHER_TAG {
                other_entries += 1;
                other_count+= entry.count;
            }
        });

        let scale = Self::SCALE;
        let h2_bits = Self::H2_BITS;
        let group_size = Self::GROUP_SIZE;
        let dupicated_rate = dupicated_count as f64 / total as f64;
        let other_rate = other_count as f64 / total as f64;
        println!("setting:{scale}-{group_size}-{h2_bits} total: {total}, unique: {unique}, dupicated_entries: {dupicated_entries}, other_entries: {other_entries}, \
        dupicated_count: {dupicated_count}:{dupicated_rate}, other_count: {other_count}:{other_rate}");

    }
}

fn build_hash_map(numbers: &[u64]){

    MyHashMap::<1, 1, 16, 7>::run_test(numbers);
    // MyHashMap::<3, 2, 16, 7>::run_test(numbers);
    MyHashMap::<2, 1, 16, 7>::run_test(numbers);
    MyHashMap::<3, 1, 16, 7>::run_test(numbers);
    MyHashMap::<4, 1, 16, 7>::run_test(numbers);
    println!();

    MyHashMap::<1, 1, 32, 7>::run_test(numbers);
    MyHashMap::<2, 1, 32, 7>::run_test(numbers);
    MyHashMap::<3, 1, 32, 7>::run_test(numbers);
    MyHashMap::<4, 1, 32, 7>::run_test(numbers);
    println!();

    MyHashMap::<1, 1, 16, 15>::run_test(numbers);  // better choice 0.2% has next
    // MyHashMap::<3, 2, 16, 15>::run_test(numbers);  // better choice 0.2% has next
    MyHashMap::<2, 1, 16, 15>::run_test(numbers);  // better choice 0.2% has next
    MyHashMap::<3, 1, 16, 15>::run_test(numbers);
    MyHashMap::<4, 1, 16, 15>::run_test(numbers);  // best choice 0.01% has next
    println!();

    MyHashMap::<1, 1, 32, 15>::run_test(numbers);
    MyHashMap::<2, 1, 32, 15>::run_test(numbers);
    MyHashMap::<3, 1, 32, 15>::run_test(numbers);
    MyHashMap::<4, 1, 32, 15>::run_test(numbers);

}
