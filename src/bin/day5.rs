//use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    assert!(is_nice2(&String::from("qjhvhtzxzqqjkmpb")));
    assert!(is_nice2(&String::from("xxyxx")));
    assert!(!is_nice2(&String::from("uurcxstgmygtbstg")));
    assert!(!is_nice2(&String::from("ieodomkazucvgmuy")));

    let f = File::open("./input/input5.txt").expect("Unable to open input file.");
    let f = BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    println!("Solution1: {}", lines.iter().filter(|x| is_nice(x)).count());
    println!(
        "Solution2: {}",
        lines.iter().filter(|x| is_nice2(x)).count()
    );
}

fn is_nice(s: &str) -> bool {
    let mut vowel_counter = 0;
    let mut has_double = false;
    let mut has_predefined_pair = false;
    let mut it = s.bytes();
    let n = it.next();
    let mut prev_b: u8;
    match n {
        Some(v) => {
            if is_vowel(v) {
                vowel_counter += 1;
            }
            prev_b = v
        }
        None => return false,
    }
    loop {
        let n = it.next();
        match n {
            Some(v) => {
                if is_vowel(v) {
                    vowel_counter += 1;
                }
                if prev_b == v {
                    has_double = true;
                }
                if is_predefined_pair(prev_b, v) {
                    has_predefined_pair = true;
                    break;
                }
                prev_b = v;
            }
            None => break,
        }
    }
    vowel_counter >= 3 && has_double && !has_predefined_pair
}

fn is_vowel(v: u8) -> bool {
    v == b'a' || v == b'e' || v == b'i' || v == b'o' || v == b'u'
}

fn is_predefined_pair(b1: u8, b2: u8) -> bool {
    b1 == b'a' && b2 == b'b'
        || b1 == b'c' && b2 == b'd'
        || b1 == b'p' && b2 == b'q'
        || b1 == b'x' && b2 == b'y'
}

fn is_nice2(s: &str) -> bool {
    let mut has_pair_repetition = false;
    let mut has_one_repetition = false;
    let bytes = s.as_bytes();
    let mut i = 1;
    while i < bytes.len() {
        if i > 1 && bytes[i - 2] == bytes[i] {
            has_one_repetition = true;
        }
        if search_pattern(s, i) {
            has_pair_repetition = true;
        }
        i += 1;
    }
    has_pair_repetition && has_one_repetition
}

fn search_pattern(s: &str, i: usize) -> bool {
    let pattern = &s[(i - 1)..=i];
    let s2 = &s[i + 1..];
    let r = s2.find(pattern);
    r.is_some()
}
