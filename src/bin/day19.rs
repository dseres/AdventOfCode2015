use multimap::MultiMap;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // let input = std::fs::read_to_string("./input/input19.txt").expect("File io error.");
    // let plant = Plant::from_string(&input);
    // println!("Solution1: {}", plant.solution1());
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

    // fn insert_elements(&mut self, e: Element, name: String) {
    //     if !elements.contains_key(e) {
    //         elements.insert(e, name);
    //     }
    // }

    // fn solution1(&self) -> usize {
    //     self.apply_rules().len()
    // }

    // fn apply_rules(&self) -> HashSet<String> {
    //     let mut molecules: HashSet<String> = HashSet::new();
    //     for i in 0..self.molecule.len() - 1 {
    //         if !self.check_and_apply_elem(&mut molecules, i, 1) {
    //             self.check_and_apply_elem(&mut molecules, i, 0);
    //         }
    //     }
    //     self.check_and_apply_elem(&mut molecules, self.molecule.len() - 1, 0);
    //     molecules
    // }

    // fn check_and_apply_elem(
    //     &self,
    //     mut molecules: &mut HashSet<String>,
    //     i: usize,
    //     l: usize,
    // ) -> bool {
    //     let elem = self.molecule.get(i..=i + l).unwrap();
    //     if self.rules.contains_key(elem) {
    //         self.apply_elem(&mut molecules, &elem, i, l);
    //         true
    //     } else {
    //         false
    //     }
    // }

    // fn apply_elem(&self, molecules: &mut HashSet<String>, elem: &str, i: usize, l: usize) {
    //     let rule_vec = self.rules.get_vec(elem).unwrap();
    //     for to_change in rule_vec {
    //         let mut new_molecule = self.molecule.clone();
    //         new_molecule.replace_range(i..=i + l, to_change);
    //         molecules.insert(new_molecule);
    //     }
    // }

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