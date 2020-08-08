use std::mem;

fn main() {
    let mut a = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 1;

    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    } else {
        println!("does match");
    }

    let b = [1u16; 10];
    for i in 0..b.len() {
        println!("{}", i);
    }

    println!("b takes up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for c in 0..mtx[i].len() {
            if i == c {
                println!("mtx[{}][{}] = {}", i, c, mtx[i][c]);
            }
        }
    }
}
