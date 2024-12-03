use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_file(path: &str) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(Path::new(path))?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut line_numbers = Vec::new();

        for word in line.split_whitespace() {
            if let Ok(number) = word.parse::<u32>() {
                line_numbers.push(number);
            }
        }

        result.push(line_numbers);
    }

    Ok(result)
}

fn good_input(array: &Vec<u32>) -> bool {
    let first_two = &array[0..2];
    let mut array = array.clone();
    if first_two[0] > first_two[1] {
        array.reverse();
    }
    for pair in array.windows(2) {
        let greater_than = pair[1] > pair[0];
        let not_greater_than_3 = pair[0] + 3 >= pair[1];
        if !greater_than || !not_greater_than_3 {
            return false;
        }
    }
    return true;
}

fn maybe_good_input(array: &Vec<u32>) -> bool {
    if good_input(array) {
        return true;
    } else {
        return (0..array.len())
            .map(|i| {
                let filtered: Vec<_> = array
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != i)
                    .map(|(_, &x)| x)
                    .collect();
                good_input(&filtered)
            })
            .any(|result| result);
    }
}

fn part1(filename: &str) {
    let data = parse_file(filename).unwrap();
    let size = data
        .iter()
        .filter(|&x| good_input(x))
        .collect::<Vec<&Vec<u32>>>();
    // println!("{:?}", size);
    println!("{:?}", size.len());
}

fn part2(filename: &str) {
    let data = parse_file(filename).unwrap();
    let size = data
        .iter()
        .filter(|&x| maybe_good_input(x))
        .collect::<Vec<&Vec<u32>>>();
    println!("{:?}", size);
    println!("{:?}", size.len());
}

fn main() {
    // part1("input1.txt");
    part2("input1.txt");

    let x = maybe_good_input(&vec![1, 3, 2, 3, 7]);
    println!("{}", x);
}
