fn main() {
    let s1 = String::from("abc");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);

    println!("The longest string is {}", result);
}

// This uses named lifetime, it couples the variables stating that 'a will remain valid for as long as both references are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
