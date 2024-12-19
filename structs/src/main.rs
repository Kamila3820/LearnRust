struct Crabby {
    name: String,
    skill_level: u32,
    damage: i32,
    carrer: String,
}

struct Character {
    name: String,
    health: u8,  //max 100
}

// implement method for struct
impl Character {
    fn healing(&mut self, health: u8) {
        if self.health+health >= 100 {
            self.health = 100;
            return;
        }
        self.health += health;
    }

    fn take_damage(&mut self, damage: u8) {
        self.health = self.health.saturating_sub(damage);  // protect overflow becuz u8 cannot < 0
    }
}

fn main() {
    let mut crabby = Crabby {
        name: String::from("Crabby"),  //heap -> move owner 
        skill_level: 12,
        damage: 500,
        carrer: String::from("Chef"),
    };

    //copy field using ..
    let new_crabby = Crabby {
        name: String::from("New Crabby"),
        carrer: crabby.carrer.clone(),
        ..crabby     //carrer is a heap, ..crabby is move owner, not copy
    };


    println!("{}", new_crabby.carrer);
    println!("{}", crabby.carrer);

    //////////////////////////////////////////////////////
    
    let mut character1 = Character {
        name: String::from("Kamila"),
        health: 100,
    };

    character1.take_damage(50);
    println!("Hit by fang, life: {}", character1.health);

    character1.take_damage(100);
    println!("Hit by fang, life: {}", character1.health);

    character1.healing(10);
    println!("Heal portion, life: {}", character1.health);

}
