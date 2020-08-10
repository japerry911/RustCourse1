fn main() {
    closures();
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        let b = 7;
        println!("{} + 2 = {}", b, plus_two(b));
    }

    // let borrow_two = &two;

    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    println!("f = {}", f);
    plus_three(&mut f);
    println!("f = {}", f);
}

fn say_hello() {
    println!("Hello");
}
