fn main() {
    //demo if-else
    let x = 5;

    if x > 0{
        println!("x is positive");
    }else{
        println!("x is negative");
    }

    //demo loop
    let y = ["apple", "banana", "orange"];
    for i in y.iter(){
        println!("{}", i);
    }

    //demo while loop 
    let mut z = 10;
    while z < 30 {
        println!("z = {}", z);
        z+=1;
    }
}
