fn main() {
    let mut my_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", my_list);

    // declares a mutable borrow.
    let mut only_borrows = || my_list.push(7);

    // would cause error trying to create a ref to variable
    // that is already borrowed as mutable
    // println!("Before calling closure: {:?}", list);

    {
        only_borrows();
    }
    println!("After calling closure: {:?}", my_list);
    
    // Calling only borrows again here would also cause a compilation error
    // this is because the closure retains ownership of the
    // list from the top level scope
    // only_borrows();

    // if we wanted the closure to take and release ownership
    // we need to use a mutable reference
    let mut another_list = vec![1,2,3];

    let modify_list = |list: &mut Vec<i32>| list.push(7);

    modify_list(&mut another_list);
    modify_list(&mut another_list);
    println!("After modifying list: {:?}", another_list);
}
