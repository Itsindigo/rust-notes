fn main() {
    let some_number = Some(5);
    // can also be pulled direct from std.
    // let some_number = std::option::Option::Some(5);

    let absent_number: Option<i32> = None;

    println!("{:?}", some_number); // note that `Some` is an enum variant, not a type/struct.

    println!("{:?}", absent_number); // prints a `None`.

    // ERROR: 
    // cannot add `Option<i32>` to `Option<{integer}>`
    // let sum = some_number + absent_number;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // ERROR:
    // Cannot do this as `y` is an Option type, not an int.
    // let sum = x + y;
    //             ^ no implementation for `i8 + Option<i8>`

    // We must first convert Option<i8> to an i8.
    // In this case we handle `Some` by returning the value,
    // And if `y` is a None type, then we return 0.
    let y = match y {
        Some(v) => v,
        None => 0,
    };

    let sum = x + y;

    println!("{}", sum);
}
