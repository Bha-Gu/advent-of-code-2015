mod day1;
mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
mod template;

use day2::{INPUT, p1, p2};

fn main() {
    let out = p1(INPUT);
    println!("{out}");

    let out = p2(INPUT);
    println!("{out}");
}
