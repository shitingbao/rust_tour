// use crate::garden::vegetables::Asparagus;

pub mod lib; // 下级包，有 mod 声明

mod core; // 同级别引用

fn main() {
    lib::client::mysql::connect_mysql();
    lib::client::pg::connect_pg();

    lib::garden::get_vegetablees();

    lib::test::set_test();

    lib::example::vec::example_vec();

    core::get_cors();
}
