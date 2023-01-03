use std::net::IpAddr;

pub fn panic_load() {
    test_test();
}

fn test_test() {
    // panic!("into panic");
    // println!("this is panic")
    let addr: IpAddr = "127.0.0.1".parse().expect("msg");

    println!("{addr}");
}
