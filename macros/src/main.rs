macro_rules! say_yes {
    (yes) => {
        println!("Yes");
    };
    ($name:expr) => {
        println!("Nah, {}", $name);
    };
}

fn main() {
    say_yes!(yes);
}
