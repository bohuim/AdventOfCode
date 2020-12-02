// use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

use std::collections::HashSet;

fn main()
{
    let name = Path::new("1.input");
    let file = File::open(name).expect("file not found");
    let numbers: Vec<i32> = BufReader::new(file).lines()
        .filter_map(Result::ok)             // Filter for ok results, which should be all lines.
        .map(|s| s.parse::<i32>().unwrap()) // Map to integers.
        .collect();                         // Guess I don't really need to collect into vector.

    // Part 1
    println!("{:#?}", numbers.iter().sum::<i32>());

    // Part 2
    let mut seen: HashSet<i32> = HashSet::new();
    let mut current = 0;
    seen.insert(current);
    // Infinitely cycle over stream of numbers. Maybe for loop isn't the best?
    for num in numbers.iter().cycle() {
        current += num;
        if seen.contains(&current) {
            println!("repeated {:#?}", current);
            return;
        }
        seen.insert(current);
    } 
}

