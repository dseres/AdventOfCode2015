
fn main() {
    let input = std::fs::read_to_string("./input/input20.txt").expect("File io error.");
    print!("Solution1; {}", solution1(i32::from_str_radix(&input,10).unwrap()));
}

fn solution1(input : i32 ) -> i32 {
    let input =input / 10;
    let mut i = 1;    
    loop {
        let presents = compute_present_count(i);
        println!(r#"Presents of house {}: {}"#, i, presents);
        if presents == input {
            break i;
        }
        i+= 1;
    }
}

fn compute_present_count(h : i32) -> i32 {
    let mut presents = 0;
    for i in 1..=h {
        if h % i == 0 {
            presents += i;
        }
    }
    presents
}


#[test]
fn test_solution1(){

    assert_eq!(compute_present_count(9),13);

    assert_eq!(solution1(10),1);
    assert_eq!(solution1(30),2);
    assert_eq!(solution1(40),3);
    assert_eq!(solution1(70),4);
    assert_eq!(solution1(60),5);
    assert_eq!(solution1(120),6);
    assert_eq!(solution1(80),7);
    assert_eq!(solution1(150),8);
    assert_eq!(solution1(130),9);
}