use std::ops::Add;

pub fn set_string() {
    let str = "aabbccddeeffgg";

    let res = &str.to_string();

    for (i, &item) in res.as_bytes().iter().enumerate() {
        println!("{:?}-{:?}-{:?}", i, item, &item.to_string())
    }

    let str1 = String::from("aa");
    let str2 = String::from("bb");
    let str3 = String::from("cc");
    // let result = str1 + &str2 + &str3;
    let mut res = str1.add(&str2).add(&str3);
    println!("res3:{res}");
    res.push_str("last one");
    println!("last res :{res}");

    let str_list = "aabbccdd";
    let vn = &str_list[1..4];
    println!("vn:{:?}", vn)
}
