#[derive(Debug, PartialEq)]
struct ParseError;

static TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

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
    fn new(state: i64) -> Self {
        Self { state: state }
    }

    fn rotate(&mut self, num: i64) -> i64 {
        let full_rotations = num.abs() / 100;

        let actual_num = num % 100;

        let mut hits = full_rotations;

        let start_state = self.state;
        self.state += actual_num;
        let mut gt_100 = false;
        if self.state >= 100 {
            self.state -= 100;
            hits += 1;
            gt_100 = true;
        } else if self.state < 0 {
            self.state = 100 - self.state.abs();
            if start_state > 0 {
                hits += 1;
            }
        }

        if self.state == 0 && !gt_100 {
            hits += 1;
        }

        hits
    }
}

fn part_2(inp: Vec<i64>) -> i64 {
    let mut d = Dial::new(50);
    let mut pwd = 0;

    for n in inp.iter() {
        let hits = d.rotate(*n);
        // println!("num {}, hits {}", n, hits);
        pwd += hits;
    }

    pwd
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(TEST_INPUT),
        Ok(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82,])
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part_2(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82,]),
        6
    );
}

#[test]
fn test_part2_prod() {
    let input = crate::fetch::fetch_input(1).unwrap();
    let parsed = parse(&input).unwrap();
    let res = part_2(parsed);
    assert_eq!(res, 6175);
}

#[test]
fn test_dial() {
    let mut d = Dial::new(50);
    let hits = d.rotate(-51);
    assert_eq!(d.state, 99);
    assert_eq!(hits, 1);

    let mut d = Dial::new(50);
    let hits = d.rotate(-50);
    assert_eq!(d.state, 0);
    assert_eq!(hits, 1);

    let mut d = Dial::new(50);
    let hits = d.rotate(-151);
    assert_eq!(d.state, 99);
    assert_eq!(hits, 2);

    let mut d = Dial::new(50);
    let hits = d.rotate(-68);
    assert_eq!(d.state, 82);
    assert_eq!(hits, 1);

    let mut d = Dial::new(82);
    let hits = d.rotate(-30);
    assert_eq!(d.state, 52);
    assert_eq!(hits, 0);
}

#[test]
fn test_dial_1() {
    let mut d = Dial::new(52);
    let hits = d.rotate(48);
    assert_eq!(d.state, 0);
    assert_eq!(hits, 1);
}

#[test]
fn test_dial_2() {
    let mut d = Dial::new(0);
    let hits = d.rotate(-5);
    assert_eq!(d.state, 95);
    assert_eq!(hits, 0);
}
