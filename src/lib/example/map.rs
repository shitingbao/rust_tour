use std::collections::HashMap;

pub fn map_load() {
    test_map();
    test_list();
}

pub fn test_map() {
    let mut mp = HashMap::new();

    mp.insert("aa", 111);
    mp.insert("bb", 1222);

    mp.entry("aa").or_insert(12000);
    let res = mp.get("aa").copied().unwrap_or(0);
    println!("{res}");

    let val = mp["aa"];
    println!("val===:{val}");

    for (i, v) in mp {
        println!("mp:{i}-{v}")
    }
}

pub fn test_list() {
    let test = String::from("a a bb cd ");
    let list = test.split(" ");
    println!("{test}t");
    for item in list {
        println!("val:{item}");
    }
}
