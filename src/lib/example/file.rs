use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

pub fn load() {
    open_file();
    open_file_result();
    let file_res = open_file_opera();
    println!("{:?}", file_res);
    let file_res = read_from_file();
    println!("your result is :{:?}", file_res);
}

fn open_file() {
    let result = File::open("./aa.txt");
    match result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./aa.txt") {
                Ok(file) => file,
                Err(err) => panic!("create err{:?}", err),
            },
            other_error => {
                panic!("err:{:?}", other_error)
            }
        },
    };
}

fn open_file_result() -> File {
    let f = File::open("./bb.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("bb.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            });
            panic!("no file");
        } else {
            panic!("open file error");
        }
    });
    return f;
}

fn open_file_opera() -> Result<String, io::Error> {
    let mut filename_res = match File::open("./aa.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut filename = String::new();
    match filename_res.read_to_string(&mut filename) {
        Ok(_) => Ok(filename),
        Err(e) => Err(e),
    }
}

fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("./aa.txt")?;
    let mut username = String::new();
    f.read_to_string(&mut username)?;
    Ok(username)
}
