fn main() {
    let items = vec![100, 200, 300, 400];

    let crabby: Vec<i32> = items.iter().map(|x| x*2).collect();

    println!("{:?}", crabby);
}
