use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub fn day12_1() -> i32 {
    let input = std::fs::read_to_string("data/day12.txt").unwrap();
    let mut lines = input.lines();
    let init = lines.next().unwrap();
    const len : usize = 200;
    let mut f = ['.'; 2 * len];
    for (i, ch) in init.chars().enumerate() {
        f[i + len] = ch;
    }
    let rules : Vec<&str> = lines.collect();
    for _ in 0..20 {
        let mut next = ['.'; 2 * len];
        for i in 0..f.len() - 5 {
            'rules: for r in &rules {
                for (idx, ch) in r[0..5].chars().enumerate() {
                    if (ch != f[i + idx]) {
                        continue 'rules;
                    }
                }
                next[i + 2] = r.chars().nth(9).unwrap();
                break;
            }
        }
        f = next;
    }
    let mut result : i32 = 0;
    for (i, ch) in f.iter().enumerate() {
        if *ch == '#' {
            result += i as i32 - len as i32;
        }
    }
    result
}

pub fn day12_2() -> i64 {
    // let input = std::fs::read_to_string("data/day12.txt").unwrap();
    // let mut lines = input.lines();
    // let init = lines.next().unwrap();
    // const len : usize = 200;
    // let mut f = ['.'; 2 * len];
    // for (i, ch) in init.chars().enumerate() {
    //     f[i + len] = ch;
    // }
    // let rules : Vec<&str> = lines.collect();
    // let mut out = File::create("day12.out").unwrap();
    // let mut writer = BufWriter::new(&out);
    // for _ in 0..93 {
    //     let mut next = ['.'; 2 * len];
    //     for i in 0..f.len() - 5 {
    //         'rules: for r in &rules {
    //             for (idx, ch) in r[0..5].chars().enumerate() {
    //                 if ch != f[i + idx] {
    //                     continue 'rules;
    //                 }
    //             }
    //             next[i + 2] = r.chars().nth(9).unwrap();
    //             break;
    //         }
    //     }
    //     for i in 0..f.len() {
    //         write!(writer, "{}", f[i]);
    //     }
    //     writeln!(writer);
    //     f = next;
    // }
    // let mut result : i32 = 0;
    // for (i, ch) in f.iter().enumerate() {
    //     if *ch == '#' {
    //         result += i as i32 - len as i32;
    //     }
    // }
    let mut result2 : i64 = 0;
    let offset = 50000000000 as i64 - 93 + 6;
    for i in offset..offset + 186 {
        result2 += i;
    }
    result2
}