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
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashSet};

    let n = 10;
    let mut heap: BinaryHeap<Reverse<(usize, (usize, usize, usize), (usize, usize, usize))>> =
        BinaryHeap::new();

    for (i, (x1, y1, z1)) in inp.iter().enumerate() {
        for (j, (x2, y2, z2)) in inp.iter().enumerate() {
            if i != j {
                let dx = (*x1 as isize - *x2 as isize).abs() as f64;
                let dy = (*y1 as isize - *y2 as isize).abs() as f64;
                let dz = (*z1 as isize - *z2 as isize).abs() as f64;
                let distance = ((dx * dx + dy * dy + dz * dz).sqrt()).round() as usize;

                let coord1 = (*x1, *y1, *z1);
                let coord2 = (*x2, *y2, *z2);
                heap.push(Reverse((distance, coord1, coord2)));
            }
        }
    }

    println!("Heap size {}", heap.len());

    let mut connections: HashSet<((usize, usize, usize), (usize, usize, usize))> = HashSet::new();
    let mut circuit_map: std::collections::HashMap<(usize, usize, usize), usize> =
        std::collections::HashMap::new();
    let mut next_circuit_id: usize = 1;

    while let Some(Reverse((dist, from, to))) = heap.pop() {
        // Order coordinates consistently so we only need to check one direction
        let (coord1, coord2) = if from < to { (from, to) } else { (to, from) };

        if !connections.contains(&(coord1, coord2)) {
            connections.insert((coord1, coord2));

            let circuit_id = match (circuit_map.get(&coord1), circuit_map.get(&coord2)) {
                (Some(&id1), Some(&id2)) => {
                    // Both already in circuits - merge them (keep id1, update id2 to id1)
                    if id1 != id2 {
                        // Reassign all points from id2 to id1
                        let points_to_reassign: Vec<(usize, usize, usize)> = circuit_map
                            .iter()
                            .filter(|(_, id)| **id == id2)
                            .map(|(coord, _)| *coord)
                            .collect();
                        for coord in points_to_reassign {
                            circuit_map.remove(&coord);
                            circuit_map.insert(coord, id1);
                        }
                    }
                    id1
                }
                (Some(&id), None) => {
                    // coord2 is new, add to coord1's circuit
                    circuit_map.insert(coord2, id);
                    id
                }
                (None, Some(&id)) => {
                    // coord1 is new, add to coord2's circuit
                    circuit_map.insert(coord1, id);
                    id
                }
                (None, None) => {
                    // Both are new, create new circuit
                    let id = next_circuit_id;
                    circuit_map.insert(coord1, id);
                    circuit_map.insert(coord2, id);
                    next_circuit_id += 1;
                    id
                }
            };

            println!(
                "Connection: {} from {:?} to {:?} (circuit {})",
                dist, coord1, coord2, circuit_id
            );
        }

        if connections.len() >= n {
            break;
        }
    }

    let mut circuit_counts: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    for &id in circuit_map.values() {
        *circuit_counts.entry(id).or_insert(0) += 1;
    }

    let mut sorted_circuits: Vec<(&usize, &usize)> = circuit_counts.iter().collect();
    sorted_circuits.sort_by(|a, b| b.1.cmp(a.1));

    let points_in_circuits = circuit_map.len();
    let points_not_in_circuits = inp.len() - points_in_circuits;

    println!("\nCircuits (by size, descending):");
    for (circuit_id, count) in sorted_circuits {
        println!("Circuit {}: {} points", circuit_id, count);
    }
    println!("Total circuits: {}", circuit_counts.len());
    println!("Points not in any circuit: {}", points_not_in_circuits);

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
