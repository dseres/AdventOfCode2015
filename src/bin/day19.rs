use std::collections::HashSet;
use multimap::MultiMap;

fn main(){
    let input = std::fs::read_to_string("./input/input19.txt").expect("File io error.");
    let plant = Plant::from_string(&input);
    dbg!(&plant);
    let molecules = plant.apply_rules();
    dbg!(&molecules);
    println!("Solution1: {}", molecules.len());
}

#[derive(Debug)]
struct Plant{
    rules: MultiMap<String, String>,
    molecule: String,
}

impl Plant{
    fn from_string(input: &str)->Plant{
        let rules = MultiMap::new();
        let mut lines  = input.lines();
        let mut plant = Plant{rules, molecule: String::new()};
        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                plant.molecule = String::from(lines.next().unwrap());
                break
            }
            plant.add_rule_from_line(&line);
        }
        plant
    }

    fn add_rule_from_line(&mut self, line: &str) {
        let mut it = line.split_whitespace();
        let elem_from = String::from(it.next().unwrap());
        it.next().unwrap();
        let elem_to = String::from(it.next().unwrap());
        self.rules.insert(elem_from, elem_to);
    }

    fn apply_rules(&self)->HashSet<String>{
        let mut molecules :HashSet<String>= HashSet::new();
        for i in 0..self.molecule.len()-1{
            if !self.check_and_apply_elem(&mut molecules, i, 1) {
                self.check_and_apply_elem(&mut molecules, i, 0);
            }
        }
        self.check_and_apply_elem(&mut molecules, self.molecule.len()-1, 0);
        molecules
    }

    fn check_and_apply_elem(&self, mut molecules :&mut HashSet<String>, i: usize, l:usize)->bool{
        let elem = self.molecule.get(i..=i+l).unwrap();
        if  self.rules.contains_key(elem){
            self.apply_elem(&mut molecules, &elem,i, l);
            true
        } else {
            false
        }
    }

    fn apply_elem(&self, molecules :&mut HashSet<String>, elem: &str, i: usize, l:usize){
        let rule_vec = self.rules.get_vec(elem).unwrap();
        for to_change in rule_vec {
            let mut new_molecule = self.molecule.clone();
            new_molecule.replace_range(i..=i+l, to_change);
            molecules.insert(new_molecule);
        }
    }
}
