use std::collections::HashMap;
use std::fmt::format;

fn main(){

    let mut ht = HashMap::with_capacity(100);

    for i in 0..10_000 {
        let value = format!("str{i}");
        ht.insert(i, value);
    }

    let str1 = ht.get(&10)
        .unwrap();
    let str2 = ht.get(&20)
        .unwrap();
    let str3 = ht.get(&30)
        .unwrap();

    println!("str1: {:?}", str1);
    println!("str2: {:?}", str2);
    println!("str3: {:?}", str3);

}