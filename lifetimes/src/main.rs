fn main() {
    let x: i32 = 5;
    let y: i32 = 6;

    println!("{}", largest(&x, &y));

    //Crabby
    let map1 = "Ancient Map of Sea";
    let map2 = "New world Minecraft";

    let crabby = longest_map(&map1, &map2);
    println!("Crabby chose: {}", crabby);
}

fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    }else {
        y
    }
}

//Crabby'a
fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str{
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}
