pub(crate) fn part_one(input: &str) -> u32 {
    let ch: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    for i in 0..ch.len() {
        for j in 0..ch[i].len() {
            for &(di, dj) in &directions {
                let int_i = i as isize;
                let int_j = j as isize;

                if ch[i].get(j) == Some(&'X')
                    && ch
                        .get((int_i + di) as usize)
                        .and_then(|row| row.get((int_j + dj) as usize))
                        == Some(&'M')
                    && ch
                        .get((int_i + di * 2) as usize)
                        .and_then(|row| row.get((int_j + dj * 2) as usize))
                        == Some(&'A')
                    && ch
                        .get((int_i + di * 3) as usize)
                        .and_then(|row| row.get((int_j + dj * 3) as usize))
                        == Some(&'S')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

pub(crate) fn part_two(input: &str) -> u32 {
    let ch: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    for i in 0..ch.len() {
        for j in 0..ch[i].len() {
            if ch[i][j] == 'A' {
                let int_i = i as isize;
                let int_j = j as isize;

                let opps = [
                    ((int_i - 1, int_j - 1), (int_i + 1, int_j + 1)),
                    ((int_i - 1, int_j + 1), (int_i + 1, int_j - 1)),
                ];

                let mut valid = true;
                for &((x1, y1), (x2, y2)) in &opps {
                    let c1 = ch.get(x1 as usize).and_then(|row| row.get(y1 as usize));
                    let c2 = ch.get(x2 as usize).and_then(|row| row.get(y2 as usize));

                    if !(c1 == Some(&'M') && c2 == Some(&'S')
                        || c1 == Some(&'S') && c2 == Some(&'M'))
                    {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}
