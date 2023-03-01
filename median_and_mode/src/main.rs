use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 9, 3, 7, 10, 2, 1, 7, 7, 2, 8, 9, 1, 10, 10, 10, 10];

    vec.sort();

    let med = median(&vec);
    let mode_val = mode(&vec);

    println!("median: {}, mode: {:?}", med, mode_val);

    let empty_vec_case: Vec<u32> = vec![];

    let med = median(&empty_vec_case);
    let mode_val = mode(&empty_vec_case);
    println!("median: {}, mode: {:?}", med, mode_val);
}


fn median(sorted_v: &Vec<u32>) -> u32 {
    let len = sorted_v.len();

    if len == 0 {
        return 0;
    }

    if len % 2 == 0 {
        let mid_point = len / 2;
        let avg = (sorted_v[mid_point] + sorted_v[mid_point - 1]) / 2;
        avg
    } else {
        let index = (len + 1) / 2;
        sorted_v[index]
    }
}

fn mode(sorted_v: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();

    for i in sorted_v {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mode = map.iter().max_by_key(|&(k, _v)| k);

    match mode {
        None => 0,
        Some(i) => **i.0
    }
}