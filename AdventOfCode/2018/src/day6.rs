pub fn day6_1() -> usize {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    let mut points: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.trim().split(",").collect();
        points.push((parts[0].trim().parse().unwrap(), parts[1].trim().parse().unwrap()));
    }
    let mut count = vec![0; points.len()];
    let mut is_infinite = vec![false; points.len()];
    let left = -200;
    let right = 600;
    for x in left..right {
        for y in left..right {
            let mut best_dist = std::i32::MAX;
            let mut best_pt = 0;
            let mut on_border = false;
            for (idx, pt) in (&points).iter().enumerate() {
                let dist = (x - pt.0).abs() + (y - pt.1).abs();
                if dist == best_dist {
                    on_border = true;
                } else if dist < best_dist {
                    best_dist = dist;
                    on_border = false;
                    best_pt = idx;
                }
            }
            if !on_border {
                count[best_pt] += 1;
                if x == left || x == right -1 || y == left || y == right - 1 {
                    is_infinite[best_pt] = true;
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..points.len() {
        if !is_infinite[i] {
            result = std::cmp::max(result, count[i]);
        }
    }
    result
}

pub fn day6_2() -> usize {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    let mut points: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.trim().split(",").collect();
        points.push((parts[0].trim().parse().unwrap(), parts[1].trim().parse().unwrap()));
    }
    let mut result = 0;
    let left = -200;
    let right = 600;
    for x in left..right {
        for y in left..right {
            let mut total_dist = 0;
            for pt in &points {
                let dist = (x - pt.0).abs() + (y - pt.1).abs();
                total_dist += dist;
            }
            if total_dist < 10000 {
                result += 1;
            }
        }
    }
    result
}