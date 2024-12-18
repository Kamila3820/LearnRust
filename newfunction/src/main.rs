fn add(x:i32, y:i32) -> i32 {
    x+y
}

fn crabby_task(task: &str, duration: i32) -> String {
    format!("Crabby has successfully done {} in {} minutes!", task, duration)
}

fn main() {
    println!("{}", add(10, 5));

    let crab = crabby_task("coding", 30);
    println!("{}", crab);
}
