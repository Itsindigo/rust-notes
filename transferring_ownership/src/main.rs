fn main() {
    let s1 = return_ownership();

    println!("s1: {}", s1);

    let s2 = take_and_give_back_ownership(s1);

    println!("s2: {}", s2)
}


fn return_ownership() -> String {
    String::from("Hello world")
}

fn take_and_give_back_ownership(str: String) -> String {
    str
}
