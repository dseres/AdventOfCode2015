#![allow(dead_code)]

pub mod graph;

pub mod input {
    pub trait ReadFromLine<T> {
        fn from_line(line: &str) -> T;
    }

    pub fn read_input_file<T>(fname: &str) -> Vec<T>
    where
        T: ReadFromLine<T>,
    {
        let contents =
            std::fs::read_to_string(fname).expect(&("Cannot read input file: ".to_owned() + fname));
        let mut structs: Vec<T> = Vec::new();
        for line in contents.lines() {
            structs.push(T::from_line(&line));
        }
        structs
    }
}
