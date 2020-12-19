use std::fs;
use std::collections::HashSet;

struct Instr {
    distance: i32,
    direction: char,
}

fn main() {

    let wires_s = fs::read_to_string("3.txt").expect("Error while reading file!");

    let mut first_wire = vec![];
    let mut second_wire = vec![];

    for (i, wire) in wires_s.split('\n').enumerate() {
        for instr in wire.split(',') {
            match i {
                0 => {
                    first_wire.push(Instr {
                        direction: instr.chars().next().unwrap(),
                        distance: instr[1..].parse().unwrap(),
                    });
                },
                1 => {
                    second_wire.push(Instr {
                        direction: instr.chars().next().unwrap(),
                        distance: instr[1..].parse().unwrap(),
                    });
                },
                _ => panic!("Problem"),
            }
        }
    }

    println!("Part 1: {}", part_1(&mut first_wire, &mut second_wire));
    println!("Part 2: {}", part_2(&mut first_wire, &mut second_wire));
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {

    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn compute_path(instructions: &Vec<Instr>) -> HashSet<(i32, i32)> {

    let mut pos = (0, 0);
    let mut wire_path = HashSet::new();

    for instr in instructions.iter() {
        for _ in 0..instr.distance {
            match instr.direction {
                'L' => pos.0 -= 1,
                'R' => pos.0 += 1,
                'D' => pos.1 += 1,
                'U' => pos.1 -= 1,
                _ => panic!("Invalid direction"),
            }
            wire_path.insert(pos);
        }
    }

    wire_path
}

fn computer_nb_steps(wire_path: &HashSet<(i32, i32)>, )

fn part_1(first_wire: &mut Vec<Instr>, second_wire: &mut Vec<Instr>) -> i32 {

    let first_wire_path = compute_path(&first_wire);
    let second_wire_path = compute_path(&second_wire);

    let intersections: HashSet<_> = first_wire_path.intersection(&second_wire_path).collect();

    let mut min_distance = 100_000_000;
    for intersection in intersections.iter() {
        let distance = manhattan_distance((0, 0), **intersection);
        if distance < min_distance {
            min_distance = distance;
        }
    }

    min_distance
}

fn part_2(first_wire: &mut Vec<Instr>, second_wire: &mut Vec<Instr>) -> i32 {

    let first_wire_path = compute_path(&first_wire);
    let second_wire_path = compute_path(&second_wire);

    let intersections: HashSet<_> = first_wire_path.intersection(&second_wire_path).collect();

    let mut min_steps = 100_000_000;

    for inter in intersections.iter() {
        let mut first_wire_steps = 0;
        let mut second_wire_step = 0;
    }

    min_steps
}
