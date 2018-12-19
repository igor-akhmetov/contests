use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day2_1() -> i32 {
    let mut has_two_letters = 0;
    let mut has_three_letters = 0;
    let mut ch_counts = HashMap::new();
    for line in fs::read_to_string("data/day2.txt").unwrap().lines() {
        ch_counts.clear();
        for ch in line.chars() {
            let counter = ch_counts.entry(ch).or_insert(0);
            *counter += 1;
        }
        for (_, &cnt) in &ch_counts {
            if cnt == 2 {
                has_two_letters += 1;
                break;
            }
        }
        for (_, &cnt) in &ch_counts {
            if cnt == 3 {
                has_three_letters += 1;
                break;
            }
        }
    }
    has_two_letters * has_three_letters
}

pub fn day2_2() -> String {
    let input = fs::read_to_string("data/day2.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut ids = HashSet::<String>::new();
    for i in 0..lines[0].len() {
        ids.clear();
        for line in &lines {
            let s = line[..i].to_owned() + &line[i+1..];
            if !ids.insert(s.clone()) {
                return s;
            }
        }
    }    
    String::from("")
}