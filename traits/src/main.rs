use std::fmt::Debug;

fn main() {
    traits();
}

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk.", self.name());
    }
}

fn test(animal: impl Animal + Debug) {
    println!("{:?}", animal);
    println!("{}", animal.name());
}

fn test2<T: Animal + Debug>(animal: T, animal2: T) {
    println!("{:?}", animal);
    println!("{:?}", animal2);
    println!("{}", animal.name());
    println!("{}", animal2.name());
}

fn test3<T>(animal: T) where T: Animal + Debug {
    println!("{:?}", animal);
    println!("{}", animal.name());
}

#[derive(Debug, Clone, Copy)]
struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello.", self.name());
    }
}

#[derive(Debug, Clone, Copy)]
struct Cat {
    name: &'static str
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for val in self {
            result += *val;
        }
        result
    }
}

fn traits() {
    let h = Human::create("Sky");
    h.talk();

    let c = Cat::create("Garfield");
    c.talk();

    let h2:Human = Animal::create("Tom");
    h2.talk();

    let a = vec![1, 2, 3];
    println!("{}", a.sum());

    test(h);
    test2(h, h2);
    test3(h);
    test3(c);
}