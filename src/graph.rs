#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: usize,
    pub name: String,
}

impl std::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({} {})", self.id, self.name)
    }
}

#[derive(Default, Clone)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Vec<i32>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: vec![vec![0; 8]; 8],
        }
    }

    pub fn insert_edge_or_increase_weight(
        &mut self,
        from: &str,
        to: &str,
        dist: i32,
    ) -> (usize, usize) {
        let (id1, id2) = self.insert_edge(from, to);
        self.edges[id1][id2] += dist;
        self.edges[id2][id1] += dist;
        (id1, id2)
    }

    pub fn insert_edge(&mut self, from: &str, to: &str) -> (usize, usize) {
        let id1 = self.insert_vertex(&from);
        let id2 = self.insert_vertex(&to);
        self.edges.resize(self.vertices.len(), Vec::new());
        for v in self.edges.iter_mut() {
            v.resize(self.vertices.len(), 0);
        }
        (id1, id2)
    }

    pub fn insert_vertex(&mut self, vertex: &str) -> usize {
        let id = self.vertices.iter().position(|x| *(x.name) == *vertex);
        if id.is_none() {
            let id = self.vertices.len();
            let v = Vertex {
                id: id,
                name: String::from(vertex),
            };
            self.vertices.push(v);
            id
        } else {
            id.unwrap()
        }
    }

    //Find minimal and maximal pathes of travelling salesman problem.
    pub fn find_shortest_and_longest_path_length(&self) -> (i32, i32) {
        self.find_tsp_loop_lengths(|g, v| g.get_path_length(v))
    }

    pub fn find_shortest_and_longest_loop_length(&self) -> (i32, i32) {
        self.find_tsp_loop_lengths(|g, v| g.get_loop_length(v))
    }

    pub fn find_tsp_loop_lengths<F>(&self, len_func: F) -> (i32, i32)
    where
        F: Fn(&Graph, &[Vertex]) -> i32,
    {
        let mut vertices = self.vertices.clone();
        let mut heap = permutohedron::Heap::new(&mut vertices);
        let mut min_length = std::i32::MAX;
        let mut min_path: Vec<Vertex> = Vec::new();
        let mut max_length = std::i32::MIN;
        let mut max_path: Vec<Vertex> = Vec::new();
        while let Some(path) = heap.next_permutation() {
            let length = len_func(self, &path);
            if length < min_length {
                min_length = length;
                min_path = path.clone();
            }
            if length > max_length {
                max_length = length;
                max_path = path.clone();
            }
        }
        println!("Minimal path: {:?}, length: {}", min_path, min_length);
        println!("Maximal path: {:?}, length: {}", max_path, max_length);

        print!("Weights: ");
        for i in 0..max_path.len() - 1 {
            print!(
                "{}, ",
                self.edges[max_path[i].id as usize][max_path[i + 1].id as usize]
            );
        }
        println!(
            "{}, ",
            self.edges[max_path[max_path.len() - 1].id as usize][max_path[0].id as usize]
        );

        (min_length, max_length)
    }

    pub fn get_path_length(&self, path: &[Vertex]) -> i32 {
        let mut length = 0;
        for i in 0..path.len() - 1 {
            length += self.edges[path[i].id][path[i + 1].id];
        }
        length
    }

    pub fn get_loop_length(&self, path: &[Vertex]) -> i32 {
        let mut length = self.get_path_length(path);
        length += self.edges[path[path.len() - 1].id][path[0].id];
        length
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Vertices :")?;
        for v in self.vertices.iter() {
            write!(f, "{}, ", v)?;
        }
        writeln!(f, "")?;
        writeln!(f, "Edges:")?;
        for ev in self.edges.iter() {
            for e in ev.iter() {
                write!(f, "{}, ", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
