const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let lines: Vec<&str> = input.trim().lines().collect();

    let matrix: Vec<Vec<usize>> = lines[..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let operators: Vec<char> = lines
        .last()
        .unwrap()
        .split_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();

    (matrix, operators)
}

fn apply_op(a: usize, op: char, b: usize) -> usize {
    match op {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unknown operator"),
    }
}

fn part1(matrix: Vec<Vec<usize>>, operators: Vec<char>) -> usize {
    let cols = matrix[0].len();
    let mut total = 0;
    for col in 0..cols {
        let mut val = matrix[0][col];
        for row in 1..matrix.len() {
            val = apply_op(val, operators[col], matrix[row][col]);
        }
        total += val;
    }
    total
}

#[test]
fn test_parse_input() {
    let (matrix, operators) = parse_input(TEST_INPUT);
    assert_eq!(
        matrix,
        vec![
            vec![123, 328, 51, 64],
            vec![45, 64, 387, 23],
            vec![6, 98, 215, 314]
        ]
    );
    assert_eq!(operators, vec!['*', '+', '*', '+']);
}

#[test]
fn test_part1() {
    let (matrix, operators) = parse_input(TEST_INPUT);
    let result = part1(matrix, operators);
    assert_eq!(result, 4277556);
}

#[test]
fn test_part1_prod() {
    let (matrix, operators) = parse_input(crate::fetch::fetch_input(6).unwrap().as_str());
    let result = part1(matrix, operators);
    assert_eq!(result, 6343365546996);
}
