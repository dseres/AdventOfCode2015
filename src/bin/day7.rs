use regex::Regex;
use std::collections::BTreeMap;
use std::io::BufRead;

fn main() {
    //unit_test();
    let f = std::fs::File::open("./input/input7.txt").expect("Unable to open input file.");
    let f = std::io::BufReader::new(f);
    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();
    let mut circuit = Circuit::from(&lines);
    circuit.compute_all_values();
    //circuit.print_insructions();
    //circuit.print_values();
    println!("Solution1: {}", circuit.get("a"));
    circuit.overwrite_b();
    println!("Solution2: {}", circuit.get("a"));
}

#[allow(dead_code)]
fn unit_test() {
    let input = String::from(
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
    );
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut circuit = Circuit::from(&lines);
    circuit.print_insructions();
    circuit.compute_all_values();
    circuit.print_values();
}

#[derive(Debug, Clone)]
enum Param {
    None,
    Wire(String),
    FixNumber(u16),
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Let,
    And,
    Or,
    Not,
    RShift,
    LShift,
}

//#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Clone)]
struct Instruction {
    param1: Param,
    param2: Param,
    op: Operation,
    dest: String,
}

struct Circuit {
    insts: BTreeMap<String, Instruction>,
    values: BTreeMap<String, u16>,
}

impl Circuit {
    fn new() -> Circuit {
        Circuit {
            insts: BTreeMap::new(),
            values: BTreeMap::new(),
        }
    }

    fn from(lines: &[String]) -> Circuit {
        let mut circuit = Circuit::new();
        for line in lines {
            let i = Instruction::from(line);
            if circuit.insts.contains_key(&i.dest) {
                let mut msg = String::from("The following wire has more than one rule: ");
                msg.push_str(&i.dest);
                msg.push('.');
                panic!("{}", msg);
            }
            circuit.insts.insert(i.dest.clone(), i);
        }
        circuit
    }
}

impl Instruction {
    fn new() -> Instruction {
        Instruction {
            param1: Param::None,
            param2: Param::None,
            op: Operation::Let,
            dest: String::new(),
        }
    }

    fn from(line: &str) -> Instruction {
        lazy_static::lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x) ^
            (   (   (?P<param1>[a-z]+|\d+) \s+ )?
                (?P<op>NOT|OR|AND|[LR]SHIFT) \s+  )?
            (?P<param2>[a-z]+|\d+)
            \s* -> \s*
            (?P<dest>[a-z]+)
            $").unwrap();
        }
        let caps = RE.captures(line).unwrap();
        let mut i = Instruction::new();
        if let Some(m) = caps.name("param1") {
            i.param1 = Param::from(m.as_str());
        }
        if let Some(m) = caps.name("op") {
            i.op = Operation::from(m.as_str());
        }
        i.param2 = caps
            .name("param2")
            .map(|m| Param::from(m.as_str()))
            .unwrap();
        i.dest = caps.name("dest").map(|m| String::from(m.as_str())).unwrap();
        i
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.param1, self.op, self.param2, self.dest
        )
    }
}

impl Param {
    fn from(s: &str) -> Param {
        if Param::is_number(s) {
            Param::FixNumber(s.parse::<u16>().unwrap())
        } else if Param::is_wire(s) {
            Param::Wire(String::from(s))
        } else {
            let mut msg = String::from("Bad param: ");
            msg.push_str(s);
            msg.push('.');
            panic!("{}", msg);
        }
    }

    fn is_number(s: &str) -> bool {
        s.chars().next().unwrap().is_digit(10)
    }

    fn is_wire(s: &str) -> bool {
        s.chars().next().unwrap().is_lowercase()
    }
}

impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Param::Wire(n) => write!(f, "{}", n),
            Param::FixNumber(n) => write!(f, "{}", n),
            Param::None => write!(f, ""),
        }
    }
}

impl Operation {
    fn from(s: &str) -> Operation {
        match s {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "NOT" => Operation::Not,
            "RSHIFT" => Operation::RShift,
            "LSHIFT" => Operation::LShift,
            _ => {
                let mut msg = String::from("Bad operation: ");
                msg.push_str(s);
                msg.push('.');
                panic!("{}", msg);
            }
        }
    }

    fn do_op(self, p1: u16, p2: u16) -> u16 {
        match self {
            Operation::And => p1 & p2,
            Operation::Or => p1 | p2,
            Operation::Let => p2,
            Operation::Not => !p2,
            Operation::LShift => p1 << p2,
            Operation::RShift => p1 >> p2,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "AND"),
            Operation::Or => write!(f, "OR"),
            Operation::Let => write!(f, "LET"),
            Operation::Not => write!(f, "NOT"),
            Operation::LShift => write!(f, "LSHIFT"),
            Operation::RShift => write!(f, "RSHIFT"),
        }
    }
}

impl Circuit {
    fn get(&self, s: &str) -> u16 {
        self.values[s]
    }

    fn compute_all_values(&mut self) {
        let keys: Vec<String> = self.insts.keys().cloned().collect();
        for s in keys {
            self.compute_value(&s);
        }
    }

    fn compute_value(&mut self, s: &str) -> u16 {
        if !self.values.contains_key(s) {
            //print!("Computing value of {}. ", s);
            let i: Instruction = self.insts[s].clone();
            let v = self.do_instruction(i);
            self.values.insert(String::from(s), v);
            v
        } else {
            self.values[s]
        }
    }

    fn do_instruction(&mut self, inst: Instruction) -> u16 {
        //println!("Processing instruction {}.", inst);
        let p1 = self.get_param_value(&inst.param1);
        let p2 = self.get_param_value(&inst.param2);
        inst.op.do_op(p1, p2)
    }

    fn get_param_value(&mut self, param: &Param) -> u16 {
        match param {
            Param::None => 0,
            Param::FixNumber(n) => *n,
            Param::Wire(w) => self.compute_value(w),
        }
    }

    fn print_insructions(&self) {
        println!("Instructions:");
        for i in self.insts.values() {
            println!("{}", i);
        }
    }

    fn print_values(&self) {
        println!("Values of wires:");
        for (s, v) in self.values.iter() {
            println!("{}: {}", s, v);
        }
    }

    fn overwrite_b(&mut self) {
        let b = self.insts["b"].clone();
        let b = Instruction {
            param2: Param::FixNumber(self.get("a")),
            ..b
        };
        //Without IndexMut I have to write this monster line instead of self.insts["b"]=b;
        *(self.insts.get_mut("b").unwrap()) = b;
        self.values.clear();
        self.compute_all_values();
    }
}
