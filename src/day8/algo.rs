#[derive(Debug, PartialEq)]
pub struct ParseError;

pub fn parse(str: &str) -> Result<Vec<(usize, usize, usize)>, ParseError> {
    str.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 3 {
                return Err(ParseError);
            }
            let a = parts[0].trim().parse::<usize>().map_err(|_| ParseError)?;
            let b = parts[1].trim().parse::<usize>().map_err(|_| ParseError)?;
            let c = parts[2].trim().parse::<usize>().map_err(|_| ParseError)?;
            Ok((a, b, c))
        })
        .collect()
}

pub fn read_test_input() -> String {
    std::fs::read_to_string("src/day8/test_input.txt").expect("Failed to read test input")
}

pub fn find_distances(
    inp: &[(usize, usize, usize)],
) -> std::collections::HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
    let mut distances: std::collections::HashMap<(usize, usize), Vec<((usize, usize), usize)>> =
        std::collections::HashMap::new();

    for (i, (x1, y1, _)) in inp.iter().enumerate() {
        for (j, (x2, y2, _)) in inp.iter().enumerate() {
            if i != j {
                let distance = ((*x1 as isize - *x2 as isize).abs()
                    + (*y1 as isize - *y2 as isize).abs()) as usize;
                let coord1 = (*x1, *y1);
                let coord2 = (*x2, *y2);
                distances
                    .entry(coord1)
                    .or_insert_with(Vec::new)
                    .push((coord2, distance));
            }
        }
    }

    distances
}

pub fn part1(inp: Vec<(usize, usize, usize)>) -> usize {
    let distances = find_distances(&inp);
    println!("Distances: {:?}", distances);
    todo!()
}

#[test]
fn test_parse() {
    let input = "162,817,812
57,618,57";
    assert_eq!(parse(input), Ok(vec![(162, 817, 812), (57, 618, 57)]));
}

#[test]
fn test_part1() {
    let input_str = read_test_input();
    let input = parse(&input_str).unwrap();
    part1(input);
}

#[test]
fn test_part1_simple() {
    let input = parse("0,0,0\n3,4,0\n6,0,0").unwrap();
    let distances = find_distances(&input);

    let expected_distances: std::collections::HashMap<
        (usize, usize),
        Vec<((usize, usize), usize)>,
    > = [
        ((0, 0), vec![((3, 4), 7), ((6, 0), 6)]),
        ((3, 4), vec![((0, 0), 7), ((6, 0), 7)]),
        ((6, 0), vec![((0, 0), 6), ((3, 4), 7)]),
    ]
    .into_iter()
    .collect();

    assert_eq!(distances, expected_distances);
}
