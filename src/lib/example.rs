pub mod string;
pub mod vec;

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

pub fn example_load() {
    get_all_ele();
    get_element();
    get_string();
}
