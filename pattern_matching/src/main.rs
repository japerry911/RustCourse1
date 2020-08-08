fn main() {
    pattern_matching();
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (231230, 5);

    match point {
        //(_, y) => println!("(?, {})", y),
        //(.., y) => println!("(?, {})", y),
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("({}, {})", x, y )
    }
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        _ if (x % 2 == 0) => "some",
        _z @ 9..=11 => "lots of",
        12 => "dozen",
        _ => "a few"
    }
}
