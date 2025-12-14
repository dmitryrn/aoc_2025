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

fn algo(arrays: Vec<Vec<usize>>, n: usize) -> usize {
    let mut total = 0;
    for a in arrays {
        let highest = highest_joltage_n(&a, n);
        let ab_str: Vec<_> = highest.iter().map(|n| (*n as u8) + 48).collect();
        let ab_str = str::from_utf8(&ab_str).unwrap();
        let ab: usize = ab_str.parse().unwrap();
        total += ab
    }

    total
}

fn part_1(arrays: Vec<Vec<usize>>) -> usize {
    algo(arrays, 2)
}

fn part_2(arrays: Vec<Vec<usize>>) -> usize {
    algo(arrays, 12)
}

fn highest_joltage(array: &Vec<usize>) -> Vec<usize> {
    let mut max1 = 0;
    let mut max2 = 0;
    for i in 0..array.len() - 1 {
        let a = array[i];
        let b = array[i + 1];

        if a > max1 {
            max1 = a;
            max2 = b;
        }
        if b > max2 {
            max2 = b;
        }
    }

    vec![max1, max2]
}

fn highest_joltage_n(array: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut maxes = vec![0; n];

    for i in 0..array.len() - n + 1 {
        let mut reset_consecutive = false;
        for j in i..i + n {
            let idx = j - i;

            if reset_consecutive {
                maxes[idx] = 0
            }

            if array[j] > maxes[idx] {
                maxes[idx] = array[j];
                reset_consecutive = true;
            }
        }
    }

    maxes
}

#[test]
fn test_highest_joltage_n() {
    let myassert = |given, expected: &str| {
        let expected: Vec<usize> = expected.bytes().map(|b| (b - 48) as usize).collect();
        assert_eq!(highest_joltage_n(&parse_input(given)[0], 12), expected);
    };

    myassert("987654321111111", "987654321111");
    myassert("811111111111119", "811111111119");
    myassert("234234234234278", "434234234278");
    myassert("818181911112111", "888911112111");
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("02\n34"), vec![vec![0, 2], vec![3, 4],]);
}

#[test]
fn test_highest_joltage() {
    assert_eq!(
        highest_joltage(&parse_input("987654321111111")[0]),
        vec![9, 8]
    );
    assert_eq!(
        highest_joltage(&parse_input("811111111111119")[0]),
        vec![8, 9]
    );
    assert_eq!(
        highest_joltage(&parse_input("234234234234278")[0]),
        vec![7, 8]
    );
    assert_eq!(
        highest_joltage(&parse_input("818181911112111")[0]),
        vec![9, 2]
    );
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
    let inp = parse_input(crate::fetch::fetch_input(3).unwrap().as_str());
    let result = part_1(inp);
    assert_eq!(result, 17613);
}

#[test]
fn test_part_2_prod() {
    let inp = parse_input(crate::fetch::fetch_input(3).unwrap().as_str());
    let result = part_2(inp);
    assert_eq!(result, 175304218462560);
}
