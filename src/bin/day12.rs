use json::object::Object;
use json::JsonValue;
use regex::Regex;

fn main() {
    let contents = std::fs::read_to_string("./input/input12.txt").unwrap();
    solution1(&contents);
    solution2(&contents);
}

//Very fast solution for question 1.
fn solution1(contents: &str) {
    lazy_static::lazy_static! {
        static ref REG: Regex = Regex::new(r"[\[:,] *(-?\d+)").unwrap();
    }
    let captures = REG.captures_iter(contents);
    let sum: i32 = captures
        .map(|caps| {
            //println!("Capture: {:?}", &caps[1]);
            caps[1].parse::<i32>().unwrap()
        })
        .sum();
    println!("Solution1: {}", sum);
}

//Solutions with json library
fn solution2(contents: &str) {
    let json = json::parse(contents).unwrap();
    let sum = count(&json);
    println!("Solution1v2: {}", sum);
    let sum = count2(&json);
    println!("Solution2: {}", sum);
}

fn count(val: &JsonValue) -> i64 {
    match val {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Array(jarray) => count_array(&jarray),
        JsonValue::Object(jobj) => count_obj(&jobj),
        _ => 0,
    }
}

fn count_array(jarray: &[JsonValue]) -> i64 {
    jarray.iter().map(|v| count(v)).sum()
}

fn count_obj(jobj: &Object) -> i64 {
    jobj.iter().map(|(_key, value)| count(value)).sum()
}

fn count2(val: &JsonValue) -> i64 {
    match val {
        JsonValue::Number(n) => n.as_fixed_point_i64(0).unwrap(),
        JsonValue::Array(jarray) => count_array2(&jarray),
        JsonValue::Object(jobj) => count_obj2(&jobj),
        _ => 0,
    }
}

fn count_array2(jarray: &[JsonValue]) -> i64 {
    jarray.iter().map(|v| count2(v)).sum()
}

fn count_obj2(jobj: &Object) -> i64 {
    if jobj.iter().any(|(_k, value)| is_red(value)) {
        0
    } else {
        jobj.iter().map(|(_k, value)| count2(value)).sum()
    }
}

fn is_red(val: &JsonValue) -> bool {
    match val {
        JsonValue::Short(s) => s == "red",
        JsonValue::String(s) => s == "red",
        _other => false,
    }
}
