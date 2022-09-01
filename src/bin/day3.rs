use core::cmp::Ordering;
//use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let buffer = read_input();
    solution1(&buffer);
    solution2(&buffer);
}

fn read_input() -> Vec<u8> {
    let mut f = File::open("./input/input3.txt").expect("Unable to open input file.");
    let mut buffer = Vec::<u8>::new();
    f.read_to_end(&mut buffer)
        .expect("Unable to read input file.");
    buffer
}

fn solution1(buffer: &[u8]) {
    let mut pos = Pos::new();
    let mut houses: HashMap<Pos, i32> = HashMap::new();
    houses.insert(pos, 1);
    for x in buffer {
        pos = pos.next(*x);
        let v = houses.entry(pos).or_insert(1);
        *v += 1;
    }
    println!("Solution1: {}", houses.len());
}

fn solution2(buffer: &[u8]) {
    let mut santa_pos = Pos::new();
    let mut robo_pos = Pos::new();
    let mut houses: HashMap<Pos, i32> = HashMap::new();
    houses.insert(santa_pos, 2);
    let mut it = buffer.iter();
    loop {
        match it.next() {
            Some(val) => {
                santa_pos = santa_pos.next(*val);
                let counter = houses.entry(santa_pos).or_insert(1);
                *counter += 1;
            }
            None => break,
        }
        match it.next() {
            Some(val) => {
                robo_pos = robo_pos.next(*val);
                let counter = houses.entry(robo_pos).or_insert(1);
                *counter += 1;
            }
            None => break,
        }
    }
    println!("Solution2: {}", houses.len());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Pos) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Pos) -> Ordering {
        let c = self.x.cmp(&other.x);
        match c {
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            _ => c,
        }
    }
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn next(self, c: u8) -> Pos {
        match c {
            b'^' => Pos {
                x: self.x,
                y: self.y - 1,
            },
            b'v' => Pos {
                x: self.x,
                y: self.y + 1,
            },
            b'<' => Pos {
                x: self.x - 1,
                y: self.y,
            },
            b'>' => Pos {
                x: self.x + 1,
                y: self.y,
            },
            _ => panic!(),
        }
    }
}
