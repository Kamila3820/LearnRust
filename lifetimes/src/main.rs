fn main() {
    let x: i32 = 5;
    let y: i32 = 6;

    println!("{}", largest(&x, &y));
}

fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    }else {
        y
    }
}
