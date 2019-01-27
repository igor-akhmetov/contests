use std::cmp::{min, max};
use std::thread;

const STACK_SIZE: usize = 8 * 1024 * 1024;

fn dfs(v: &mut Vec<Vec<u8>>, x: usize, y: usize, callback: &mut FnMut(usize, usize)) {
    let mut process_cell = |v: &mut Vec<Vec<u8>>, x: usize, y: usize| {
        v[x][y] = 2;
        callback(x, y);
        if y == v[x].len() - 1 {
            return false;
        }
        if v[x][y+1] == 0 {
            dfs(v, x, y + 1, callback)
        }
        return v[x][y+1] == 1 || v[x][y + 1] == 3;
    };
    if !process_cell(v, x, y) {
        return;
    }
    let mut left = x - 1;
    while v[left][y] == 0 {
        if !process_cell(v, left, y) {
            break;
        }
        left -= 1;
    }
    let mut right = x + 1;
    while v[right][y] == 0 {
        if !process_cell(v, right, y) {
            break;
        }
        right += 1;
    }
    if v[left][y] == 1 && v[right][y] == 1 {
        for xx in left + 1..right {
            v[xx][y] = 3;
        }
    }
}

fn day17_impl() -> (i32, i32) {
    let input = std::fs::read_to_string("data/day17.txt").unwrap();    
    let mut v = vec![vec![0 as u8; 2000]; 1000];
    let mut min_y = std::usize::MAX;
    let mut max_y = std::usize::MIN;
    for line in input.lines() {
        let comma_pos = line.find(",").unwrap();
        let dotdot_pos = line.find("..").unwrap();
        let a: usize = line[2..comma_pos].parse().unwrap();
        let b: usize = line[comma_pos + 4..dotdot_pos].parse().unwrap();
        let c: usize = line[dotdot_pos + 2..].parse().unwrap();
        if line.starts_with("x=") {
            for y in b..c+1 {
                v[a][y] = 1;
            }
            min_y = min(min_y, b);
            max_y = max(max_y, c);
        } else {
            for x in b..c+1 {
                v[x][a] = 1;
            }
            min_y = min(min_y, a);
            max_y = max(max_y, a);
        }
    }
    let mut cnt = 0;
    dfs(&mut v, 500, 1, &mut |_x, y| { if y >= min_y && y <= max_y { cnt += 1}; });
    let mut water_cnt = 0;
    for y in 0..v[0].len() {
        for x in 0..v.len() {
            // print!("{}", match v[x][y] {
            //     0 => '.',
            //     1 => '#',
            //     2 => '|',
            //     3 => '~',
            //     _ => panic!("Unexpected value")
            // });
            if v[x][y] == 3 {
                water_cnt += 1;
            }
        }
        //println!();
    }
    (cnt, water_cnt)
}

pub fn day17() -> (i32, i32) {
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(day17_impl)
        .unwrap();
    child.join().unwrap()
}