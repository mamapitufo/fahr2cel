use std::env;
use std::process;

fn main() {
    let words: Vec<String> = env::args().skip(1).collect();

    if words.is_empty() {
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

// tries to follow the rules from https://en.wikipedia.org/wiki/Pig_Latin
fn to_pig_latin(word: &str) -> String {
    let (prefix, suffix) = split_word(word);

    if prefix.is_empty() {
        suffix + "way"
    } else {
        suffix + &prefix + "ay"
    }
}

fn split_word(word: &str) -> (String, String) {
    let mut prefix = String::new();
    let mut suffix = String::new();
    let mut is_prefix = true;

    // FIXME: this is incorrect if the word
    //   * begins with 'h<vowel>' as in 'honest'
    //   * begins with 'gui' as in 'guitar'
    for c in word.chars() {
        if is_prefix && is_vowel(c) {
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

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}
