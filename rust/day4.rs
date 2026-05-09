const TEST_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c == '@' { '@' } else { '.' })
                .collect()
        })
        .collect()
}

fn part1(matrix: Vec<Vec<char>>) -> usize {
    let mut count_valid_rolls = 0;
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            if valid(&matrix, x, y, 4) {
                count_valid_rolls += 1;
            }
        }
    }

    count_valid_rolls
}

fn part2(mut matrix: Vec<Vec<char>>) -> usize {
    let mut total_removed = 0;

    loop {
        let mut removed = false;

        for y in 0..matrix.len() {
            for x in 0..matrix[0].len() {
                if valid(&matrix, x, y, 4) {
                    matrix[y][x] = '.';

                    total_removed += 1;
                    removed = true
                }
            }
        }

        if !removed {
            break;
        }
    }

    total_removed
}

fn valid(matrix: &Vec<Vec<char>>, x: usize, y: usize, max: usize) -> bool {
    if matrix[y][x] != '@' {
        return false;
    }

    let coords_around = [
        (x.checked_sub(1), Some(y)),
        (x.checked_add(1), Some(y)),
        (Some(x), y.checked_sub(1)),
        (Some(x), y.checked_add(1)),
        (x.checked_sub(1), y.checked_sub(1)),
        (x.checked_sub(1), y.checked_add(1)),
        (x.checked_add(1), y.checked_sub(1)),
        (x.checked_add(1), y.checked_add(1)),
    ];

    let mut count_rolls_around = 0;

    for (x, y) in coords_around {
        if x.is_none() || y.is_none() {
            continue;
        }
        let x = x.unwrap();
        let y = y.unwrap();

        let item_wrapped = matrix.get(y).map(|row| row.get(x));
        if item_wrapped.is_none() || item_wrapped.unwrap().is_none() {
            continue;
        }
        let item = item_wrapped.unwrap().unwrap();

        if *item == '@' {
            count_rolls_around += 1;
            if count_rolls_around >= max {
                return false;
            }
        }
    }

    true
}

#[test]
fn test_parse_input() {
    let res = parse_input(TEST_INPUT);
    assert_eq!(res.len(), 10);
    assert_eq!(res[0].len(), 10);
    assert_eq!(
        res[0],
        vec!['.', '.', '@', '@', '.', '@', '@', '@', '@', '.']
    );
}

#[test]
fn test_part1() {
    let matrix = parse_input(TEST_INPUT);
    assert_eq!(part1(matrix), 13);
}

#[test]
fn test_valid() {
    let matrix = parse_input(TEST_INPUT);
    // assert_eq!(valid(&matrix, 0, 0, 4), false);
    assert_eq!(valid(&matrix, 2, 0, 4), true);
}

#[test]
fn test_part1_prod() {
    let inp = parse_input(crate::fetch::fetch_input(4).unwrap().as_str());
    let result = part1(inp);
    assert_eq!(result, 1395);
}

#[test]
fn test_part2() {
    let matrix = parse_input(TEST_INPUT);
    assert_eq!(part2(matrix), 43);
}

#[test]
fn test_part2_prod() {
    let inp = parse_input(crate::fetch::fetch_input(4).unwrap().as_str());
    let result = part2(inp);
    assert_eq!(result, 8451);
}
