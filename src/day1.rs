use std::cmp::max;
use std::fs;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read};

fn main() {
    // --snip--
    let file_path = "src/input/day1.txt";
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut max_2 = 0;
    let mut max_3 = 0;
    let mut current_calories = 0;
    let mut top_3 = Vec::new();
    top_3.push(0);
    top_3.push(0);
    top_3.push(0);


    let lines = reader.lines();
    for line in lines {
        if let Ok(ip) = line {
            if ip.is_empty() {
                top_3.push(current_calories);
                top_3.sort_by(|a, b| b.cmp(a));
                top_3.pop();

                current_calories = 0;
            } else {
                let cal_value = ip.parse::<i32>().unwrap();
                current_calories += cal_value;
            }
        }
    }

    println!();

    println!("total of top 3 {}", top_3.get(0).unwrap() + top_3.get(1).unwrap() + top_3.get(2).unwrap());
}