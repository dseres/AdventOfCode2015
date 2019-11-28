use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("./input/input2.txt").expect("Unable to open input file.");
    let f = BufReader::new(f);
    let mut sum_paper: u32 = 0;
    let mut sum_ribbon: u32 = 0;
    for line in f.lines() {
        let mybox = Box::from(line.unwrap());
        sum_paper += mybox.paper_area();
        sum_ribbon += mybox.ribbon_length();
    }
    println!("Sum paper should order: {}", sum_paper);
    println!("Sum Ribbon should order: {}", sum_ribbon);
}

#[derive(Debug)]
struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl Box {
    fn area(&self) -> u32 {
        self.l * self.w * 2 + self.l * self.h * 2 + self.w * self.h * 2
    }

    fn paper_area(&self) -> u32 {
        let area = self.area();
        let sides = vec![self.l * self.w, self.l * self.h, self.w * self.h];
        let min = *sides.iter().min_by(|x, y| x.cmp(y)).unwrap();
        area + min
    }

    fn ribbon_length(&self) -> u32 {
        let length = self.l * self.w * self.h;
        let bow_length = if self.l >= self.w && self.l >= self.h {
            self.w * 2 + self.h * 2
        } else if self.w >= self.l && self.w >= self.h {
            self.l * 2 + self.h * 2
        } else {
            self.l * 2 + self.w * 2
        };
        length + bow_length
    }

    fn from(line: String) -> Box {
        let mut it = line.split('x');
        let v0 = it.next().unwrap();
        let v1 = it.next().unwrap();
        let v2 = it.next().unwrap();
        Box{
            l: v0.parse::<u32>().unwrap(),
            w: v1.parse::<u32>().unwrap(),
            h: v2.parse::<u32>().unwrap(),
        }
    }
}

impl fmt::Display for Box {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} x {} x {})", self.l, self.w, self.h)
    }
}
