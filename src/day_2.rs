use itertools::Itertools;

pub(crate) fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u8> = line
                .split(" ")
                .map(|num| num.parse::<u8>().unwrap())
                .collect();

            let diff = nums
                .iter()
                .tuple_windows()
                .all(|(a, b)| 1 <= a.abs_diff(*b) && a.abs_diff(*b) <= 3);

            let inc = nums.iter().tuple_windows().all(|(a, b)| a < b);
            let dec = nums.iter().tuple_windows().all(|(a, b)| a > b);

            diff && (inc || dec)
        })
        .count()
}

pub(crate) fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u8> = line
                .split(" ")
                .map(|num| num.parse::<u8>().unwrap())
                .collect();

            let diff = |nums: &[u8]| {
                nums.iter()
                    .tuple_windows()
                    .all(|(&a, &b)| 1 <= a.abs_diff(b) && a.abs_diff(b) <= 3)
            };

            let is_inc = |nums: &[u8]| nums.iter().tuple_windows().all(|(a, b)| a < b);
            let is_dec = |nums: &[u8]| nums.iter().tuple_windows().all(|(a, b)| a > b);

            let condn1 = diff(&nums) && (is_inc(&nums) || is_dec(&nums));

            let condn2 = (0..nums.len()).any(|i| {
                let mut news = nums.clone();
                news.remove(i);
                diff(&news) && (is_inc(&news) || is_dec(&news))
            });

            condn1 || condn2
        })
        .count()
}
