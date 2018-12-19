pub fn day14_1() -> String {
    let input = 260321;
    let mut v : Vec<u8> = vec![3, 7];
    let (mut i, mut j) = (0, 1);
    while v.len() < input + 10 {
        let mut s = v[i] + v[j];
        if s >= 10 {
            v.push(1);
            s -= 10;
        }
        v.push(s);
        i = (i + 1 + v[i] as usize) % v.len();
        j = (j + 1 + v[j] as usize) % v.len();
    }
    let mut s = String::new();
    for x in &v[input..input + 10] {
        s.push((x + b'0') as char);
    }
    s
}

fn ends_with(v : &Vec<u8>, w : &Vec<u8>) -> bool {
    for i in 0..w.len() {
        let j = v.len() - i;
        if j == 0 {
            return false;
        }
        if v[j - 1] != w[i] {
            return false;
        }
    }
    return true;
}

pub fn day14_2() -> usize {
    let mut input : usize = 260321;
    let mut input_v = vec![];
    while input > 0 {
        input_v.push((input % 10) as u8);
        input /= 10;
    }
    let mut v : Vec<u8> = vec![3, 7];
    let (mut i, mut j) = (0, 1);
    loop {
        let mut s = v[i] + v[j];
        if s >= 10 {
            v.push(1);
            if ends_with(&v, &input_v) {
                return v.len() - input_v.len();
            }
            s -= 10;
        }
        v.push(s);
        if ends_with(&v, &input_v) {
            return v.len() - input_v.len();
        }
        i = (i + 1 + v[i] as usize) % v.len();
        j = (j + 1 + v[j] as usize) % v.len();
    }
}