fn main() {
    let s = String::from("hello");  // s comes into scope


    // when a function takes a variable, it calls
    // a `move` operation.`
    // this invalidates the original `s` variable.
    takes_ownership(s); 

    // this line will cause an error because `s` has
    // moved and been invalidated.
    println!("{}", s);

    // x comes into scope
    let x = 5;

    makes_copy(x);

    // this is still valid, because `x` is an integer
    // which implements the `Copy` method. The `x`
    // passed to makes_copy is distinct on the stack
    // from the `x` we print here.
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
  // Note that it is the String type which determines this.
  // Because String is a mutable type, it is allocated to the heap.
  // Because it is allocated to the heap, `drop` is called on scope closure.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens. (popped from stack?)