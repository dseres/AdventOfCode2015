use std::io::BufRead;

fn main() {
    let f = std::fs::File::open("./input/input8.txt").expect("Unable to open input file.");
    let f = std::io::BufReader::new(f);
    let mut diff = 0;
    let mut diff2 = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let parsed_str = decode_str(&line);
        println!(
            "Strings: {}(length={}), {}(length={})",
            line,
            line.len(),
            parsed_str,
            parsed_str.chars().count()
        );
        diff += line.len() - parsed_str.chars().count();
        let encoded_str = encode_str(&line);
        println!("Encoded str: {}", encoded_str);
        diff2 += encoded_str.len() - line.len();
    }
    println!("Solution1: {}", diff);
    println!("Solution2: {}", diff2);
}

fn decode_str(input_str: &str) -> String {
    let mut parsed_str = String::new();
    let mut it = input_str.chars();
    //Check first char
    assert_eq!(it.next().unwrap(), '"');
    while let Some(c) = it.next() {
        match c {
            '\\' => match it.next() {
                Some('\"') => parsed_str.push('\"'),
                Some('\\') => parsed_str.push('\\'),
                Some('x') => {
                    let mut s = String::new();
                    s.push(it.next().unwrap());
                    s.push(it.next().unwrap());
                    let c = u8::from_str_radix(&s, 16).unwrap();
                    let c: char = c as char;
                    parsed_str.push(c);
                }
                _ => panic!(),
            },
            '\"' => match it.next() {
                Some(_) => panic!(),
                None => break,
            },
            _ => parsed_str.push(c),
        }
    }
    parsed_str
}

fn encode_str(input_str: &str) -> String {
    let mut output = String::new();
    output.push('\"');
    for c in input_str.chars() {
        match c {
            '\\' => output.push_str("\\\\"),
            '\"' => output.push_str("\\\""),
            _ => output.push(c),
        }
    }
    output.push('\"');
    output
}
