fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&number_list);

    println!("The largest number is {}", largest_number);
}


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}