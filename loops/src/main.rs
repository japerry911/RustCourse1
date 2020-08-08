fn main() {
    //while_loop();
    while_loop();
    loop_loop();
}

fn while_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        println!("{}", x);
    }
}

fn loop_loop() {
    let mut x = 1;

    loop {
        if x == 3 {
            x += 1;
            continue;
        }

        if x >= 10 {
            break;
        }

        println!("{}", x);
        x += 1;
    }
}