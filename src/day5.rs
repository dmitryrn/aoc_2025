use std::io::BufRead;

const TEST_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

trait Contains {
    fn contains(&self, n: &usize) -> bool;
}

impl Contains for (usize, usize) {
    fn contains(&self, n: &usize) -> bool {
        *n >= self.0 && *n <= self.1
    }
}

trait Intersects {
    fn intersects(&self, other: &(usize, usize)) -> bool;
}

impl Intersects for (usize, usize) {
    fn intersects(&self, other: &(usize, usize)) -> bool {
        (self.1 >= other.0 && self.0 < other.1) || (self.0 <=other.1 && self.0 >= other.1)
    }
}

// a b
//  c d
// 
//  a b
// c d

fn merge(a: &(usize, usize), b: &(usize, usize)) -> (usize, usize) {
    if !a.intersects(b) {
        panic!("ranges do not intersect")
    }
    
    let mut all = vec![a.0, a.1, b.0, b.1];
    all.sort();
    
    (all[0], all[3])
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let input = input.trim();
    let (ranges, products) = input.split_at(input.find("\n\n").unwrap());
    
    let ranges = ranges.trim().as_bytes().lines().map(|l|  {
        let l = l.unwrap();
        let spl: Vec<_> = l.split('-').collect();
        let a: usize = spl[0].parse().unwrap();
        let b: usize = spl[1].parse().unwrap();
        (a, b)
    }).collect();
    
    let products = products.trim().lines().map(|l| l.parse().unwrap()).collect();
    
    (ranges, products)
}

fn part1(ranges: Vec<(usize, usize)>, products: Vec<usize>) -> usize {
    let mut count = 0;
    
    for p in products.iter() {
        for r in ranges.iter() {
            if r.contains(p) {
                count += 1;
                break
            }
        }
    }
    
    count
}

fn part2(ranges: Vec<(usize, usize)>, products: Vec<usize>) -> usize {
    let mut ranges = ranges.clone();
    ranges.sort_by(|(a, _), (b, _)| {
        if a > b {
            std::cmp::Ordering::Greater
        } else if a < b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    });
    
    let mut merged: Vec<(usize, usize)> = vec![];
    for r in ranges {
        if merged.len() == 0 {
            merged.push(r);
            continue
        }
        
        let last = merged.last().unwrap();
        if last.intersects(&r) {
            let l = merged.len();
            merged[l - 1] = merge(last, &r)
        } else {
            merged.push(r)
        }
    }
    
    let mut total = 0;
    for r in merged {
        total += r.1 - r.0 + 1
    }
    
    total
}

#[test]
fn test_parse_input() {
    let (ranges, products) = parse_input(TEST_INPUT);
    assert_eq!(ranges, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
    assert_eq!(products, vec![1, 5, 8, 11, 17, 32]);
}

#[test]
fn test_part1() {
    let (ranges, products) = parse_input(TEST_INPUT);
    assert_eq!(part1(ranges, products), 3);
}

#[test]
fn test_part1_prod() {
    let (ranges, products) = parse_input(crate::fetch::fetch_input(5).unwrap().as_str());
    let result = part1(ranges, products);
    assert_eq!(result, 679);
}

#[test]
fn test_part2() {
    let (ranges, products) = parse_input(TEST_INPUT);
    assert_eq!(part2(ranges, products), 14);
}

#[test]
fn test_part2_prod() {
    let (ranges, products) = parse_input(crate::fetch::fetch_input(5).unwrap().as_str());
    let result = part2(ranges, products);
    assert_eq!(result, 358155203664116);
}

#[test]
fn test_intersects() {
    assert_eq!((1, 2).intersects(&(3, 4)), false);
    assert_eq!((3, 4).intersects(&(1, 2)), false);
    
    assert_eq!((1, 2).intersects(&(2, 3)), true);
    assert_eq!((2, 3).intersects(&(1, 2)), true);
    
    assert_eq!((1, 3).intersects(&(2, 3)), true);
    assert_eq!((2, 3).intersects(&(1, 3)), true);
}