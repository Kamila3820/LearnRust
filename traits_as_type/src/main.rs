trait Gear {
    fn use_gear(&self);
}

struct Sword;
struct Bow;
struct Potion;

impl Gear for Sword {
    fn use_gear(&self) {
        println!("Swing Sword");
    }
}

impl Gear for Bow {
    fn use_gear(&self) {
        println!("Snow Bow");
    }
}

impl Gear for Potion {
    fn use_gear(&self) {
        println!("Health Potion");
    }
}

fn get_gear<T: Gear>(item: T) {
    item.use_gear();
}

fn main() {
    let five = Sword;
    let fang = Bow;
    let focus = Potion;

    get_gear(five);
    get_gear(fang);
    get_gear(focus);
}
