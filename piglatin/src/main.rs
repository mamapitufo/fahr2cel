fn main() {
    // let word = String::from("aardvark");
    // let word = String::from("bee");
    let word = String::from("crocodile");

    let (prefix, suffix) = split_word(&word);

    println!("word: {}, prefix: {}, suffix: {}", word, prefix, suffix);
}

fn split_word(word: &String) -> (String, String) {
    let mut prefix = String::new();
    let mut suffix = String::new();
    let mut is_prefix = true;

    for c in word.chars() {
        if is_prefix && is_vowel(&c) {
            is_prefix = false;
        }

        if is_prefix {
            prefix.push(c);
        } else {
            suffix.push(c);
        }
    }

    (prefix, suffix)
}

fn is_vowel(c: &char) -> bool {
    *c == 'a' || *c == 'e' || *c == 'i' || *c == 'o' || *c == 'u'
}
