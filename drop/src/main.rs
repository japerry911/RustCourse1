fn main() {
    let mut clever:Creature;
    {
        let goblin = Creature::new("Jeff".to_string());
        //drop(goblin);
        println!("Game Proceeds");
        //clever = goblin;
        println!("End of Scope");
    }
    println!("Game Ends");
}

struct Creature {
    name: String
}

impl Creature {
    fn new<S>(name: S) -> Creature where S: Into<String> {
        let into_value = name.into();
        println!("{} has entered the game", into_value);
        Creature { name: into_value }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}