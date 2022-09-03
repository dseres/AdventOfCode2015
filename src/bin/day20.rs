fn main() {
    let input = std::fs::read_to_string("./input/input20.txt").expect("File io error.");
    println!("Solution1; {}", solution1(input.parse::<i32>().unwrap()));
    println!("Solution2; {}", solution2(input.parse::<i32>().unwrap()));
}

fn solution1(input: i32) -> i32 {
    let input = (input / 10) as usize;
    let mut numbers: Vec<usize> = vec![0; input];
    for i in 1..=input {
        let mut ind = i - 1;
        while ind < input {
            numbers[ind] += i;
            ind += i;
        }
        //dbg!(&numbers);
    }
    for i in 0..numbers.len() {
        if numbers[i] >= input {
            return (i + 1) as i32;
        }
    }
    0
}

fn solution2(input: i32) -> i32 {
    let mut numbers: Vec<usize> = vec![0; input as usize];
    for i in 1..=input {
        let mut ind = i - 1;
        for _j in 0..50 {
            if ind < input {
                numbers[ind as usize] += i as usize * 11;
                ind += i;
            }
            else {
                break;
            }
        }
        //dbg!(&numbers);
    }
    for i in 0..numbers.len() {
        if numbers[i] >= input as usize {
            return (i + 1) as i32;
        }
    }
    0
}

#[test]
fn test_solution1() {
    assert_eq!(solution1(10), 1);
    assert_eq!(solution1(30), 2);
    assert_eq!(solution1(40), 3);
    assert_eq!(solution1(70), 4);
    assert_eq!(solution1(60), 4);
    assert_eq!(solution1(120), 6);
    assert_eq!(solution1(80), 6);
    assert_eq!(solution1(150), 8);
    assert_eq!(solution1(130), 8);
    assert_eq!(solution1(280), 12);
}
