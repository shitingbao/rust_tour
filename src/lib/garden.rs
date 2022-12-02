use crate::lib::test::set_test2;

pub mod canl;
pub mod tomato;
pub mod vegetables;

pub fn get_vegetablees() {
    println!("get vegetables");

    // 同级别的引用
    set_test2();
}
