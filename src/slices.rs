pub fn main() {
    {
        fn first_word(words: &String) -> &str {
            let bytes = words.as_bytes();
            for (i, &c) in bytes.iter().enumerate() {
                if c == b' ' {
                    return &words[..i];
                }
            }
            return &words[..];

        }
        let s = String::from("silky smooth chocolate");
        let first_word = first_word(&s);
        println!("{first_word:?}");
    }

    {
        let s = String::from("fda fdsla fjd");
        let p = &s[0..3];
        dbg!(p);
    }
}

