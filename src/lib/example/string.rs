use std::ops::Add;

pub fn set_string() {
    let str = "aabbccddeeffgg";

    let res = &str.to_string();

    for (i, &item) in res.as_bytes().iter().enumerate() {
        println!("{:?}-{:?}", i, item)
    }

    let str1 = String::from("aa");
    let str2 = String::from("bb");
    let str3 = String::from("cc");
    // let result = str1 + &str2 + &str3;
    let res = str1.add(&str2).add(&str3);
    println!("res3:{res}")
}
