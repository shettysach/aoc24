use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn step(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::N => (pos.0 - 1, pos.1),
            Dir::S => (pos.0 + 1, pos.1),
            Dir::E => (pos.0, pos.1 - 1),
            Dir::W => (pos.0, pos.1 + 1),
        }
    }
    fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }
}

fn grid_walk(grid: &[Vec<char>], start: (usize, usize), direction: Dir) -> Option<usize> {
    let mut pos = start;
    let mut dir = direction;
    let mut vis_pos = HashSet::from([start]); // Needed for part 1
    let mut vis_pos_dir = HashSet::from([(start, dir)]); // Needed for part 2

    // Can be made faster by splitting into 2 fns for the 2 parts

    loop {
        let new_pos = dir.step(pos);

        if let Some(&char) = grid.get(new_pos.0).and_then(|row| row.get(new_pos.1)) {
            if vis_pos_dir.contains(&(new_pos, dir)) {
                return None;
            }

            if char == '#' {
                dir = dir.turn_right();
            } else {
                pos = new_pos;
            }

            vis_pos.insert(pos);
            vis_pos_dir.insert((pos, dir));
        } else {
            break;
        }
    }

    Some(vis_pos.len())
}

pub(crate) fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find_map(|(col, &ch)| (ch == '^').then_some((row, col)))
        })
        .unwrap();

    grid_walk(&grid, start, Dir::N).unwrap()
}

pub(crate) fn part_two(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .enumerate()
                .find_map(|(col, &ch)| (ch == '^').then_some((row, col)))
        })
        .unwrap();

    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch != '#' && ch != '^' {
                let mut new_grid = grid.clone();
                new_grid[i][j] = '#';

                if grid_walk(&new_grid, start, Dir::N).is_none() {
                    count += 1;
                }
            }
        }
    }

    count
}
