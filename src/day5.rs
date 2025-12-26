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