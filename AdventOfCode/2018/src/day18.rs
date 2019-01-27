pub fn day18_1() -> i32 {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();
    let mut v = vec![vec![0 as u8; 52]; 52];
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            v[y + 1][x + 1] = match b {
                b'.' => 0,
                b'|' => 1,
                b'#' => 2,
                _ => panic!()
            }
        }
    }
    let dx = [-1 as i32, 0, 1, 1, 1, 0, -1, -1];
    let dy = [-1 as i32, -1, -1, 0, 1, 1, 1, 0];
    for _ in 0..10 {
        let mut w = v.clone();
        for y in 1..51 as usize {
            for x in 1..51 as usize {
                let mut cnt = [0; 3];
                for i in 0..dx.len() {
                    let yy = (y as i32 + dy[i]) as usize;
                    let xx = (x as i32 + dx[i]) as usize;
                    if xx > 0 && yy > 0 && xx <= 50 && yy <= 50 {
                        cnt[v[yy][xx] as usize] += 1;
                    }
                }
                w[y][x] = match v[y][x] {
                    0 => if cnt[1] >= 3 { 1 } else { 0 },
                    1 => if cnt[2] >= 3 { 2 } else { 1 },
                    2 => if cnt[1] >= 1 && cnt[2] >= 1 { 2 } else { 0 },
                    _ => panic!()
                }
            }
        }
        v = w;
    }
    let mut wood = 0;
    let mut lumber = 0;
    for y in 1..51 as usize {
        for x in 1..51 as usize {
            if v[y][x] == 1 {
                wood += 1;
            } else if v[y][x] == 2 {
                lumber += 1;
            }
        }
    }
    wood * lumber
}

pub fn day18_2() -> i32 {
    let input = std::fs::read_to_string("data/day18.txt").unwrap();
    let mut v = vec![vec![0 as u8; 52]; 52];
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.as_bytes().iter().enumerate() {
            v[y + 1][x + 1] = match b {
                b'.' => 0,
                b'|' => 1,
                b'#' => 2,
                _ => panic!()
            }
        }
    }
    let dx = [-1 as i32, 0, 1, 1, 1, 0, -1, -1];
    let dy = [-1 as i32, -1, -1, 0, 1, 1, 1, 0];
    for iter in 1..1000 {
        let mut w = v.clone();
        for y in 1..51 as usize {
            for x in 1..51 as usize {
                let mut cnt = [0; 3];
                for i in 0..dx.len() {
                    let yy = (y as i32 + dy[i]) as usize;
                    let xx = (x as i32 + dx[i]) as usize;
                    if xx > 0 && yy > 0 && xx <= 50 && yy <= 50 {
                        cnt[v[yy][xx] as usize] += 1;
                    }
                }
                w[y][x] = match v[y][x] {
                    0 => if cnt[1] >= 3 { 1 } else { 0 },
                    1 => if cnt[2] >= 3 { 2 } else { 1 },
                    2 => if cnt[1] >= 1 && cnt[2] >= 1 { 2 } else { 0 },
                    _ => panic!()
                }
            }
        }
        v = w;
        if iter > 524 && (iter - 524) % 28 == (1000000000 - 524) % 28 {
            let mut wood = 0;
            let mut lumber = 0;
            for y in 1..51 as usize {
                for x in 1..51 as usize {
                    if v[y][x] == 1 {
                        wood += 1;
                    } else if v[y][x] == 2 {
                        lumber += 1;
                    }
                }
            }
            return wood * lumber;
        }
    }
    0
}