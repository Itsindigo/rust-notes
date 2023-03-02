use std::collections::HashMap;

fn main() {
    let mut vec = vec![1, 9, 3, 7, 10, 2, 1, 7, 7, 2, 8, 9, 1, 1, 1, 1, 1];

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

    // max by key returns an option, where the value the option is:
    // Some((key, value)) it's a some tuple. Would be nicer if we could just return Some(key) here.
    // None

    // the call back function is the criteria for calculating 
    // the comparison value, if the comparsion value is greatest,
    // then the K,V pair will be returned as an option

    // if we wanted to destructure the tuple in the function closure:
    //.max_by_key(|&(k, v)| k);
    let mode = map.into_iter().max_by_key(|x| x.1);
    

    // handle the option, None for when no max value was found (empty vector)
    match mode {
        Some(i) => {
            println!("{:?}", i);
            *i.0 // Double ref seems to be because max_by_key() + iter() both return references, creating two layers.
        },
        None => 0,
    }
}
