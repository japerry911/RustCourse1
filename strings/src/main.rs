fn main() {
    let s:&'static str = "hello there";

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_character) = s.chars().nth(0) {
        println!("First Letter is - {}", first_character);
    }

    // Heap - String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{:?}", letters);

    // &str <> String
    //let u:&str = &letters;

    // Concatenation
    // String + str
    //let z = letters + &letters;
    let mut abc = "hello world".to_string();//String::from("hello world");
    abc.remove(0);
    abc.push_str("!!!!!!");
    println!("{}", abc.replace("ello", "goodbye"));

    println!("{}", abc);
}
