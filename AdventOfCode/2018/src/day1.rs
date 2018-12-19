use std::fs;
use std::collections::HashSet;

pub fn read_numbers() -> Vec<i32> {
    fs::read_to_string("data/day1.txt").unwrap().lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

pub fn day1_1() -> i32 {
    read_numbers().iter().sum()
}

pub fn day1_2() -> Option<i32> {
    let mut hash = HashSet::new();
    let mut sum = 0;
    let numbers = read_numbers();
    loop {
        for num in &numbers {
            sum += num;
            if !hash.insert(sum) {
                return Some(sum)
            }
        }
    }
}