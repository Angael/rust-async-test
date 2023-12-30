fn increase(x: &mut u8) {
    *x += 1;
}

pub fn run() {
    println!("[Mutable ref]");
    let mut x: u8 = 0;
    increase(&mut x);
    increase(&mut x);
    println!("x: {}", x);
}
