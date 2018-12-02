use std::collections::HashSet;
use std::io::{self, BufRead, BufReader};
fn parse_input() -> io::Result<Vec<i32>> {
    use std::fs::File;
    let file = File::open("inputs/input_day1_part1")?;
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    for line in reader.lines() {
        let i: i32 = line?.parse().unwrap();
        numbers.push(i);
    }
    Ok(numbers)
}

fn main() {
    let numbers = parse_input().expect("Unable to read input");
    let sum: i32 = numbers.iter().sum();
    println!("Solution 1 = {}", sum);
    let mut cache = HashSet::new();
    cache.insert(0);
    let mut result = None;
    let mut sum = 0;
    while result.is_none() {
        for &n in &numbers {
            sum += n;
            if !cache.insert(sum) {
                result = Some(sum);
                break;
            }
        }
    }
    println!("Solution 2 = {:?}", result);
}
