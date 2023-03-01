fn main() {
    let v = vec![1, 2, 3, 4];

    // accessing at [index] returns the <T> type of the vector
    let third_int: &i32 = &v[2];
    println!("Third: {:?}", third_int);

    // accessing [index] with the .get() method returns an Option
    let third_as_option = v.get(2);

    match third_as_option {
        Some(third) => println!("Also Third: {}", third),
        None => println!("There is no third!"),
    }

    // what about invalid indexes

    // Causes a kernel panic!
    // let does_not_exist = &v[100];

    // Handles safely!
    let does_not_exist = v.get(100);

    match does_not_exist {
        Some(num) => println!("This should not run"),
        None => println!("There number did not exist!"),
    }

    /* Enums as union types. */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    /* Initialise vector with mixed typing via enum */
    let v: Vec<SpreadsheetCell> = Vec::from([
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]);

    println!("Enumerated vector: {:?}", v);
}
