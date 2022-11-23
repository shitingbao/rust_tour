// use crate::garden::vegetables::Asparagus;

pub mod lib;

fn main() {
    lib::client::mysql::connect_mysql();
    lib::client::pg::connect_pg();

    lib::garden::get_vegetablees();

    lib::test::set_test();
}
