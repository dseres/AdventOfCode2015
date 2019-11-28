use aoc2015::input;
use aoc2015::input::ReadFromLine;
use std::str::SplitWhitespace;

fn main() {
    let aunt_sues = input::read_input_file::<AuntSue>("./input/input16.txt");
    let def_sue = AuntSue {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    for &a in &aunt_sues {
        if def_sue.match_other(&a) {
            println!("Matched: {:?}", a);
            println!("Solution1: {}", a.id);
        }
        if def_sue.match_other2(&a) {
            println!("Matched: {:?}", a);
            println!("Solution1: {}", a.id);
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct AuntSue {
    id: i32,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl ReadFromLine<AuntSue> for AuntSue {
    fn from_line(line: &str) -> AuntSue {
        let mut aunt_sue: AuntSue = Default::default();
        let mut it = line.split_whitespace();
        aunt_sue.id = AuntSue::parse_id(&mut it);
        for _i in 0..3 {
            let (prop, value) = AuntSue::parse_property(&mut it);
            aunt_sue.add_property(&prop, value);
        }
        aunt_sue
    }
}

impl AuntSue {
    fn parse_id(it: &mut SplitWhitespace) -> i32 {
        assert_eq!("Sue", it.next().unwrap());
        let mut w = String::from(it.next().unwrap());
        w.pop();
        w.parse::<i32>().unwrap()
    }

    fn parse_property(it: &mut SplitWhitespace) -> (String, i32) {
        let mut w1 = String::from(it.next().unwrap());
        w1.pop();
        let mut w2 = String::from(it.next().unwrap());
        if w2.ends_with(',') {
            w2.pop();
        }
        (w1, w2.parse::<i32>().unwrap())
    }

    fn add_property(&mut self, prop: &str, v: i32) {
        match prop {
            "children" => self.children = Some(v),
            "cats" => self.cats = Some(v),
            "samoyeds" => self.samoyeds = Some(v),
            "pomeranians" => self.pomeranians = Some(v),
            "akitas" => self.akitas = Some(v),
            "vizslas" => self.vizslas = Some(v),
            "goldfish" => self.goldfish = Some(v),
            "trees" => self.trees = Some(v),
            "cars" => self.cars = Some(v),
            "perfumes" => self.perfumes = Some(v),
            _ => panic!(String::from("Bad property: ") + prop),
        }
    }

    fn match_other(&self, other: &AuntSue) -> bool {
        (self.children == other.children || other.children.is_none())
            && (self.cats == other.cats || other.cats.is_none())
            && (self.samoyeds == other.samoyeds || other.samoyeds.is_none())
            && (self.pomeranians == other.pomeranians || other.pomeranians.is_none())
            && (self.akitas == other.akitas || other.akitas.is_none())
            && (self.vizslas == other.vizslas || other.vizslas.is_none())
            && (self.goldfish == other.goldfish || other.goldfish.is_none())
            && (self.trees == other.trees || other.trees.is_none())
            && (self.cars == other.cars || other.cars.is_none())
            && (self.perfumes == other.perfumes || other.perfumes.is_none())
    }


    fn match_other2(&self, other: &AuntSue) -> bool {
        (self.children == other.children || other.children.is_none())
            && (self.cats < other.cats || other.cats.is_none())
            && (self.samoyeds == other.samoyeds || other.samoyeds.is_none())
            && (self.pomeranians > other.pomeranians || other.pomeranians.is_none())
            && (self.akitas == other.akitas || other.akitas.is_none())
            && (self.vizslas == other.vizslas || other.vizslas.is_none())
            && (self.goldfish > other.goldfish || other.goldfish.is_none())
            && (self.trees < other.trees || other.trees.is_none())
            && (self.cars == other.cars || other.cars.is_none())
            && (self.perfumes == other.perfumes || other.perfumes.is_none())
    }
}
