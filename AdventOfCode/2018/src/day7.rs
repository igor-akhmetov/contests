use std::fs;

pub fn day7_1() -> String {
    let mut g = [[false; 26]; 26];
    let mut seen = [false; 26];
    for line in fs::read_to_string("data/day7.txt").unwrap().lines() {
        let before = (line.bytes().nth(5).unwrap() - b'A') as usize;
        let after = (line.bytes().nth(36).unwrap() - b'A') as usize;
        seen[before] = true;
        seen[after] = true;
        g[after][before] = true;
    }
    let mut result = String::new();
    let mut mark = [false; 26];
    for _ in 0..26 {
        'outer: for i in 0..26 {
            if !seen[i] || mark[i] {
                continue;                
            }
            for j in 0..26 {
                if g[i][j] && !mark[j] {
                    continue 'outer;
                }
            }
            mark[i] = true;
            result.push((i as u8 + b'A') as char);
            break;
        }
    }
    result
}

pub fn day7_2() -> usize {
    let mut g = [[false; 26]; 26];
    let mut seen = [false; 26];
    for line in fs::read_to_string("data/day7.txt").unwrap().lines() {
        let before = (line.bytes().nth(5).unwrap() - b'A') as usize;
        let after = (line.bytes().nth(36).unwrap() - b'A') as usize;
        seen[before] = true;
        seen[after] = true;
        g[after][before] = true;
    }
    let mut mark = [false; 26];
    let mut done = [false; 26];
    let mut time = [0; 5];
    let mut task = [0; 5];
    for cur in 0..2500 {
        let mut finished = true;
        for worker in 0..5 {
            if time[worker] > 0 {
                time[worker] -= 1;
                if time[worker] == 0 {
                    done[task[worker]] = true;
                }
                finished = false;
            }
        }
        for worker in 0..5 {
            if time[worker] > 0 {
                continue;
            }
            'outer: for i in 0..26 {
                if !seen[i] || mark[i] {
                    continue;                
                }
                for j in 0..26 {
                    if g[i][j] && !done[j] {
                        continue 'outer;
                    }
                }
                time[worker] = 61 + i;
                mark[i] = true;
                finished = false;
                task[worker] = i;
                break;
            }
        }
        if finished {
            return cur;
        }
    }
    0
}