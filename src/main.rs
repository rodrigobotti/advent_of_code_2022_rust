mod day_1;
mod day_2;
mod day_3;
mod utils;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("Advent of code 2022");
    utils::print_separator();

    day_1::solution_part_1();
    day_1::soluction_iterator_part_1();
    day_1::soluction_iterator_part_2();

    utils::print_separator();

    day_2::solution_part_1();
    day_2::solution_part_2();

    utils::print_separator();

    day_3::solution_part_1();
    day_3::solution_part_2();
}
