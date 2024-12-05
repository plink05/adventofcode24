use regex::Regex;
use std::path::Path;
use std::fs::File;
use std::io::{ self, BufRead, BufReader};


struct ExpressionFinder {
    mul_pattern: Regex,
    do_pattern: Regex,
    dont_pattern: Regex,
}

impl ExpressionFinder {
    fn new() -> Result<Self, regex::Error> {
        Ok(ExpressionFinder {
            mul_pattern: Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?,
            do_pattern: Regex::new(r"do\(\)")?,
            dont_pattern: Regex::new(r"don't\(\)")?,
        })
    }

    fn extract_numbers(&self, text: &str) -> Vec<(u32, u32)> {
        self.mul_pattern
            .captures_iter(text)
            .filter_map(|cap| {
                let first = cap.get(1)?.as_str().parse().ok()?;
                let second = cap.get(2)?.as_str().parse().ok()?;
                Some((first, second))
            })
            .collect()
    }
    fn process_line(&self, line: &str) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
        let mut do_numbers = Vec::new();
        let mut dont_numbers = Vec::new();
        let mut current_pos = 0;
        let mut current_vector = &mut do_numbers;  // Start with do vector

        // Find all pattern matches and sort by their position
        let mut all_matches = Vec::new();
        
        // Collect mul matches with their positions
        for m in self.mul_pattern.find_iter(line) {
            all_matches.push((m.start(), "mul", m.as_str()));
        }
        
        // Collect do() matches
        for m in self.do_pattern.find_iter(line) {
            all_matches.push((m.start(), "do", m.as_str()));
        }
        
        // Collect don't() matches
        for m in self.dont_pattern.find_iter(line) {
            all_matches.push((m.start(), "dont", m.as_str()));
        }
        
        // Sort by position in line
        all_matches.sort_by_key(|&(pos, _, _)| pos);
        
        // Process matches in order
        for (_, match_type, text) in all_matches {
            match match_type {
                "do" => current_vector = &mut do_numbers,
                "dont" => current_vector = &mut dont_numbers,
                "mul" => {
                    if let Some(caps) = self.mul_pattern.captures(text) {
                        if let (Some(first), Some(second)) = (
                            caps.get(1).and_then(|m| m.as_str().parse().ok()),
                            caps.get(2).and_then(|m| m.as_str().parse().ok())
                        ) {
                            current_vector.push((first, second));
                        }
                    }
                },
                _ => {}
            }
        }

        (do_numbers, dont_numbers)
    }
}

struct MulExpressionFinder {
    pattern: Regex,
}

impl MulExpressionFinder {
    fn new() -> Result<Self, regex::Error> {
        // Match 'mul' followed by 1-3 digits, comma, 1-3 digits in parentheses
        Ok(MulExpressionFinder {
            pattern: Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?
        })
    }

    fn find_all<'a>(&self, text: &'a str) -> Vec<&'a str> {
        self.pattern
            .find_iter(text)
            .map(|m| m.as_str())
            .collect()
    }

    // Optional: Extract just the numbers
    fn extract_numbers(&self, text: &str) -> Vec<(u32, u32)> {
        self.pattern
            .captures_iter(text)
            .filter_map(|cap| {
                let first = cap.get(1)?.as_str().parse().ok()?;
                let second = cap.get(2)?.as_str().parse().ok()?;
                Some((first, second))
            })
            .collect()
    }
}

fn process_file(path: &Path) -> io::Result<Vec<(u32, u32)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let finder = MulExpressionFinder::new().expect("Invalid regex pattern");
    
    let mut all_numbers = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let numbers = finder.extract_numbers(&line);
        all_numbers.extend(numbers);
    }
    
    Ok(all_numbers)
}

fn process_file2(path: &Path) -> io::Result<(Vec<(u32, u32)>, Vec<(u32, u32)>)> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let finder = ExpressionFinder::new().expect("Invalid regex pattern");

    println!("{:?}", finder.process_line("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
    
    let mut all_do_numbers = Vec::new();
    let mut all_dont_numbers = Vec::new();

    let content: String = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .join(" ");
    
    let (do_nums, dont_nums) = finder.process_line(&content);
    all_do_numbers.extend(do_nums);
    all_dont_numbers.extend(dont_nums);
    
    Ok((all_do_numbers, all_dont_numbers))
}


fn part1() {
    let path = Path::new("input.txt");
    let mut sum = 0; 
    for number_pair in  process_file(path).unwrap() {
        sum += number_pair.0 * number_pair.1; 
    }
    println!("{:?}", sum);
}

fn part2() {
    let path = Path::new("input.txt");
    let mut sum = 0; 
    println!("{:?}", process_file2(path).unwrap().0);

    for number_pair in process_file2(path).unwrap().0 {
        sum += number_pair.0 * number_pair.1;
    }
    println!("Sum {:?}", sum);
    
}

fn main() {
    part1();
    part2();

}
