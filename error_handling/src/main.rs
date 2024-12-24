fn open_chest(is_empty: bool) -> Option<String> {
    if is_empty {
        None
    } else {
        Some("You found a treasure".to_string())
    }
}

fn open_door(is_danger: bool) -> Result<String, String>  {
    if is_danger {
        Err("Wrong door!!".to_string())
    } else {
        Ok("Heavens door".to_string())
    }
}


fn main() {
    let chest = match open_chest(false) {
        Some(value) => value,
        None => "Nothings here".to_string()
    };

    println!("{}", chest);

    let door = match open_door(true) {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };

    println!("{}", door);
}
