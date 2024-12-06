use std::collections::HashMap;

pub(crate) fn part_one(input: &str) -> usize {
    let (rules, orders) = input.split_once("\n\n").unwrap();

    let mut rules_hmap: HashMap<u8, Vec<u8>> = HashMap::new();
    for line in rules.lines() {
        let (f, s) = line.split_once('|').unwrap();
        let key = s.parse::<u8>().unwrap();
        let val = f.parse::<u8>().unwrap();

        rules_hmap.entry(key).or_default().push(val);
    }

    orders
        .lines()
        .filter_map(|line| {
            let nums: Vec<u8> = line.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
            let l = nums.len();
            let mut valid = true;

            for i in 0..l - 1 {
                let sec = nums[i];

                if let Some(first) = rules_hmap.get(&sec) {
                    if first.iter().any(|f| nums[i + 1..].contains(f)) {
                        valid = false;
                        break;
                    }
                }
            }

            valid.then_some(nums[l / 2] as usize)
        })
        .sum()
}

pub(crate) fn part_two(input: &str) -> usize {
    let (rules, orders) = input.split_once("\n\n").unwrap();

    let mut rules_hmap: HashMap<u8, Vec<u8>> = HashMap::new();
    for line in rules.lines() {
        let (f, s) = line.split_once('|').unwrap();
        let key = s.parse::<u8>().unwrap();
        let val = f.parse::<u8>().unwrap();

        rules_hmap.entry(key).or_default().push(val);
    }

    orders
        .lines()
        .filter_map(|line| {
            let mut nums: Vec<u8> = line.split(',').map(|n| n.parse::<u8>().unwrap()).collect();
            let l = nums.len();
            let mut invalid = false;
            let mut i = 0;

            while i < l - 1 {
                let sec = nums[i];

                if let Some(first) = rules_hmap.get(&sec) {
                    let mut swapped = false;

                    for &f in first {
                        for j in (i + 1)..l {
                            if f == nums[j] {
                                nums.swap(i, j);
                                swapped = true;
                                invalid = true;
                                break;
                            }
                        }
                    }

                    if swapped {
                        continue;
                    }
                }

                i += 1;
            }

            invalid.then_some(nums[l / 2] as usize)
        })
        .sum()
}
