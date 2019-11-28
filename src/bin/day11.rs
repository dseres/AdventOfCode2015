fn main() {
    unit_test();
    let s1 = generate_pass(b"hxbxwxba");
    let mut s2 = s1.clone();
    next(&mut s2);
    println!("Solution1: {}", String::from_utf8(s1.clone()).unwrap());
    println!("Solution1: {}", String::from_utf8(generate_pass(&s2)).unwrap());
}

fn unit_test() {
    assert!(has_two_doubles(b"abcdffaa"));
    assert_eq!("abcdffaa", String::from_utf8(generate_pass(b"abcdefgh")).unwrap());
    assert_eq!("ghjaabcc", String::from_utf8(generate_pass(b"ghijklmn")).unwrap());
}

fn generate_pass(input: &[u8]) -> Vec<u8> {
    let mut seq = vec![0; input.len()];
    seq.clone_from_slice(input);
    while !is_valid(&seq) {
        next(&mut seq);
    }
    seq
}

fn next(seq: &mut [u8]) {
    for i in 0..seq.len() {
        let i = seq.len() - 1 - i;
        if seq[i] == b'z' {
            seq[i] = b'a';
        } else {
            seq[i] += 1;
            break;
        }
    }
}

fn is_valid(input: &[u8]) -> bool {
    // println!(
    //     "Sequence: {}, has_increasing_three_letter: {}, contains_iol: {}, has_two_doubles: {}",
    //     String::from_utf8(Vec::from(input)).unwrap(),
    //     has_increasing_three_letter(input),
    //     contains_iol(input),
    //     has_two_doubles(input)
    // );
    has_increasing_three_letter(input) && !contains_iol(input) && has_two_doubles(input)
}

fn has_increasing_three_letter(input: &[u8]) -> bool {
    for i in 0..input.len() - 2 {
        if input[i] + 1 == input[i + 1] && input[i] + 2 == input[i + 2] {
            return true;
        }
    }
    false
}

fn contains_iol(input: &[u8]) -> bool {
    input.iter().any(|x| *x == b'i' || *x == b'o' || *x == b'l')
}

fn has_two_doubles(input: &[u8]) -> bool {
    for i in 0..(input.len() - 3) {
        for j in (i + 2)..(input.len() - 1) {
            if input[i] == input[i + 1] && input[j] == input[j + 1] && input[i] != input[j] {
                return true;
            }
        }
    }
    false
}
