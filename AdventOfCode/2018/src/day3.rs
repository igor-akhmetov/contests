use std::fs;

fn parse_rect(line: &str) -> (&str, usize, usize, usize, usize) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();

    let x_idx = parts[3].find("x").unwrap();
    let w = parts[3][0..x_idx].parse::<usize>().unwrap();
    let h = parts[3][x_idx + 1..].parse::<usize>().unwrap();

    let comma_idx = parts[2].find(",").unwrap();
    let x = parts[2][0..comma_idx].parse::<usize>().unwrap();
    let y = parts[2][comma_idx + 1..parts[2].len() - 1].parse::<usize>().unwrap();

    (parts[0], x, y, w, h)
}

pub fn day3_1() -> i32 {
    let mut field = vec![vec![0; 1000]; 1000];
    for line in fs::read_to_string("data/day3.txt").unwrap().lines() {
        let (_, x, y, w, h) = parse_rect(line);
        for i in x..x + w {
            for j in y..y + h {
                field[i][j] += 1;
            }
        }
    }
    let mut result = 0;
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if field[i][j] > 1 {
                result += 1;
            }
        }
    }
    result
}

pub fn day3_2() -> String {
    let mut field = vec![vec![0; 1000]; 1000];
    let input = fs::read_to_string("data/day3.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    for line in &lines {
        let (_, x, y, w, h) = parse_rect(line);
        for i in x..x + w {
            for j in y..y + h {
                field[i][j] += 1;
            }
        }
    }
    for line in &lines {
        let (id, x, y, w, h) = parse_rect(line);
        let mut good = true;
        'outer: for i in x..x + w {
            for j in y..y + h {
                if field[i][j] != 1 {
                    good = false;
                    break 'outer;
                }
            }
        }
        if good {
            return String::from(id);
        }
    }    
    String::from("")
}