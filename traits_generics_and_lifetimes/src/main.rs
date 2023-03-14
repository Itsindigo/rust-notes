use std::fmt::Display;


// we use angle brackets to first declare a named lifetime parameter 'a, and then a type parameter T
// NOTE: lifetime parameters are always declared before type parameters.
// fn longest_with_an_announcement<T, 'a> would cause a compilation error for example.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    // Trait bound syntax specifies T implements Display, therefore it can e printed.
    println!("Announcement! {}", ann);

    // The return of `x` or `y` is ambigious, therefore a named lifetime parameter is required
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "hello";
    let y = String::from("world");
    let ann = format!("Comparing {} and {}", x, y);
    let result = longest_with_an_announcement(x, &y, ann);

    println!("The longest is {}", result);
}
