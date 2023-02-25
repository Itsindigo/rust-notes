fn main() {
    let s = String::from("Hello World");
    let x = first_word(&s);

    println!("{}", x);

    let s2 = String::from("Foo Bar Baz");
    let y = second_word(&s2);

    println!("{}", y);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return slice of 0th to first space characters.
        }
    }

    &s[..] // whole string is one word
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    let mut counter = 0;
    let mut last_space_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            counter += 1;
            if counter == 2 {
                return &s[last_space_index + 1..i]; // return slice of 0th to first space characters.
            }
            last_space_index = i;
        }
    }

    &s[..] // whole string is one word
}

