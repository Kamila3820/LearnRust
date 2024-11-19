fn main() {
    let msg = String::from("Can I borrow");
    take_ownership(&msg);
}

fn take_ownership(s: &String) {
    println!("{}", s);
}
