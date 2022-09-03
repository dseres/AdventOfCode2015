use std::io::BufRead;
//use std::io::Write;

fn main() {
    let f = std::fs::File::open("./input/input6.txt").expect("Unable to open input file.");
    let f = std::io::BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let l = LightDecoration::from(&lines);
    println!("Solution1: {}", l.count_lights());
    println!("Solution1: {}", l.count_brightness());

    // let mut f = std::fs::File::create("./output6.txt").expect("Unable to open output file.");
    // let s = format!("{}\n", l);
    // f.write_all(s.as_bytes()).unwrap();
}

#[derive(Debug)]
enum LightOperation {
    Toggle,
    TurnOn,
    TurnOff,
}

const LIGHT_SIZE: usize = 1000;

struct LightDecoration {
    lights: Vec<Vec<bool>>, //[[bool; 1000]; 1000],
    blights: Vec<Vec<u32>>, //[[u32; 1000]; 1000]
}

impl LightDecoration {
    fn new() -> LightDecoration {
        LightDecoration {
            lights: vec![vec![false; LIGHT_SIZE]; LIGHT_SIZE], //[[false; 1000]; 1000],
            blights: vec![vec![0; LIGHT_SIZE]; LIGHT_SIZE],
        }
    }

    fn from(lines: &[String]) -> LightDecoration {
        let mut l = LightDecoration::new();
        for line in lines {
            l.parse_line(line);
        }
        l
    }

    fn parse_line(&mut self, line: &str) {
        let mut it = line.split_whitespace();
        let op = self.parse_op(&mut it);
        let (x0, y0) = self.parse_point(it.next().unwrap());
        it.next();
        let (x1, y1) = self.parse_point(it.next().unwrap());
        self.swith_lights(&op, x0, y0, x1, y1);
    }

    fn parse_op(&self, it: &mut std::str::SplitWhitespace) -> LightOperation {
        let s = it.next().unwrap();
        match s {
            "toggle" => LightOperation::Toggle,
            "turn" => match it.next().unwrap() {
                "on" => LightOperation::TurnOn,
                "off" => LightOperation::TurnOff,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    fn parse_point(&self, points: &str) -> (usize, usize) {
        let mut it2 = points.split(',');
        let x0: usize = it2.next().unwrap().parse::<usize>().unwrap();
        let y0: usize = it2.next().unwrap().parse::<usize>().unwrap();
        (x0, y0)
    }

    fn swith_lights(&mut self, op: &LightOperation, x0: usize, y0: usize, x1: usize, y1: usize) {
        //println!("{:?} {:?} {:?} {:?} {:?}", op, x0, y0, x1, y1);
        for i in x0..=x1 {
            for j in y0..=y1 {
                self.switch_light(op, i, j);
            }
        }
    }

    fn switch_light(&mut self, op: &LightOperation, i: usize, j: usize) {
        let l = &mut self.lights[i][j];
        let bl = &mut self.blights[i][j];
        match op {
            LightOperation::Toggle => {
                *l = !(*l);
                *bl += 2;
            }
            LightOperation::TurnOn => {
                *l = true;
                *bl += 1;
            }
            LightOperation::TurnOff => {
                *l = false;
                if *bl > 0 {
                    *bl -= 1;
                }
            }
        }
    }

    fn count_lights(&self) -> usize {
        let sum_v: Vec<usize> = self
            .lights
            .iter()
            .map(|v| v.iter().filter(|b| **b).count())
            .collect();
        sum_v.iter().sum()
    }

    fn count_brightness(&self) -> usize {
        let sum_v: Vec<usize> = self
            .blights
            .iter()
            .map(|v| {
                let sum: u32 = v.iter().sum();
                sum as usize
            })
            .collect();
        sum_v.iter().sum()
    }
}

impl std::fmt::Display for LightDecoration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..LIGHT_SIZE {
            for j in 0..LIGHT_SIZE {
                let c = if self.lights[i][j] { 'X' } else { '.' };
                write!(f, "{}", c).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}
