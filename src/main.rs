mod day_2;
use day_2::*;

fn main() {
    let example = include_str!("../data/day_2/example.txt");
    let data = include_str!("../data/day_2/data.txt");

    let ea = part_one(example);
    println!("{ea}");

    let ta = part_one(data);
    println!("{ta}");

    println!();

    let eb = part_two(example);
    println!("{eb}");

    let rb = part_two(data);
    println!("{rb}");
}
