struct Cart {
    x: usize, 
    y: usize, 
    dir: usize,
    cnt: i32,
    is_active: bool
}

pub fn day13_1() -> (usize, usize) {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let mut lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut carts = Vec::new();
    for (x, line) in lines.iter_mut().enumerate() {
        for (y, ch) in line.iter_mut().enumerate() {
            let dir = match ch {                
                b'^' => { *ch = b'|'; 0 },
                b'>' => { *ch = b'-'; 1 },
                b'v' => { *ch = b'|'; 2 },
                b'<' => { *ch = b'-'; 3 },
                _ => continue
            };
            carts.push(Cart { x, y, dir, cnt: 0, is_active : true});
        }
    }
    let dx : [i32; 4] = [-1, 0, 1, 0];
    let dy : [i32; 4] = [0, 1, 0, -1];
    loop {
        carts.sort_unstable_by(|a, b| (a.x, a.y).cmp(&(b.x, b.y)));    
        for i in 0..carts.len() {
            {
                let c = &mut carts[i];
                c.x = (c.x as i32 + dx[c.dir]) as usize;
                c.y = (c.y as i32 + dy[c.dir]) as usize;
                if lines[c.x][c.y] == b'+' {
                    c.dir = match c.cnt {
                        0 => (c.dir + 3) % 4,
                        1 => c.dir,
                        2 => (c.dir + 1) % 4,
                        _ => panic!("Unexpected cnt")
                    };
                    c.cnt = (c.cnt + 1) % 3;
                } else if lines[c.x][c.y] == b'/' {
                    let change = [1, 0, 3, 2];
                    c.dir = change[c.dir];
                } else if lines[c.x][c.y] == b'\\' {
                    let change = [3, 2, 1, 0];
                    c.dir = change[c.dir];                    
                }
            }
            for j in 0..carts.len() {
                if i != j && carts[i].x == carts[j].x && carts[i].y == carts[j].y {
                    return (carts[i].y, carts[i].x);
                }
            }
        }
    }
}

pub fn day13_2() -> (usize, usize) {
    let input = std::fs::read_to_string("data/day13.txt").unwrap();
    let mut lines: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let mut carts = Vec::new();
    for (x, line) in lines.iter_mut().enumerate() {
        for (y, ch) in line.iter_mut().enumerate() {
            let dir = match ch {                
                b'^' => { *ch = b'|'; 0 },
                b'>' => { *ch = b'-'; 1 },
                b'v' => { *ch = b'|'; 2 },
                b'<' => { *ch = b'-'; 3 },
                _ => continue
            };
            carts.push(Cart { x, y, dir, cnt: 0, is_active: true});
        }
    }
    let dx : [i32; 4] = [-1, 0, 1, 0];
    let dy : [i32; 4] = [0, 1, 0, -1];
    'iteration: loop {
        carts.sort_unstable_by(|a, b| (a.x, a.y).cmp(&(b.x, b.y)));    
        for i in 0..carts.len() {
            {
                let c = &mut carts[i];
                if !c.is_active {
                    continue;
                }
                c.x = (c.x as i32 + dx[c.dir]) as usize;
                c.y = (c.y as i32 + dy[c.dir]) as usize;
                if lines[c.x][c.y] == b'+' {
                    c.dir = match c.cnt {
                        0 => (c.dir + 3) % 4,
                        1 => c.dir,
                        2 => (c.dir + 1) % 4,
                        _ => panic!("Unexpected cnt")
                    };
                    c.cnt = (c.cnt + 1) % 3;
                } else if lines[c.x][c.y] == b'/' {
                    let change = [1, 0, 3, 2];
                    c.dir = change[c.dir];
                } else if lines[c.x][c.y] == b'\\' {
                    let change = [3, 2, 1, 0];
                    c.dir = change[c.dir];                    
                }
            }
            for j in 0..carts.len() {
                if i != j && carts[j].is_active && carts[i].x == carts[j].x && carts[i].y == carts[j].y {
                    carts[i].is_active = false;
                    carts[j].is_active = false;
                }
            }
        }
        let mut result: Option<(usize, usize)> = None;
        for cart in &carts {
            if cart.is_active {
                result = match result {
                    None => Some((cart.y, cart.x)),
                    Some(_) => continue 'iteration
                }
            }
        }
        return result.unwrap();
    }
}