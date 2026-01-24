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

pub fn part_1(inp: Vec<(usize, usize, usize)>) -> usize {
    todo!()
}

#[test]
fn test_parse() {
    let input = "162,817,812
57,618,57";
    assert_eq!(parse(input), Ok(vec![(162, 817, 812), (57, 618, 57)]));
}
