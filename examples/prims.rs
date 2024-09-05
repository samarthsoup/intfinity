use intfinity::SingleInfiniteNumber;
use std::collections::HashSet;

struct Graph {
    vertices: usize,
    edges: Vec<Vec<(usize, usize)>>,  // adjacency list: (neighbor, weight)
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Self {
            vertices,
            edges: vec![vec![]; vertices],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: usize) {
        self.edges[u].push((v, weight));
        self.edges[v].push((u, weight));
    }

    fn prim_mst(&self) {
        let mut in_mst = HashSet::new();
        let mut key: Vec<SingleInfiniteNumber<usize>> = vec![SingleInfiniteNumber::Infinity; self.vertices];  
        let mut parent: Vec<Option<usize>> = vec![None; self.vertices];  
        key[0] = SingleInfiniteNumber::new(0);  

        for _ in 0..self.vertices {
            let u = self.min_key(&key, &in_mst);

            in_mst.insert(u);

            for &(v, weight) in &self.edges[u] {
                let weight = SingleInfiniteNumber::new(weight);
                if !in_mst.contains(&v) && weight < key[v] {
                    key[v] = weight;
                    parent[v] = Some(u);
                }
            }
        }


        self.print_mst(&parent);
    }

    fn min_key(&self, key: &[SingleInfiniteNumber<usize>], in_mst: &HashSet<usize>) -> usize {
        let mut min = SingleInfiniteNumber::Infinity;
        let mut min_index = 0;

        for v in 0..self.vertices {
            if !in_mst.contains(&v) && key[v] < min {
                min = key[v];
                min_index = v;
            }
        }
        min_index
    }

    fn print_mst(&self, parent: &[Option<usize>]) {
        println!("Edge \tWeight");
        for i in 1..self.vertices {
            if let Some(p) = parent[i] {
                for &(v, weight) in &self.edges[p] {
                    if v == i {
                        println!("{} - {} \t{}", p, i, weight);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut g = Graph::new(5);

    g.add_edge(0, 1, 2);
    g.add_edge(0, 3, 6);
    g.add_edge(1, 2, 3);
    g.add_edge(1, 3, 8);
    g.add_edge(1, 4, 5);
    g.add_edge(2, 4, 7);
    g.add_edge(3, 4, 9);

    g.prim_mst();
}
