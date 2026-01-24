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

pub fn print_circuit_sizes(circuit_map: &std::collections::HashMap<(usize, usize, usize), usize>) {
    use std::collections::HashMap;

    let mut circuit_counts: HashMap<usize, usize> = HashMap::new();
    for &id in circuit_map.values() {
        *circuit_counts.entry(id).or_insert(0) += 1;
    }

    let mut circuit_sizes: Vec<(usize, usize)> = circuit_counts.into_iter().collect();
    circuit_sizes.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Circuits (by size, descending):");
    for (circuit_id, size) in &circuit_sizes {
        if *size > 1 {
            println!("Circuit {}: {} points", circuit_id, size);
        }
    }

    let single_point_circuits = circuit_sizes.iter().filter(|(_, size)| *size == 1).count();
    println!("Circuits with 1 point: {}", single_point_circuits);
}

pub fn part1(inp: Vec<(usize, usize, usize)>) -> usize {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};

    let n = 10;
    let mut heap: BinaryHeap<Reverse<(usize, (usize, usize, usize), (usize, usize, usize))>> =
        BinaryHeap::new();

    for i in 0..inp.len() {
        let (x1, y1, z1) = inp[i];
        for j in (i + 1)..inp.len() {
            let (x2, y2, z2) = inp[j];
            let dx = (x1 as isize - x2 as isize) as isize;
            let dy = (y1 as isize - y2 as isize) as isize;
            let dz = (z1 as isize - z2 as isize) as isize;
            let dist2 = (dx * dx + dy * dy + dz * dz) as usize;

            let coord1 = (x1, y1, z1);
            let coord2 = (x2, y2, z2);
            heap.push(Reverse((dist2, coord1, coord2)));
        }
    }

    // Initialize circuit_map: each point is its own circuit
    let mut circuit_map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut next_circuit_id: usize = 1;

    for &coord in inp.iter() {
        circuit_map.insert(coord, next_circuit_id);
        next_circuit_id += 1;
    }

    let mut edges_processed = 0;
    while edges_processed < n {
        if let Some(Reverse((_, from, to))) = heap.pop() {
            edges_processed += 1;
    
            let id1 = circuit_map[&from];
            let id2 = circuit_map[&to];
    
            if id1 != id2 {
                let coords_to_reassign: Vec<_> = circuit_map
                    .iter()
                    .filter(|(_, id)| **id == id2)
                    .map(|(coord, _)| *coord)
                    .collect();
    
                for coord in coords_to_reassign {
                    circuit_map.insert(coord, id1);
                }
            }
    
            print_circuit_sizes(&circuit_map);
            println!("-----------------------------");
        } else {
            break;
        }
    }

    print_circuit_sizes(&circuit_map);

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
