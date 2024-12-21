fn main() {
    let mut items: Vec<String> = Vec::new();
    // let mut greet = vec!["Hi", "Hello"];
    // if push the item to greet, the capacity will add more x2 which is 6
    // because rust will provide more when you add the item that more than the current capacity


    items.push(String::from("Gojizo"));
    items.push(String::from("Chemy"));
    items.push(String::from("Raise Buckle"));
    items.push(String::from("ViStamp"));

    items.remove(0);
    items.pop();
    items.push(String::from("Sun")); // capacity still 4

    println!("Length: {}", items.len());
    println!("Capacity: {}", items.capacity());
    println!("{:?}", items);

    for i in items.iter() {
        println!("{}", i);
    }
}
