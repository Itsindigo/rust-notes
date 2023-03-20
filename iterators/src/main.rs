fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // iterators are _lazy_, and definition of an iterator
    // does not trigger the execution.
    let v1_iter = v1.iter().map(|x| x + 1);

    // in order to execute the above logic we must _consume_
    // the iterator
    let v2: Vec<i32> = v1_iter.collect();

    println!("{:?}", v2);
}
