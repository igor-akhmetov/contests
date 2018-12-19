fn react(input: &str) -> usize {
    let mut result = Vec::new();
    for ch in input.chars() {
        match result.pop() {
            None => {
                result.push(ch)
            },
            Some(last_ch) => {
                if ch.is_lowercase() != last_ch.is_lowercase() && ch.to_lowercase().eq(last_ch.to_lowercase()) {
                    continue;
                }
                result.push(last_ch);
                result.push(ch);
            }
        }
    }
    result.len()
}

pub fn day5_1() -> usize {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();    
    react(input.trim())
}

pub fn day5_2() -> usize {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();    
    let input = input.trim();
    let mut shortest = std::usize::MAX;
    for b in b'a'..b'z' + 1 {
        let ch = b as char;
        let s : String = input.chars().filter(|c| !c.to_lowercase().eq(ch.to_lowercase())).collect();
        shortest = std::cmp::min(shortest, react(&s));
    }
    shortest
}