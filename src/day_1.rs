pub(crate) fn part_one(input: &str) -> u32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let (f, s) = line.split_once("   ").unwrap();
            (f.parse::<i32>().unwrap(), s.parse::<i32>().unwrap())
        })
        .collect();

    first.sort_unstable();
    second.sort_unstable();

    first.iter().zip(second).map(|(f, s)| f.abs_diff(s)).sum()
}

pub(crate) fn part_two(input: &str) -> usize {
    let (first, second): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let (f, s) = line.split_once("   ").unwrap();
            (f.parse::<i32>().unwrap(), s.parse::<i32>().unwrap())
        })
        .collect();

    first
        .iter()
        .map(|f| *f as usize * second.iter().filter(|&s| s == f).count())
        .sum()
}
