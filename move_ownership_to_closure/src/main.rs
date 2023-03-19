use std::thread;

fn main() {
    let list = vec![1,2,3];

    // ownership transferred with `move` keyword
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    
    // if we removed `move`, we would get the follwing error:
    //     --> src/main.rs:6:19
    //     |
    //   6 |     thread::spawn(|| println!("From thread: {:?}", list))
    //     |                   ^^                               ---- `list` is borrowed here
    //     |                   |
    //     |                   may outlive borrowed value `list`
    //     |
    //   note: function requires argument type to outlive `'static`
    //    --> src/main.rs:6:5
    //     |
    //   6 |     thread::spawn(|| println!("From thread: {:?}", list))
    //     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //   help: to force the closure to take ownership of `list` (and any other referenced variables), use the `move` keyword
    //     |
    //   6 |     thread::spawn(move || println!("From thread: {:?}", list))
    //     |                   ++++
}
