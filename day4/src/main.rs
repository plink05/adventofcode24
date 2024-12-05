use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn parse_file(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut line_numbers = Vec::new();

        for word in line.split_inclusive("").skip(1) {
            if let Ok(number) = word.parse::<String>() {
                line_numbers.push(number);
            }
        }

        result.push(line_numbers);
    }

    Ok(result)
}

fn find_starts(array: Vec<Vec<String>>, start: String) -> Vec<(usize, usize)> {
    array.iter().enumerate()
        .flat_map(|(i, sublist)| {
            sublist.iter().enumerate()
                .filter(|(_, &ref val)| *val == start)
                .map(move |(j, _)| (i, j))
        })
        .collect()
}

// Function that checks if next value is correct
fn is_correct(slice: String) -> bool {
    slice == "XMAS".to_string()
}

const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0], 
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0], 
    [1, 1]
];

const XDIRECTIONS: [[i32; 2]; 2] = [
    [-1, -1],
    [-1, 1],
];

fn get_string_from_position(array: &Vec<Vec<String>>, index: (usize, usize), direction: &[i32; 2], slice: std::ops::Range<i32>) -> String {

    slice.into_iter().map(|i| {
        
        let index1:  Result<usize,_>= (index.0 as i32 + direction[0] * i).try_into();
        let index2: Result<usize, _> = (index.1 as i32 + direction[1] * i).try_into();
        if index1.is_ok() && index2.is_ok() {
            let index1 = index1.unwrap();
            let index2 = index2.unwrap();
            if index1 < array.len() && index2 < array[0].len() {
                array[index1][index2].clone()
            } else {
                return "".to_string()
            }
        } else {
            return "".to_string()
        }
    }).collect::<String>()
    
}

fn get_count(array: &Vec<Vec<String>>, index: (usize, usize)) -> u32 {
    DIRECTIONS.iter().fold(0, |acc, direction| {
        let value: String = get_string_from_position(array, index, direction, 0..4);
        if is_correct(value.clone()) {
            acc + 1
        } else {
            acc
        }
    }) 
}

fn is_diag_correct(test: String) -> bool {
    let forward = "MAS".to_string();
    let backward = "SAM".to_string();
    test == forward || test == backward
}

fn get_diagonal_count(array: &Vec<Vec<String>>, index: (usize, usize)) -> u32 {
    let correct = XDIRECTIONS.iter().fold(0, |acc, diagonal| {

        let value: String = get_string_from_position(array, index, diagonal, -1..2);
        if is_diag_correct(value.clone()) {
            acc + 1
        } else {
            acc
        }
    });
    if correct == 2 {
        1
    } else {
        0
    }
}

fn part2() {
    let sample = parse_file("input.txt").unwrap();
    let positions = find_starts(sample.clone(), "A".to_string());
    let sum = positions.into_iter().fold(0, |acc, position| {
        acc + get_diagonal_count(&sample, position)
    });
    println!("{:?}", sum);
}



fn part1() {
    let sample = parse_file("input.txt").unwrap();
    let positions = find_starts(sample.clone(), "X".to_string());
    let sum = positions.into_iter().fold(0, |acc, position| {
        acc + get_count(&sample, position)
    });
    println!("{:?}", sum);
    
}

// Function that takes slices in each of the 8 directions

fn main() {
    part1();
    part2();

    
}
