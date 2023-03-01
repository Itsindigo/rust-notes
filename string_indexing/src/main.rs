fn main() {
    let hello = "Здравствуйте";

    // UTF-8 characters have an underlying implementation
    // which requires 2 bytes (16 bits).

    // Therefore we can only slice from and to even digits else
    // we risk partially slicing characters.

    let s1 = &hello[0..4];
    println!("{}", s1);


    let s2 = &hello[10..14];
    println!("{}", s2);


    // causes kernel panic
    let s3 = &hello[11..15];
    println!("{}", s3);

    // causes kernel panic
    let s4 = &hello[0..3];
    println!("{}", s4);
}
