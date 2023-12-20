pub fn clojoures() {
    println!("[Clojoures]");

    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = |x| list.push(x);

    borrows_mutably(33);
    borrows_mutably(421);
    println!("After calling closure: {:?}", list);
}
