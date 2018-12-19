use std::cmp::min;
use std::cmp::max;

struct Star {
    x: i32, y: i32, dx: i32, dy: i32
}

pub fn day10_1() -> u32 {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    let mut stars = Vec::new();
    for line in input.lines() {
        stars.push(Star {
            x: line[10..16].trim().parse().unwrap(),
            y: line[18..24].trim().parse().unwrap(),
            dx: line[36..38].trim().parse().unwrap(),
            dy: line[40..42].trim().parse().unwrap()
        });
    }
    let mut prev_width = std::i32::MAX;
    let mut prev_height = std::i32::MAX;
    let mut seconds = 0;
    loop {
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut min_y = std::i32::MAX;
        let mut max_y = std::i32::MIN;
        for star in &mut stars {
            star.x += star.dx;
            star.y += star.dy;
            min_x = min(min_x, star.x);
            max_x = max(max_x, star.x);
            min_y = min(min_y, star.y);
            max_y = max(max_y, star.y);
        }
        let cur_width = max_x - min_x;
        let cur_height = max_y - min_y;
        if prev_width < cur_width || prev_height < cur_height {
            break;
        }
        seconds += 1;
        prev_width = cur_width;
        prev_height = cur_height;
    }
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for star in &mut stars {
        star.x -= star.dx;
        star.y -= star.dy;
        min_x = min(min_x, star.x);
        max_x = max(max_x, star.x);
        min_y = min(min_y, star.y);
        max_y = max(max_y, star.y);
    }
    let mut sky = vec![vec![false; (max_y - min_y + 1) as usize]; (max_x - min_x + 1) as usize];
    for star in &mut stars {
        sky[(star.x - min_x) as usize][(star.y - min_y) as usize] = true;
    }
    for cur_y in 0..sky[0].len() {
        for cur_x in 0..sky.len() {
            print!("{}", if sky[cur_x][cur_y] { '#' } else { ' ' });
        }
        println!();
    }
    seconds
}