fn main() {
    let s = "hello_world";
    let mut mutable_string = String::from("hello");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! return &String 
    let s4 = format!("{s1}-{s2}-{s3}");

    coerce_success(&mutable_string);
    coerce_success(&s4);
    
    coerce_fail(s);
}

fn coerce_success(data: &str) { // compiles just fine, even though we put in a &String
    println!("{}", data);
}

fn coerce_fail(data: &String) { // error - expected &String, but found &str
    println!("{}", data);
}