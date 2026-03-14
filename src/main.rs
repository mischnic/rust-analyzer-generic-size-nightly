use std::num::NonZero;

pub struct Test<K, T = u8> {
    pub k: K,
    pub t: T,
}

// "needs Drop"
pub struct X {
    pub a: Vec<u32>,     // "needs Drop"
    pub b: NonZero<i32>, // "no Drop"
    pub c: u32,          // "size = 4, align = 0x4, no Drop"
    pub d: Test<i32>,    // "size = 8, align = 0x4, no Drop"
}

fn main() {
    // "size = 8, align = 0x4, no Drop"
    let _x = Test { k: 42, t: 255 };

    // "no Drop"
    let _y = std::num::NonZero::new(1i32).unwrap();
    println!("Hello, world!");
}
