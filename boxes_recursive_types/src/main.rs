use crate::List::{Cons, Nil};

/* Declaring in this way will cause a compilation error.
The enum is allocated to the stack, however because the enum
is recursive, it has a variable size

To avoid this we must use indirection

error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:3:1
  |
3 | enum List {
  | ^^^^^^^^^
4 |     Cons(i32, List),
  |               ---- recursive without indirection

We can use a `Box` to redirect the control to the heap
rather than the stack.

The box creates a fixed size reference to a pointer the
size of which can be known at compile time.
*/
// enum List {
//     Cons(i32, List),
//     Nil
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
