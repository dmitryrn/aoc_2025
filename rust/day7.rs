use std::collections::{HashSet, VecDeque};

const TEST_INPUT: &str = ".......S.......
.......|.......
......|^|......
......|.|......
.....|^|^|.....
.....|.|.|.....
....|^|^|^|....
....|.|.|.|....
...|^|^|||^|...
...|.|.|||.|...
..|^|^|||^|^|..
..|.|.|||.|.|..
.|^|||^||.||^|.
.|.|||.||.||.|.
|^|^|^|^|^|||^|
|.|.|.|.|.|||.|";

fn part_one(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut debug_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut splitters: Vec<Vec<usize>> = vec![vec![]; width];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '^' {
                splitters[x].push(y);
            }
        }
    }

    let mut deq = VecDeque::from([(0, 0)]);

    let start_x = input.find('S').unwrap();
    deq.push_back((start_x, 0));
    let mut used_splitters = 0;
    let mut starting_positions = HashSet::new();
    let mut found_splitters = HashSet::new();

    while !deq.is_empty() {
        let pos = deq.pop_front().unwrap();
        let splitter_pos = cast(pos, &splitters[pos.0]);
        if let Some(splitter_pos) = splitter_pos {
            let spl_pos = splitter_pos;
            if !found_splitters.insert(spl_pos) {
                print_debug_matrix(&debug_matrix);
                println!(
                    "Warning: Splitter at ({}, {}) already found",
                    spl_pos.0, spl_pos.1
                );
                continue;
            }
            used_splitters += 1;
            println!("found {} {}", spl_pos.0, spl_pos.1);
            debug_matrix[spl_pos.1][spl_pos.0] = '*';

            let x = spl_pos.0;
            let y = spl_pos.1;
            let left = (x - 1, y);
            let right = (x + 1, y);

            if starting_positions.insert(left) {
                deq.push_back(left);
            }
            if starting_positions.insert(right) {
                deq.push_back(right);
            }
        }
    }

    print_debug_matrix(&debug_matrix);

    used_splitters
}

fn print_debug_matrix(matrix: &Vec<Vec<char>>) {
    let height = matrix.len();
    let width = matrix[0].len();

    // Print X axis
    print!("  ");
    for x in 0..width {
        print!("{}", x % 10);
    }
    println!();

    // Print rows with Y
    for y in 0..height {
        print!("{:2} ", y);
        for &c in &matrix[y] {
            print!("{}", c);
        }
        println!();
    }
}

// perform search of splitter below, from position, returns splitter postion if any.
fn cast(from: (usize, usize), splitters: &Vec<usize>) -> Option<(usize, usize)> {
    println!("cast {} {}", from.0, from.1);
    dbg!(splitters);

    let (x, y) = from;

    let mut found_spl_y = None;
    for spl_y in splitters {
        println!("assessing {} {}", spl_y, y);
        if *spl_y <= y {
            println!("skipping {}", spl_y);
            continue;
        }

        found_spl_y = Some(spl_y);
        break;
    }

    if found_spl_y.is_some() {
        return Some((x, *found_spl_y.unwrap()));
    }

    None
}

#[cfg(test)]
mod cast_tests {
    use super::*;

    #[test]
    fn test_cast_with_splitter() {
        let from = (0, 0);
        let splitters = vec![3];
        assert_eq!(cast(from, &splitters), Some((0, 3)));
    }

    #[test]
    fn test_cast_no_splitter() {
        let from = (0, 0);
        let splitters = vec![0];
        assert_eq!(cast(from, &splitters), None);
    }
}

#[test]
fn test_part1() {
    assert_eq!(part_one(TEST_INPUT), 21);
}

#[test]
fn test_part1_prod() {
    let input = crate::fetch::fetch_input(7).unwrap();
    let result = part_one(&input);
    assert_eq!(result, 1560);
}
