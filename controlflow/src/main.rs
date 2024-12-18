fn main() {
    //demo if-else
    let x = 5;

    if x == 5{
        println!("Hello Mr.Five!!!!!")
    }else if x > 0 {
        println!("x is positive");
    }else{
        println!("x is negative");
    }

    //demo match
    let enemy = "slime";

    match enemy {
        "goblin" => println!("Punch its fucking face!"),
        "slime" => println!("Use water blast"),
        "dragon" => println!("Run for cover"),
        _ => println!("I dont know what to do?"),
    }

    //demo loop
    let y = ["apple", "banana", "orange"];
    for i in y.iter(){
        println!("{}", i);
    }

    //demo loop 2
    let mut wood = 0;

    loop {
        wood+=1;
        println!("Nong fang has got {} woods to build her house.", wood);

        if wood == 10 {
            println!("!!Collect {} woods for the house success!!", wood);
            break;
        }
    }

    //demo while loop 
    let mut z = 0;
    while z < 5 {
        println!("z = {}", z);
        z+=1;
    }
}
