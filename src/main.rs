mod day_7;
use day_7::*;

fn main() {
    let example = include_str!("../data/day_7/example.txt");
    let data = include_str!("../data/day_7/data.txt");

    let e_one = part_one(example);
    println!("{e_one}");

    let d_one = part_one(data);
    println!("{d_one}");

    println!();

    let e_two = part_two(example);
    println!("{e_two}");

    let d_two = part_two(data);
    println!("{d_two}");
}
