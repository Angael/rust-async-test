pub fn string_ops() {
    println!("[String Operations]");

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let my_cut = &s;
    let _a = s.len();
    let _b = my_cut.len();

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
        print!("({})", c);
    }
    println!();
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn first_word() {
    println!("[First Word]");

    let s = String::from("One Two Three");
    let word = get_first_word(s.as_str());
    println!("First word: {}", word);

    let s = "Hi, Nice to meet you!";
    let word = get_first_word(&s);
    println!("First word: {}", word);
}
