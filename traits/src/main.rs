struct Inventory<T> {
    item: T
}

trait DisplayItem {
    fn display_item(&self);
}

// To print generic type -> use std::fmt::Debug  && {:?}
impl<T: std::fmt::Debug> DisplayItem for Inventory<T> {
    fn display_item(&self) {
        println!("I have a {:?}", self.item);
    }
}

fn main() {
    let crabby = Inventory {
        item: 100,
    };

    crabby.display_item();
}
