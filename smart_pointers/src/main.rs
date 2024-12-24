use std::{cell::RefCell, rc::Rc};

fn main() {
    let five = Box::new(10);
    let fang_borrow = Rc::new(RefCell::new(five));

    let per1 = Rc::clone(&fang_borrow);
    let per2 = Rc::clone(&fang_borrow);

    **per1.borrow_mut() += 5;
    **per2.borrow_mut() += 25;



    println!("{}", per1.borrow());
}
