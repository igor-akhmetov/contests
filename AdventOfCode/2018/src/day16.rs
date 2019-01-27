type State = [i32; 4];

pub fn parse_state(s: &str) -> State {
    let v: Vec<i32> = s.split(", ").map(|s| s.parse::<i32>().unwrap()).collect();
    [v[0], v[1], v[2], v[3]]
}

pub fn addr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] + st[b as usize];
}

pub fn addi(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] + b;
}

pub fn mulr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] * st[b as usize];
}

pub fn muli(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] * b;
}

pub fn banr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] & st[b as usize];
}

pub fn bani(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] & b;
}

pub fn borr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] | st[b as usize];
}

pub fn bori(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = st[a as usize] | b;
}

pub fn setr(st: &mut State, a: i32, _b: i32, c: i32) {
    st[c as usize] = st[a as usize];
}

pub fn seti(st: &mut State, a: i32, _b: i32, c: i32) {
    st[c as usize] = a;
}

pub fn gtir(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (a > st[b as usize]) as i32;
}

pub fn gtri(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (st[a as usize] > b) as i32;
}

pub fn gtrr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (st[a as usize] > st[b as usize]) as i32;
}

pub fn eqir(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (a == st[b as usize]) as i32;
}

pub fn eqri(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (b == st[a as usize]) as i32;
}

pub fn eqrr(st: &mut State, a: i32, b: i32, c: i32) {
    st[c as usize] = (st[a as usize] == st[b as usize]) as i32;
}

pub fn day16() -> (usize, i32) {
    let input = std::fs::read_to_string("data/day16.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let all_ops = [addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr];
    //let mut possible_opcodes = [[true; 16]; 16];
    let mut possible_opcodes = [[false; 16]; 16];
    let mut impossible_opcodes = [[false; 16]; 16];
    let mut idx = 0;
    let mut cnt = 0;    
    while lines[idx].len() > 0 {
        let before = parse_state(&lines[idx][9..lines[idx].len() - 1]);
        let args : Vec<i32> = lines[idx + 1].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let after = parse_state(&lines[idx + 2][9..lines[idx + 2].len() - 1]);
        let mut ops = 0;
        for (opidx, op) in all_ops.iter().enumerate() {
            let mut st = before.clone();
            op(&mut st, args[1], args[2], args[3]);
            if st == after {
                ops += 1;
                possible_opcodes[opidx][args[0] as usize] = true;
            } else {
                impossible_opcodes[opidx][args[0] as usize] = true;
            }
        }
        if ops >= 3 {
            cnt += 1;
        }
        idx += 4;
    }
    let mut opcodes = [-1 as i32; 16];
    let mut rev_opcodes = [-1 as i32; 16];
    loop {
        let mut found = false;
        for i in 0..16 {
            if opcodes[i] != -1 {
                continue;
            }
            let mut idx : i32 = -1;
            for j in 0..16 {
                if !possible_opcodes[i][j] || impossible_opcodes[i][j] {
                    continue;
                }
                if idx == -1 {
                    idx = j as i32;
                } else {
                    idx = -2;
                }
            }
            if idx >= 0 {
                found = true;
                opcodes[i] = idx;
                rev_opcodes[idx as usize] = i as i32;
                for j in 0..16 {
                    possible_opcodes[j][idx as usize] = false;
                }
            }
        }
        if !found {
            break;
        }
    }
    idx += 2;
    let mut state = [0; 4];
    for line in &lines[idx..] {
        let args : Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        all_ops[rev_opcodes[args[0] as usize] as usize](&mut state, args[1], args[2], args[3]);
        idx += 4;
    }
    (cnt, state[0])
}