fn main() {
    let s1 = to_pig_latin(&"Здравствуйте");
    println!("{}", s1);

    let s2 = to_pig_latin(&"first");
    println!("{}", s2);

    let s3 = to_pig_latin(&"apple");
    println!("{}", s3);

    let s4 = to_pig_latin(&"");
    println!("{}", s4);

}


fn to_pig_latin(s: &str) -> String {
    // dont index like this, as it will be inconsistent based
    // on underlying unicode characters.
    // let prefix = &s[2..];
    // let suffix = &s[0..2];
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if s.len() == 0 {
        return String::new();
    }

    if vowels.iter().any(|&v| s.starts_with(v)) {
        // vowel path
        return format!("{s}-hay");
    } else {
        // non vowel path
        let first_character = s.chars().take(1).last().unwrap();
        let mut rest_of_string = String::new();
        for (i, c) in s.chars().enumerate() {
            if i == 0 {
                continue
            };
            rest_of_string.push(c)
        }
        return format!("{rest_of_string}-{first_character}ay");
    }
}