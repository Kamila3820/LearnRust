fn main() {
    let msg = String::from("Can I borrow");
    take_ownership(&msg);

    // Crabby
    let mut treasure = String::from("Gold");
    let focus = &treasure;
    let fang = &treasure;

    println!("Focus borrow my: {}", focus);
    println!("Fang borrow my: {}", fang);

    // mut can let other borrow only 1 person/time because if more than 1, it can cause a conflict!
    let trusted_friend = &mut treasure;
    trusted_friend.push_str(" and diamond");

    println!("Trusted friends updates to: {}", trusted_friend);
}

fn take_ownership(s: &String) {
    println!("{}", s);
}
