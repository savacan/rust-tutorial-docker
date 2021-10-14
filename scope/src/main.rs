fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    change_num(&mut s1);
    let idx = first_word(&s1);
    println!("The length of '{}' is {}. {}", s1, len, idx);

    let s2 = String::from("hello world");
    let word = first_word_slice(&s2);

    println!("fist word is {}", word);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_num(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
