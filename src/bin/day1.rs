use std::fs;

fn main() {
    let fname = "./input/input1.txt";
    let input = fs::read(fname).expect("Unable to read input file.");
    //println!("{}",input);
    println!("Solution1 : {}", get_floor(&input));
    println!("Solution2 : {}", get_index_of_minus_one(&input));
}

fn get_floor(input: &[u8]) -> i32 {
    let mut floor = 0;
    for x in input {
        match x {
            b'(' => floor += 1,
            b')' => floor += -1,
            _ => panic!(),
        }
    }
    floor
}

fn get_index_of_minus_one(input: &[u8]) -> i32 {
    let mut floor = 0;
    let mut ind = 0;
    for x in input {
        ind += 1;
        match x {
            b'(' => floor += 1,
            b')' => floor += -1,
            _ => panic!(),
        }
        if floor == -1 {
            break;
        }
    }
    ind
}
