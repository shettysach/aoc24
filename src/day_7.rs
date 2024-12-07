use std::iter::once;

fn perms(nums: &[usize], curr: usize) -> Box<dyn Iterator<Item = usize>> {
    if nums.is_empty() {
        return Box::new(once(curr));
    }

    let next = nums[0];
    let rest = &nums[1..];

    let adds = perms_concat(rest, curr + next);
    let muls = perms_concat(rest, curr * next);

    Box::new(adds.chain(muls))
}

pub(crate) fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (res, nums) = line.split_once(':')?;
            let res = res.parse::<usize>().ok()?;
            let nums: Vec<usize> = nums
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let first = nums[0];
            let rest = &nums[1..];

            perms(rest, first).any(|perm| perm == res).then_some(res)
        })
        .sum()
}

fn perms_concat(nums: &[usize], curr: usize) -> Box<dyn Iterator<Item = usize>> {
    if nums.is_empty() {
        return Box::new(once(curr));
    }

    let next = nums[0];
    let rest = &nums[1..];
    let conc = format!("{}{}", curr, next).parse::<usize>().unwrap();

    let adds = perms_concat(rest, curr + next);
    let muls = perms_concat(rest, curr * next);
    let cats = perms_concat(rest, conc);

    Box::new(adds.chain(muls).chain(cats))
}

pub(crate) fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (res, nums) = line.split_once(':')?;
            let res = res.parse::<usize>().ok()?;
            let nums: Vec<usize> = nums
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let first = nums[0];
            let rest = &nums[1..];

            perms_concat(rest, first)
                .any(|perm| perm == res)
                .then_some(res)
        })
        .sum()
}
