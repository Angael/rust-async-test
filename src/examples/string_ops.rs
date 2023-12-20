pub fn string_ops() {
    println!("[String Operations]");

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let my_cut = &s;
    my_cut.len();

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{}", s3);

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}", s4, s5, s6);
    println!("{}", s7);

    let hello = "Здравствуйте";
    let s8 = &hello[0..4];
    println!("{}", s8);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
