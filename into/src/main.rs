fn main() {
    // let sky = Person::new("Sky");
    //
    // let name:String = "Tom".to_string();
    // let tom = Person::new(name.as_ref());

    let sky = Person::new("Sky");
    let sky2 = Person::new("Sky2".to_string());
    println!("{}", sky.name());
    println!("{}", sky2.name());
}

struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string() }
    // }
    // fn new<S: Into<String>>(name: S) -> Person {
    //     Person { name: name.into() }
    // }
    fn new<S>(name: S) -> Person where S: Into<String> {
        Person { name: name.into() }
    }

    fn name(self) -> String {
        self.name
    }
}
