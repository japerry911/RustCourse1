fn main() {
    operators();
}

fn operators() {
    let mut a:i32 = 2 + 3 * 4;

    println!("a = {}", a);

    a += 1;
    a -= 2;

    println!("a = {}",a);

    a %= 10;

    println!("a = {}", a);

    let a_cubed:i32 = i32::pow(a, 3);

    println!("a_cubed = {}", a_cubed);

    let b:f64 = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi:f64 = f64::powf(b, std::f64::consts::PI);

    println!("b = {}, b_cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    let c:i32 = 1 | 2; // | OR & AND ^ XOR ! NOR
                       // 01 OR 10 = 11 3_10

    println!("1|2 = {}", c);

    let two_to_10:i32 = 1 << 10;

    println!("2^10 = {}", two_to_10);

    let pi_less_4 = std::f64::consts::PI < 4.0;
}