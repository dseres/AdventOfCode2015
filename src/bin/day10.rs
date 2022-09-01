fn main() {
    unit_test();
    let result = iterate_look_and_say("1113122113", 40);
    println!("Solution1: {}", result.len());
    let result = iterate_look_and_say(&result, 10);
    println!("Solution2: {}", result.len());
}

fn unit_test() {
    let solution = iterate_look_and_say(&String::from("1"), 5);
    assert_eq!(&solution, "312211");
}

fn iterate_look_and_say(input_str: &str, count: i32) -> String {
    //println!("Input str: {}.", input_str);
    let mut output = String::from(input_str);
    for _i in 0..count {
        output = look_and_say(&output);
        //println!("After {}th iteration: {}.", i+1,  output);
    }
    output
}

fn look_and_say(input_str: &str) -> String {
    let mut output_str = String::new();
    let mut iter = input_str.as_bytes().iter();
    let mut prev = iter.next().unwrap();
    let mut counter = 1;
    for b in iter {
        if b == prev {
            counter += 1;
        } else {
            output_str.push_str(&counter.to_string());
            output_str.push(*prev as char);
            prev = b;
            counter = 1;
        }
    }
    output_str.push_str(&counter.to_string());
    output_str.push(*prev as char);
    output_str
}
