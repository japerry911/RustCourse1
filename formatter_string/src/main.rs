fn main() {
    let name = "Sky";
    let greeting = format!("hi, I'm {}, nice to meet you.", name);
    println!("{}", greeting);

    let run = "Run";
    let forest = "Forest";
    let rfr = format!("{0}, {1}, {0}!", run, forest);
    println!("{}", rfr);

    let info = format!("the name's {last}. {first} {last}.", first="James", last="Bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {} {data}", "alpha", "beta", data="delta");
    println!("{}", mixed);
}
