use std::mem;

fn main() {
    let a:u8 = 123; // 8 bits || 1 byte || unsigned
    let b:i8 = 123; // 8 bits || 1 byte || signed

    println!("a = {} || b = {}", a, b);

    //  a = 456; - Compile Error - Immutable

    let mut c:i8 = 0; // mutable

    println!("c = {}", c);

    c = 100;

    println!("c = {}", c);

    let mut d:i32 = 123456789; // 32-bit signed i32

    println!("d = {}, size of d = {} bytes", d, mem::size_of_val(&d));

    d = -1;

    println!("d = {}", d);

    // i8 u8 i16 i32 u32 i64 u64

    let z:isize = 123; // isize / usize
    let size_of_z:usize = mem::size_of_val(&z);

    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let e:char = 'x';

    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let f:f32 = 2.5;

    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    let g:bool = false;

    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
