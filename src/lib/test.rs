use crate::lib::example;
use std::{fs::File, io};

pub fn set_test() {
    example::example_load();
    let res = test1(&1);
    println!("{:?}", res);

    let res = test2(&1);
    println!("{:?}", res);

    let res = test3(&1);
    println!("{:?}", res.err())
}

pub fn set_test2() {
    println!("this is test2")
}

pub fn test1(arg: &u32) -> Result<u32, &'static str> {
    let error = "It didn’t work";
    if arg < &10 {
        Err(error)
    } else {
        Ok(1)
    }
}

pub fn test2(arg: &u32) -> Result<u32, String> {
    let error = String::from("this is err");
    if arg < &10 {
        Err(error)
    } else {
        Ok(1)
    }
}

pub struct Content {
    pub con: String,
}

pub fn test3(arg: &u32) -> io::Result<Content> {
    println!("{:?}", arg);
    File::open("path")?;
    let con = Content {
        con: String::from(""),
    };
    Ok(con)
}
