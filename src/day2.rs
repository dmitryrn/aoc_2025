use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct ParseError;

static TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

fn parse(str: &str) -> Result<Vec<(usize, usize)>, ParseError> {
    str.trim().split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return Err(ParseError);
            }
            let start = parts[0].parse::<usize>().map_err(|_| ParseError)?;
            let end = parts[1].parse::<usize>().map_err(|_| ParseError)?;
            Ok((start, end))
        })
        .collect()
}

fn is_invalid_id_part_1(n: usize) -> bool {
    let s = n.to_string();

    if s.len() % 2 != 0 {
        return false;
    }

    let (a, b) = s.split_at(s.len() / 2);
    assert!(a.len() > 0);
    assert!(b.len() > 0);

    a == b
}

fn factorize(n: usize) -> Vec<usize> {
    let mut factors = HashSet::new();

    let squared = (n as f64).sqrt() as usize;

    for i in 1..squared + 1 {
        if n % i == 0 {
            factors.insert(i);
            factors.insert(n / i);
        }
    }

    let mut sl: Vec<usize> = factors.into_iter().collect();
    sl.sort();

    sl
}

fn is_invalid_id_part_2(n: usize) -> bool {
    let s = n.to_string();

    if s.len() == 1 {
        return false;
    }

    let factors = factorize(s.len());

    for f in factors {
        if f == s.len() {
            continue;
        }

        let mut all_eq = true;

        let first = &s[..f];
        for i in (f..s.len()).step_by(f) {
            let chunk = &s[i..i + f];

            if first != chunk {
                all_eq = false;
                break;
            }
        }
        if all_eq {
            return true;
        }
    }

    false
}

fn algo(inp: Vec<(usize, usize)>, is_invalid_id: fn(usize) -> bool) -> usize {
    let mut sum = 0;
    for (a, b) in inp.iter() {
        for n in *a..*b + 1 {
            if is_invalid_id(n) {
                sum += n;
            }
        }
    }

    sum
}

fn part_1(inp: Vec<(usize, usize)>) -> usize {
    algo(inp, is_invalid_id_part_1)
}

fn part_2(inp: Vec<(usize, usize)>) -> usize {
    algo(inp, is_invalid_id_part_2)
}

#[test]
fn test_parse_1() {
    assert_eq!(parse("11-22,95-115"), Ok(vec![(11, 22), (95, 115)]));
}

#[test]
fn test_parse_2() {
    assert_eq!(
        parse(TEST_INPUT),
        Ok(vec![
            (11, 22),
            (95, 115),
            (998, 1012),
            (1188511880, 1188511890),
            (222220, 222224),
            (1698522, 1698528),
            (446443, 446449),
            (38593856, 38593862),
            (565653, 565659),
            (824824821, 824824827),
            (2121212118, 2121212124)
        ])
    );
}

#[test]
fn test_is_invlaid_id() {
    assert_eq!(is_invalid_id_part_1(99), true);
    assert_eq!(is_invalid_id_part_1(12), false);
}

#[test]
fn test_part1() {
    let parsed = parse(TEST_INPUT).unwrap();
    let res = part_1(parsed);
    assert_eq!(res, 1227775554);
}

#[test]
fn test_part2() {
    let parsed = parse(TEST_INPUT).unwrap();
    let res = part_2(parsed);
    assert_eq!(res, 4174379265);
}

#[test]
fn test_part1_prod() {
    let input = crate::fetch::fetch_input(2).unwrap();
    let parsed = parse(&input).unwrap();
    let res = part_1(parsed);
    assert_eq!(res, 40398804950);
}

#[test]
fn test_part2_prod() {
    let input = crate::fetch::fetch_input(2).unwrap();
    let parsed = parse(&input.trim()).unwrap();
    let res = part_2(parsed);
    assert_eq!(res, 65794984339);
}

#[test]
fn test_factorize() {
    let fs = factorize(6);
    assert_eq!(fs, vec![1, 2, 3, 6]);

    let fs = factorize(50);
    assert_eq!(fs, vec![1, 2, 5, 10, 25, 50]);

    let fs = factorize(12);
    assert_eq!(fs, vec![1, 2, 3, 4, 6, 12]);

    let fs = factorize(18);
    assert_eq!(fs, vec![1, 2, 3, 6, 9, 18]);
}

#[test]
fn test_is_invalid_id_part_2_1() {
    assert_eq!(is_invalid_id_part_2(11), true);
}

#[test]
fn test_is_invalid_id_part_2_2() {
    assert_eq!(is_invalid_id_part_2(12), false);
}

#[test]
fn test_part2_with_2_txt() {
    let input = crate::fetch::fetch_input(2).unwrap();
    let parsed = parse(&input.trim()).unwrap();
    let res = part_2(parsed);
    assert_eq!(res, 65794984339);
}
