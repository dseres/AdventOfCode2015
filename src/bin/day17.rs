use permutator::Combination;

fn main() {
    let input = std::fs::read_to_string("./input/input17.txt").expect("File io error.");
    let input: Vec<i32> = input.lines().map(|l| l.parse().unwrap()).collect();
    //dbg!(&input);
    println!("Solution1: {}", get_solution1(&input));
    println!("Solution2: {}", get_solution2(&input));
}

fn get_solution1(input: &[i32]) -> i32 {
    let mut counter = 0;
    for (i, _iter) in input.iter().enumerate() {
        for c in input.combination(i + 1) {
            let sum = c.iter().map(|x| **x).sum();
            if 150 == sum {
                counter += 1;
                //println!("combination{}: {:?}", counter, c);
            }
        }
    }
    counter
}

fn get_solution2(input: &[i32]) -> i32 {
    let mut counter = 0;
    for (i, _iter) in input.iter().enumerate() {
        for c in input.combination(i + 1) {
            let sum = c.iter().map(|x| **x).sum();
            if 150 == sum {
                counter += 1;
                //println!("combination{}: {:?}", counter, c);
            }
        }
        if counter > 0 {
            //println!("{:?}", i+1);
            break;
        }
    }
    counter
}
