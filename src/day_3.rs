pub(crate) fn part_one(input: &str) -> u32 {
    input
        .split("mul(")
        .filter_map(|m| {
            let mut nums = m.split(',');

            nums.next()
                .and_then(|f| f.parse::<u32>().ok())
                .and_then(|f| {
                    nums.next()
                        .and_then(|s| s.split(')').next())
                        .and_then(|s| s.parse::<u32>().ok())
                        .map(|s| f * s)
                })
        })
        .sum()
}

pub(crate) fn part_two(input: &str) -> u32 {
    let mut input = input.to_string();

    while let Some(start) = input.find("don't()") {
        if let Some(do_pos) = input[start..].find("do()") {
            let end = start + do_pos + 4;
            input.replace_range(start..end, "");
        } else {
            break;
        }
    }

    part_one(&input)
}
