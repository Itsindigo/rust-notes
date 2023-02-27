enum Message {
  Quit, // unit struct
  Move { x: i32, y: i32}, // named fields, not a struct
  Write(String), // tuple struct
  ChangeColor(i32, i32, i32), // tuple struct
}

// enums can impl just like structs
impl Message {
    fn call(&self) {
      println!("call function invoked");
    }
}

fn main() {
    let m = Message::Write(String::from("Hellooo")); // note that `m` is an instance of the Write variant, not a Message struct.
    m.call()
}
