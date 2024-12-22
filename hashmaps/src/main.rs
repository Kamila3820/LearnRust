use::std::collections::HashMap;

fn main() {
    let mut items = HashMap::new();

    items.insert("Gold coin", 5);
    items.insert("Emerald", 3);

    // way to get value from HashMap
    if let Some(eme) = items.get("Emerald") {
        println!("Emerald: {}", eme);
    }

    //ex. x2 value inside the HashMap
    for (item, count) in items.iter_mut() {
        *count *= 2;
        println!("{}: {}", item, count);
    }

}
