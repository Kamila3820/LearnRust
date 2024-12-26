trait Weapon {
    fn attack(&self);
}

struct Sword;
struct Scepter;
struct Orb;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack");
    }
}

impl Weapon for Scepter {
    fn attack(&self) {
        println!("Scepter power");
    }
}

impl Weapon for Orb {
    fn attack(&self) {
        println!("Orb healing");
    }
}

impl Warrior {
    fn new() -> Self {
        Self { 
                health: 100,
                strength: 10,
                intelligence: 5,
                weapon: Box::new(Sword),
        }
    }

    fn health_increase(&mut self, heal: u8) {
        if self.health + heal > 100 {
            self.health = 100;
            return
        }
        self.health += heal;
       
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }
}

struct Warrior {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self { 
                health: 100,
                strength: 4,
                intelligence: 10,
                weapon: Box::new(Scepter),
        }
    }

    fn health_increase(&mut self, heal: u8) {
        if self.health + heal > 100 {
            self.health = 100;
            return
        } 
        self.health + heal;
        
        
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }
}

struct Mage {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self { 
                health: 100,
                strength: 1,
                intelligence: 8,
                weapon: Box::new(Orb),
        }
    }

    fn health_increase(&mut self, heal: u8) {
        if self.health + heal > 100 {
            self.health = 100;
            return
        } 
        self.health += heal;
    }

    fn health_decrease(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);
    }
}

struct Healer {
    health: u8,
    strength: u8,
    intelligence: u8,
    weapon: Box<dyn Weapon>,
}

fn do_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

fn main() {
    let mut five = Warrior::new();
    let mut fang = Mage::new();
    let mut focus = Healer::new();

    five.health_decrease(50);
    println!("{}", five.health);

    five.health_increase(60);
    println!("{}", five.health);

    do_attack(five.weapon);
}
