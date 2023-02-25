fn main() {
    let temp = farenheit_to_celsius(106);

    println!("The temperature in celsius is {temp}")
}


fn farenheit_to_celsius(f: i32) -> i32 {
    5 * (f - 32) / 9
}