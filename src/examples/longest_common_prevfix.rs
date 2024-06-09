pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut i: usize = 0;
    let first_word_char_count = strs[0].chars().count();

    while i < first_word_char_count {
        let new_letter = strs[0].chars().nth(i).unwrap();

        for my_str in &strs {
            match my_str.chars().nth(i as usize) {
                None => return strs[0][..i as usize].to_string(),
                _ => {
                    if my_str.chars().nth(i as usize).unwrap() != new_letter {
                        return strs[0][..i as usize].to_string();
                    }
                }
            }
            // if my_str.chars().nth(i as usize).unwrap() != new_letter {
            //     return strs[0][..i as usize].to_string();
            // }
        }

        i += 1;
    }

    return strs[0][..i as usize].to_string();
}
