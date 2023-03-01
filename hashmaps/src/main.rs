use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    let copyable_field_name = "Age";
    let copyable_field_value = 29;

    
    // OK: these types can be used as they implement Copy
    map.insert(copyable_field_name, copyable_field_value);
    println!("{}: {}", copyable_field_name, copyable_field_value);


    // Heap allocated variables will have ownership
    // transferred once they are assigned to the HashMap.

    let field_name = String::from("Favourite colour");
    let field_value = String::from("Blue");
    
    let mut map: HashMap<String, String> = HashMap::new();

    // COMPLIATION ERROR:
    // ownership of key + value
    // are both transferred on insert
    
    map.insert(field_name, field_value);
    println!("{}: {}", field_name, field_value);
}
