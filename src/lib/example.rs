fn get_all_ele() -> (i32, String) {
    (18, String::from("stb"))
}

pub fn get_element() {
    let (age, name) = get_all_ele();
    println!("{age},{name}")
}
