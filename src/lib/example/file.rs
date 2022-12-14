use std::{fs::File, io::ErrorKind};

pub fn load() {
    open_file();
    open_file_result();
}

fn open_file() {
    let result = File::open("./aa.txt");

    let f = match result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./aa.txt") {
                Ok(file) => file,
                Err(err) => panic!("{err}"),
            },
            other_error => {
                panic!("{other_error}")
            }
        },
    };
}

fn open_file_result() {
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
}
