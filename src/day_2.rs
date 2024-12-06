pub(crate) fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let nums: Vec<u8> = line
                .split(" ")
                .map(|num| num.parse::<u8>().unwrap())
                .collect();

            let diff = nums
                .windows(2)
                .all(|n| 1 <= n[0].abs_diff(n[1]) && n[0].abs_diff(n[1]) <= 3);

            let inc = nums.windows(2).all(|n| n[0] < n[1]);
            let dec = nums.windows(2).all(|n| n[0] > n[1]);

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
                nums.windows(2)
                    .all(|n| 1 <= n[0].abs_diff(n[1]) && n[0].abs_diff(n[1]) <= 3)
            };

            let is_inc = |nums: &[u8]| nums.windows(2).all(|n| n[0] < n[1]);
            let is_dec = |nums: &[u8]| nums.windows(2).all(|n| n[0] > n[1]);

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
