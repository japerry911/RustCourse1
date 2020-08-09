fn main() {
    vectors();
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);

    println!("{:?}", a);
    println!("a[0] = {}", a[0]);

    let idx:usize = 0;

    println!("a[0] = {}", a[idx]);

    // Panics - Out of Index error
    /*let idx2:usize = 100;
    println!("a[100] = {}", a[idx2]);*/

    // Option
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("No such element, error")
    }

    match a.get(3) {
        Some(x) => println!("a[3] = {}", x),
        None => println!("No such element, error")
    }

    for x in &a {
        println!("{}", x);
    }

    for x in 0..a.len() {
        println!("{} - {}", x, a[x]);
    }

    let last_element = a.pop(); // Option

    println!("{:?}", a);
    println!("LAST_ELEMENT_POPPED - {:?}", last_element);

    match last_element {
        Some(x) => println!("LAST_ELEMENT - {}", x),
        None => println!("No Last Element!")
    }

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
