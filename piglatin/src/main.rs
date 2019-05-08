use std::env;
use std::process;

fn main() {
    let words: Vec<String> = env::args()
        .skip(1)
        .collect();

    if words.len() == 0 {
        usage_and_exit("Error: You did not provide any words!");
    }

    for word in words {
        println!("{} => {}", word, to_pig_latin(&word));
    }
}

fn usage_and_exit(msg: &str) {
    eprintln!("{}\nUsage: piglatin [WORD ...]", msg);
    process::exit(2);
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
