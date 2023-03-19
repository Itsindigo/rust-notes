#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}


// This example will fail.
// The sort_by_key closure will only implement the `fn_once`
// trait because `sort_operations.push()` transfers ownership
// of `value` during the first iteration of sort_by_key.
// This transferral of ownership in the closure body
// means that the closure will only implement the `FnOnce` traits.
fn fn_once_trait_example() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);

}

// This example is valid.
// The `sort_by_key` function is defined to accept a `FnMut`
// closure. The reason for this is because it calls the closure
// once for each item in the slice. 
// The closure |r| r.width doesn’t capture, mutate, or move
// out anything from its environment, so it meets the trait
// bound requirements.
fn fn_mut_trait_example() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

// Unlike the `FnOnce` example above, this closure does
// not transfer ownership. The closure captures a mutable 
// reference to `num_sort_operations` which it can increment.
// But at no point is ownership transferred.
fn another_fn_mut_example() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
}