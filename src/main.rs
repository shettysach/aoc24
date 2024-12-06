mod day_6;
use day_6::*;

fn main() {
    let example = include_str!("../data/day_6/example.txt");
    let data = include_str!("../data/day_6/data.txt");

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
