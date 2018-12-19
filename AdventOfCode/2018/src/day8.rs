fn sum_metadata(data : &[u32]) -> (usize, u32) {
    let child_nodes = data[0];
    let metadata_entries = data[1] as usize;
    let mut offset : usize = 2;
    let mut total_sum : u32 = 0;
    for _ in 0..child_nodes {
        let (len, sum) = sum_metadata(&data[offset..]);
        offset += len;
        total_sum += sum;
    }
    (offset + metadata_entries, total_sum + data[offset..offset + metadata_entries].iter().sum::<u32>())
}

fn node_value(data : &[u32]) -> (usize, u32) {
    let child_nodes = data[0];
    let metadata_entries = data[1] as usize;
    let mut offset : usize = 2;
    if child_nodes == 0 {
        return (offset + metadata_entries, data[offset..offset + metadata_entries].iter().sum::<u32>());
    }
    
    let mut child_values = Vec::with_capacity(child_nodes as usize);
    for _ in 0..child_nodes {
        let (len, value) = node_value(&data[offset..]);
        offset += len;
        child_values.push(value);
    }    
    let mut result_value = 0;
    for &entry in &data[offset..offset + metadata_entries] {
        if entry > 0 && entry <= child_nodes {
            result_value += child_values[entry as usize - 1];
        }
    }
    (offset + metadata_entries, result_value)
}

pub fn day8_1() -> u32 {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    let data : Vec<u32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    sum_metadata(&data).1
}

pub fn day8_2() -> u32 {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    let data : Vec<u32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
    node_value(&data).1
}