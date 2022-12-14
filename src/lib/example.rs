pub mod file;
pub mod map;
pub mod panic;
pub mod string;
pub mod vec;

pub fn example_load() {
    get_all_ele();
    get_element();
    get_string();
    map::map_load();
    panic::panic_load();
    file::load();
}

fn get_all_ele() -> (i32, String) {
    (18, String::from("stb"))
}

fn get_element() {
    let (age, name) = get_all_ele();
    println!("{age},{name}")
}

fn get_string() {
    string::set_string();
}

pub fn cyc() {
    let list = [1, 2, 3];
    for v in list {
        println!("{v}")
    }
}
