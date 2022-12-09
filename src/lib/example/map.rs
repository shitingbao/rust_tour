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
    let mut mp = HashMap::new();
    let test = String::from("a a bb cd ");
    for item in test.split_whitespace() {
        println!("val:{item}");
        let count = mp.entry(item).or_insert(0);
        *count += 1
    }
    println!("{:?}", mp)
}
