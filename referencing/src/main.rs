fn main() {
    // OK
    // Create a mutable variable, pass a mutatable
    // reference, mutate the reference, observe
    // that the original value has changed.
    let mut s1 = String::from("hello");

    change(&mut s1);

    println!("{}", s1);


    // ERROR
    // if you create one mutable reference, you cannot create another.
    let r1 = &mut s1;
    let r2 = &s1; // causes error because mutable borrow has already occurred.
    
    println!("{}, {}", r1, r2);


}

// think &mut makes explicit that the original value
// will be mutated.
fn change(s: &mut String) {
    s.push_str(", world");
}