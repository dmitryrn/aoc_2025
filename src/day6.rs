const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

fn parse_input_part_1(input: &str) -> (Vec<Vec<usize>>, Vec<char>) {
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

fn part2(input: &str) -> usize {
    let last = input.lines().last().unwrap();

    let mut operator_indexes = vec![];
    let mut operators = vec![];
    for (i, c) in last.chars().enumerate() {
        if c == '*' || c == '+' {
            operator_indexes.push(i);
            operators.push(c);
        }
    }

    let height = input.lines().count() - 1;
    let width = last.len();
    println!("w{} '{}'", width, last);

    let input_b: Vec<_> = input.chars().collect();

    let mut total = 0;

    for i in 0..operator_indexes.len() {
        let end_idx = if i + 1 < operator_indexes.len() {
            operator_indexes[i + 1]
        } else {
            width + 1
        };
        let pair = (operator_indexes[i], end_idx);

        let start = pair.0;
        let end = pair.1 - 1;
        let op = operators[i];
        println!("pair{} {}", pair.0, pair.1);

        let mut local_total: usize = if op == '+' { 0 } else { 1 };

        for x in start..end {
            let mut num_str = String::new();
            for y in 0..height {
                let i = x + (width + 1) * y;
                let mut c = input_b[i];
                if c == ' ' {
                    c = '_';
                }
                println!("{} {} {} {}", x, y, i, c);

                if c != '_' {
                    num_str.push(c);
                }
            }

            let num: usize = num_str.parse().unwrap();
            println!("num{}", num);

            println!("--- {} {} {}", local_total, op, num);
            local_total = apply_op(local_total, op, num)
        }

        println!("lt{}", local_total);
        total += local_total;
    }

    total
}

#[test]
fn test_parse_input() {
    let (matrix, operators) = parse_input_part_1(TEST_INPUT);
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
    let (matrix, operators) = parse_input_part_1(TEST_INPUT);
    let result = part1(matrix, operators);
    assert_eq!(result, 4277556);
}

#[test]
fn test_part1_prod() {
    let (matrix, operators) = parse_input_part_1(crate::fetch::fetch_input(6).unwrap().as_str());
    let result = part1(matrix, operators);
    assert_eq!(result, 6343365546996);
}

#[test]
fn test_part2() {
    let result = part2(TEST_INPUT);
    assert_eq!(result, 3263827);
}

#[test]
fn test_part2_prod() {
    let result = part2(crate::fetch::fetch_input(6).unwrap().as_str());
    assert_eq!(result, 11136895955912);
}
