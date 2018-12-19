use std::collections::HashMap;

fn calc_asleep() -> HashMap<i32, [i32; 60]> {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();
    let mut lines : Vec<&str> = input.lines().collect();    
    let mut asleep = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_start = 0;
    lines.sort_unstable();
    for line in &lines {
        let parts : Vec<&str> = line.split_whitespace().collect();
        if parts[2] == "Guard" {
            guard_id = parts[3][1..].parse().unwrap();
        } else if parts[2] == "falls" {
            sleep_start = parts[1][3..5].parse().unwrap();
        } else {
            assert_eq!(parts[2], "wakes");
            let sleep_end = parts[1][3..5].parse().unwrap();
            let counter = asleep.entry(guard_id).or_insert([0; 60]);
            for i in sleep_start..sleep_end {
                counter[i] += 1
            }
        }
    }
    asleep
}

pub fn day4_1() -> i32 {
    let asleep = calc_asleep();

    let mut best_guard : i32 = -1;
    let mut best_guard_time = -1;
    for (&id, sleep) in &asleep {
        let sum = sleep.iter().sum();
        if sum > best_guard_time {
            best_guard = id;
            best_guard_time = sum;
        }
    }
    let sleep_counts = asleep.get(&best_guard).unwrap();
    let mut best_minute = 0;
    for (i, &cnt) in sleep_counts.iter().enumerate() {
        if cnt > sleep_counts[best_minute] {
            best_minute = i;
        }
    }
    best_guard * best_minute as i32
}

pub fn day4_2() -> i32 {
    let asleep = calc_asleep();

    let mut best_guard : i32 = -1;
    let mut best_minute = 0;
    let mut best_cnt = -1;
    for (&id, sleep) in &asleep {
        for (minute, &cnt) in sleep.iter().enumerate() {
            if cnt > best_cnt {
                best_guard = id;
                best_minute = minute;
                best_cnt = cnt;
            }
        }
    }
    best_guard * best_minute as i32
}