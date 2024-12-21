mod print;

struct Point {
    x: i32,
    y: i32,
}

enum Color{
    Red,
    Green,
    Blue,
}

fn main() {
    let p = Point{
        x: 15,
        y: 16,
    };

    println!("{}, {}", p.x, p.y);
     
    ////////////////////////////////////

    let red:Color = Color::Red;

    match red {
        Color::Red => println!("RED"),
        Color::Green => println!("GREEN"),
        Color::Blue => println!("BLUE"),
    }

    ///////////////////////////////////
    //Call fn from print.rs
    print::run();

    let mut word1 = String::from("Pencil");

    let word2 = &word1;

    let mut word3 = word2.to_string();

    word3.push_str(" Apple");

    println!("{:?}", word3);
}