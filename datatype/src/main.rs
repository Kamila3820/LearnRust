use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


fn main() {
    let name = "Atthapinya";
    println!("Hello Khun {}", name);

    let num = 10;
    println!("{}", num);

    let x:i32 = 100;
    let y:bool = true;
    let z:char = 'f';
    println!("x = {}, y = {}, z = {}", x, y, z);
    println!("type x = {}", type_of(&x))

}
