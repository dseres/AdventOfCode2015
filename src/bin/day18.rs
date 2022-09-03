use std::fmt::Display;
use std::fmt::Formatter;

fn main() {
    unit_test();
    let input = std::fs::read_to_string("./input/input18.txt").expect("File io error.");
    let mut p = Panel::from(&input);
    for _i in 0..100 {
        p.tick();
    }
    println!("{}", p);
    println!("Solution1: {}", p.get_ons());
    let mut p = Panel::from(&input);
    for _i in 0..100 {
        p.tick2();
    }
    println!("{}", p);
    println!("Solution2: {}", p.get_ons());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Panel {
    lights: Vec<Vec<Light>>,
}

impl Light {
    fn from(c: u8) -> Light {
        match c {
            b'#' => Light::On,
            b'.' => Light::Off,
            _ => panic!("Bad input"),
        }
    }
}

impl Panel {
    fn from(input: &str) -> Panel {
        let mut p = Panel { lights: Vec::new() };
        for line in input.lines() {
            let mut lights: Vec<Light> = Vec::new();
            for c in line.as_bytes().iter() {
                lights.push(Light::from(*c));
            }
            p.lights.push(lights);
        }
        p
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Light::On => '#',
                Light::Off => '.',
            }
        )
    }
}

impl Display for Panel {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for line in &self.lights {
            for l in line {
                write!(f, "{}", l).unwrap();
            }
            writeln!(f).unwrap();
        }
        write!(f, "")
    }
}

impl Panel {
    fn tick(&mut self) {
        let mut next: Vec<Vec<Light>> = Vec::new();
        for (i, iter) in self.lights.iter().enumerate() {
            next.push(vec![Light::Off; iter.len()]);
            for (j, _iter2) in iter.iter().enumerate() {
                let n = self.count_neighbours(i, j);
                if self.lights[i][j] == Light::On && n == 2 || n == 3 {
                    next[i][j] = Light::On;
                }
            }
        }
        self.lights = next;
    }

    fn tick2(&mut self) {
        self.turn_on_corners();
        self.tick();
        self.turn_on_corners();
    }

    fn turn_on_corners(&mut self) {
        let line = self.lights.first_mut().unwrap();
        let corner = line.first_mut().unwrap();
        *corner = Light::On;
        let corner = line.last_mut().unwrap();
        *corner = Light::On;
        let line = self.lights.last_mut().unwrap();
        let corner = line.first_mut().unwrap();
        *corner = Light::On;
        let corner = line.last_mut().unwrap();
        *corner = Light::On;
    }

    fn count_neighbours(&self, i: usize, j: usize) -> u32 {
        self.get_next_coords(i, j)
            .iter()
            .filter(|(i, j)| self.lights[*i][*j] == Light::On)
            .count() as u32
    }

    fn get_next_coords(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut nexts = Vec::<(usize, usize)>::new();
        for di in -1..=1 {
            for dj in -1..=1 {
                if di != 0 || dj != 0 {
                    let i = i as i32 + di;
                    let j = j as i32 + dj;
                    if 0 <= i
                        && i < (self.lights.len() as i32)
                        && 0 <= j
                        && j < (self.lights[i as usize].len() as i32)
                    {
                        nexts.push((i as usize, j as usize));
                    }
                }
            }
        }
        nexts
    }

    fn get_ons(&self) -> usize {
        self.lights
            .iter()
            .map(|l| l.iter().filter(|x| **x == Light::On).count())
            .sum()
    }
}

fn unit_test() {
    let input = r".#.#.#
...##.
#....#
..#...
#.#..#
####..";
    let mut p = Panel::from(input);
    println!("{}", p);
    for _i in 0..4 {
        p.tick();
        //println!("{}", p);
    }
    assert_eq!(4, p.get_ons());
    let mut p = Panel::from(input);
    for _i in 0..5 {
        p.tick2();
    }
    println!("{}", p);
    assert_eq!(17, p.get_ons());
}
