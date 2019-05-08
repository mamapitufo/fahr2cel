fn main() {
    let word = String::from("aardvark");
    // let word = String::from("bee");
    // let word = String::from("crocodile");

    println!("{} => {}", word, to_pig_latin(&word));
}

fn to_pig_latin(word: &String) -> String {
    let (prefix, suffix) = split_word(&word);

    if prefix.len() == 0 {
        suffix + "way"
    } else {
        suffix + &prefix + "ay"
    }
}

fn split_word(word: &String) -> (String, String) {
    let mut prefix = String::new();
    let mut suffix = String::new();
    let mut is_prefix = true;

    // FIXME: this should also check if the first letter is 'h'
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
