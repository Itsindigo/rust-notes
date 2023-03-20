
/* this is contrived example to illustrate how data can be stored on the heap.
Though in practice you would probably not store a single i32 using a box.
*/
fn main() {
    // b is allocated to the heap rather than the stack.
    let b = Box::new(5);
    println!("b = {}", b);
} // b goes out of scope and `Drop` is called like any var on stack.
