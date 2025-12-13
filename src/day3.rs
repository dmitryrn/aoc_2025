use crate::fetch;

const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .map(|l| l.bytes().map(|b| (b - 48) as usize).collect())
        .collect()
}

fn part_1(arrays: Vec<Vec<usize>>) -> usize {
    let mut total = 0;
    for a in arrays {
        let highest = highest_joltage(&a);
        // println!("highest: {:?} {:?}", a, highest);
        let a_str = highest.0.to_string();
        let b_str = highest.1.to_string();
        let ab_str = [a_str, b_str].join("");
        let ab: usize = ab_str.parse().unwrap();
        total += ab
    }

    total
}

fn highest_joltage(array: &Vec<usize>) -> (usize, usize) {
    // let (ign, max1) = max(array, None);
    // let (_, max2) = max(array, Some(ign));

    // (max1, max2)

    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..array.len()-1 {
        let a = array[i];
        let b = array[i+1];

        if a > max1 {
            max1 = a;
            max2 = b;
        }
        if b > max2 {
            max2 = b;
        }
    }

    (max1, max2)
}

fn max(array: &Vec<usize>, ignore_idx: Option<usize>) -> (usize, usize) {
    let mut max = 0;
    let mut max_i = 0;
    for (i, b) in array.iter().enumerate() {
        if *b > max && (ignore_idx.is_none() || (ignore_idx.is_some() && i != ignore_idx.unwrap()))
        {
            max = *b;
            max_i = i;
        }
    }

    (max_i, max)
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("02\n34"), vec![vec![0, 2], vec![3, 4],]);
}

#[test]
fn max_test() {
    assert_eq!(max(&vec![1, 2, 3, 4, 5], None), (4, 5));
    assert_eq!(max(&vec![3,2,1], None), (0, 3));
    assert_eq!(max(&vec![5, 2, 3, 4, 5], None), (0, 5));
    assert_eq!(max(&vec![1, 2, 3, 4, 5], Some(4)), (3, 4));
}

#[test]
fn test_highest_joltage() {
    assert_eq!(highest_joltage(&parse_input("987654321111111")[0]), (9, 8));
    assert_eq!(highest_joltage(&parse_input("811111111111119")[0]), (8, 9));
    assert_eq!(highest_joltage(&parse_input("234234234234278")[0]), (7, 8));
    assert_eq!(highest_joltage(&parse_input("818181911112111")[0]), (9, 2));
}

// 818181911112111
//              ab
// a b
// 9 1

#[test]
fn test_part_1() {
    let inp = parse_input(TEST_INPUT);
    let result = part_1(inp);
    assert_eq!(result, 357);
}

#[test]
fn test_part_1_prod() {
    let inp = parse_input(fetch::fetch_input(3).unwrap().as_str());
    let result = part_1(inp);
    assert_eq!(result, 17613);
}