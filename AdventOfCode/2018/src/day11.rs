const size : usize = 300;
const serial : usize = 7689;

fn calc_power() -> Vec<Vec<i32>> {    
    let mut power : Vec<Vec<i32>> = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            let rack_id = i + 11;
            let p = rack_id * (j + 1);
            let p = p + serial;
            let p = p * rack_id;
            let p = (p / 100 % 10) as i32 - 5;
            power[i][j] = p;
        }
    }
    power
}

pub fn day11_1() -> (usize, usize) {
    let power = calc_power();
    let mut best_sum = -90;
    let mut best_square = (0, 0);
    for x in 0..size - 3 {
        for y in 0..size - 3 {
            let mut cur_sum = 0;
            for i in 0..3 {
                for j in 0..3 {
                    cur_sum += power[x + i][y + j];
                }
            }
            if cur_sum > best_sum {
                best_sum = cur_sum;
                best_square = (x + 1, y + 1);
            }
        }
    }
    best_square
}

pub fn day11_2() -> (usize, usize, usize) {
    let power = calc_power();
    let mut best_sum = std::i32::MIN;
    let mut best_square = (0, 0, 0);
    let mut sums : Vec<Vec<i32>> = vec![vec![0; size]; size];
    for len in 1..size + 1 {
        for x in 0..size {
            let mut cur_sum = 0;
            for y in 0..size {
                cur_sum += power[x][y];
                if y + 1 >= len {
                    let i = y + 1 - len;
                    sums[x][i] = cur_sum;
                    cur_sum -= power[x][i];
                }
            }
        }
        for y in 0..size - len {
            let mut cur_sum = 0;
            for x in 0..size {                
                cur_sum += sums[x][y];
                if x + 1 >= len {
                    if cur_sum > best_sum {
                        best_sum = cur_sum;
                        best_square = (x + 2 - len, y + 1, len);
                    }
                    cur_sum -= sums[x + 1 - len][y];
                }
            }
        }
    }    
    best_square
}