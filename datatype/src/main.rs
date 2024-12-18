use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


fn main() {

    // mutable -> changable
    let mut num = 10;
    num = 5;
    println!("{}", num);

    // declare variable type string
    let msg1 = String::from("Hello Rust");
    let msg2 = "Hello Rust".to_string();
    let msg3 = "Atthapinya";
    let extra = format!("Mutable var: {}", num);


    println!("1. {}", extra);

    let x:i32 = 100;
    let y:bool = true;
    let z:char = 'f';
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("type x = {}", type_of(&x));  // Print type of that variable

    // cast variable using 'as'
    let num1= 10;
    let num2 = 5.5;

    let sum = num1 + num2 as i32;
    println!("cast result = {}", sum)


}
