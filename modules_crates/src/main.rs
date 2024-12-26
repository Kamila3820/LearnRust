use modules_crates::calculator;   //super::ใช้ใน folder เดียวกัน // crate::ใช้เรียกของข้าม folder
fn main() {

    let sum = calculator::add::add(10, 15);
    let sum2 = calculator::sub::sub(10, 5);
    println!("{}", sum);
    println!("{}", sum2);
}
