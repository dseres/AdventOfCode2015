use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("./input/input19.txt").expect("File io error.");
    let mut plant = Plant::from_string(&input);
    let solution1 = plant.solution1();
    println!("Solution1: {}", solution1);
    let solution2 = plant.solution2();
    println!("Solution1: {}", solution2);
}

type Element = u8;
type Molecule = Vec<u8>;

#[derive(Debug)]
struct Plant {
    element_map: HashMap<String, Element>,
    reversed_element_map: HashMap<Element,String>,
    rules: MultiMap<Element, Molecule>,
    molecule: Molecule,
}

impl Plant {
    fn new() -> Self {
        Plant {
            element_map: HashMap::new(),
            reversed_element_map: HashMap::new(),
            rules: MultiMap::new(),
            molecule: Vec::new(),
        }
    }

    fn from_string(input: &str) -> Self {
        let mut plant = Plant::new();
        plant.collect_elements(input);
        plant.parse_rules(input);
        plant
    }

    fn collect_elements(&mut self, input: &str) {
        let element_names = Plant::collect_element_names(input);
        for en in element_names {
            let index = self.element_map.len() as u8;
            self.element_map.insert(en.clone(), index);
            self.reversed_element_map.insert(index, en);
        }
    }

    fn collect_element_names(input: &str) -> HashSet<String> {
        let mut elem_names = HashSet::<String>::new();
        let mut lines: Vec<&str> = input.lines().collect();
        //Split last line
        let last_line = lines.pop().unwrap();
        Plant::split_str(last_line).iter().for_each(|s| {
            elem_names.insert(String::from(s));
        });
        lines.pop();
        //Parse rules
        for line in lines {
            let rule_parts = line.split(" => ").collect::<Vec<&str>>();
            for s in rule_parts {
                Plant::split_str(s).iter().for_each(|s| {
                    elem_names.insert(String::from(s));
                });
            }
        }
        elem_names
    }

    fn parse_rules(&mut self, input: &str) {
        let mut lines: Vec<&str> = input.lines().collect();
        //Split last line
        let last_line = lines.pop().unwrap();
        self.molecule = Self::split_str(last_line)
            .iter()
            .map(|s| self.element_map.get(s).unwrap().clone())
            .collect();
        lines.pop();
        //Parse rules
        for line in lines {
            let rule_parts = line.split(" => ").collect::<Vec<&str>>();
            let e: Element = self.element_map.get(rule_parts[0]).unwrap().clone();
            let m: Molecule = Self::split_str(rule_parts[1])
                .iter()
                .map(|s| self.element_map.get(s).unwrap().clone())
                .collect();
            self.rules.insert(e, m);
        }
    }

    fn split_str(input: &str) -> Vec<String> {
        let mut e_names: Vec<String> = Vec::new();
        if input.len() == 0 {
        } else if input.len() == 1 {
            e_names.push(String::from(input));
        } else {
            let mut prev_c = 'a';
            for c in input.chars() {
                if prev_c.is_uppercase() && c.is_uppercase() {
                    e_names.push(String::from(prev_c))
                } else if prev_c.is_uppercase() {
                    let mut n = String::from(prev_c);
                    n.push(c);
                    e_names.push(n);
                }
                prev_c = c;
            }
            if prev_c.is_uppercase() {
                e_names.push(String::from(prev_c));
            }
        }
        e_names
    }

    fn solution1(&mut self) -> usize {
        self.apply_rules(self.molecule.clone()).len()
    }

    fn apply_rules(&mut self, molecule: Molecule) -> HashSet<Molecule> {
        let mut molecules: HashSet<Molecule> = HashSet::new();
        for i in 0..molecule.len() {
            if self.rules.contains_key(&molecule[i]) {
                for replace in self.rules.get_vec(&molecule[i]).unwrap() {
                    let new_molecule = Self::apply_rule_on_molecule(i, replace, &molecule);
                    molecules.insert(new_molecule);
                }
            }
        }
        molecules
    }

    fn apply_rule_on_molecule(i : usize, replace : &Molecule, target: &Molecule) -> Molecule {
        let mut new_molecule = Molecule::new();
        for j in 0..i{
            new_molecule.push(target[j])
        }
        for e in replace {
            new_molecule.push(e.clone());
        }
        for j in (i+1)..(target.len()){
            new_molecule.push(target[j]);
        }
        new_molecule
    }

    fn solution2(&mut self) -> i32 {
        let mut m = Molecule::new();
        m.push(self.element_map.get("e").unwrap().clone());
        return self.next_molecule(0,m).unwrap();
    }

    fn next_molecule(&mut self, step : i32, m: Molecule) -> Option<i32> {
        if m.len() > self.molecule.len(){
            None
        }
        else if m == self.molecule {
            return Some(step)
        }
        else {
            let next_molecules = self.apply_rules(m);
            let ret = next_molecules.iter()
                .map( |nm| self.next_molecule(step + 1, nm.clone()))
                .filter( |o| o.is_some())
                .min_by( |o1,o2| o1.unwrap().cmp(&o2.unwrap()) );
            match ret {
                None => None,
                Some(option) => option
            }    
        }
    }
}


#[test]
fn test_split_str() {
    assert_eq!(Plant::split_str(""), Vec::<String>::new());
    assert_eq!(Plant::split_str("e"), vec!["e"]);
    assert_eq!(Plant::split_str("H"), vec!["H"]);
    assert_eq!(Plant::split_str("HC"), vec!["H", "C"]);
    assert_eq!(Plant::split_str("AlFH"), vec!["Al", "F", "H"]);
    assert_eq!(Plant::split_str("FAlH"), vec!["F", "Al", "H"]);
    assert_eq!(Plant::split_str("FHAl"), vec!["F", "H", "Al"]);
    assert_eq!(
        Plant::split_str("CRnFYFYFAr"),
        vec!["C", "Rn", "F", "Y", "F", "Y", "F", "Ar"]
    );
    assert_eq!(
        Plant::split_str("CRnFYFYFAr"),
        vec!["C", "Rn", "F", "Y", "F", "Y", "F", "Ar"]
    );
    assert_eq!(Plant::split_str("RnArAl"), vec!["Rn", "Ar", "Al"]);
}

#[test]
fn test_collect_element_names() {
    let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHArAl"#;
    let mut set = HashSet::<String>::new();
    set.insert(String::from("e"));
    set.insert(String::from("H"));
    set.insert(String::from("O"));
    set.insert(String::from("Al"));
    set.insert(String::from("Ar"));
    assert_eq!(Plant::collect_element_names(&input), set);
}

#[test]
fn test_from_string() {
    let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHArAl"#;
    let plant = Plant::from_string(input);
    dbg!(&plant);
    assert_eq!(plant.element_map.len(),5);
    assert_eq!(plant.reversed_element_map.len(),5);
    assert_eq!(plant.molecule.len(),5);
    assert_eq!(plant.rules.iter().map( |(_e,v)| v.len()).sum::<usize>(),5);
}

#[test]
fn test_solution1() {
    let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOH"#;
    let mut plant = Plant::from_string(input);
    dbg!(&plant);
    assert_eq!(plant.solution1(), 4);
}


#[test]
fn test_solution2() {
    let input = r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#;
    let mut plant = Plant::from_string(input);
    dbg!(&plant);
    assert_eq!(plant.solution2(), 6);
}
