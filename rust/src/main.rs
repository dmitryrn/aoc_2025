#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
struct ParseError;

fn main() {}

fn parse(str: &str) -> Result<Vec<i64>, ParseError> {
    let actions = str.lines().map(|l| {
        let direction = l.get(0..1).unwrap();
        let amount = l.get(1..).unwrap().parse::<i64>().map_err(|_| ParseError)?;
        match direction {
            "L" => Ok(-amount),
            "R" => Ok(amount),
            _ => Err(ParseError),
        }
    });
    actions.collect()
}


struct Dial {
    state: i64,
}

impl Dial {
    fn rotate(num: i64) -> i64 {
        todo!();
    }
}

fn part_1(inp: Vec<i64>) -> i32 {
    todo!();
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        ),
        Ok(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82,])
    );
}

#[test]
fn test_part_1() {
    
}
