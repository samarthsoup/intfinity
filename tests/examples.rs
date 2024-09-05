use intfinity::SingleInfiniteNumber;
use std::collections::HashSet;

struct Graph {
    vertices: usize,
    edges: Vec<Vec<(usize, u32)>>,  // adjacency list: (neighbor, weight)
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Self {
            vertices,
            edges: vec![vec![]; vertices],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, weight: u32) {
        self.edges[u].push((v, weight));
        self.edges[v].push((u, weight));
    }

    fn prim_mst(&self) -> Vec<Option<usize>> {
        let mut in_mst = HashSet::new();  
        let mut key: Vec<SingleInfiniteNumber<u32>> = vec![SingleInfiniteNumber::Infinity; self.vertices];
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

        parent  
    }

    fn min_key(&self, key: &[SingleInfiniteNumber<u32>], in_mst: &HashSet<usize>) -> usize {
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

    fn prim_mst_no_intfinity(&self) -> Vec<Option<usize>> {
        let mut in_mst = HashSet::new();  
        let mut key: Vec<u32> = vec![u32::MAX; self.vertices];  
        let mut parent: Vec<Option<usize>> = vec![None; self.vertices]; 
        key[0] = 0;  

        for _ in 0..self.vertices {
            let u = self.min_key_no_intfinity(&key, &in_mst);
            in_mst.insert(u);

            for &(v, weight) in &self.edges[u] {
                if !in_mst.contains(&v) && weight < key[v] {
                    key[v] = weight;
                    parent[v] = Some(u);
                }
            }
        }

        parent  
    }

    fn min_key_no_intfinity(&self, key: &[u32], in_mst: &HashSet<usize>) -> usize {
        let mut min = u32::MAX;
        let mut min_index = 0;

        for v in 0..self.vertices {
            if !in_mst.contains(&v) && key[v] < min {
                min = key[v];
                min_index = v;
            }
        }
        min_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prims_algorithm_basic() {
        let mut g = Graph::new(5);

        g.add_edge(0, 1, 2);
        g.add_edge(0, 3, 6);
        g.add_edge(1, 2, 3);
        g.add_edge(1, 3, 8);
        g.add_edge(1, 4, 5);
        g.add_edge(2, 4, 7);
        g.add_edge(3, 4, 9);

        let mst = g.prim_mst();

        assert_eq!(mst[1], Some(0)); // edge: 0 - 1
        assert_eq!(mst[2], Some(1)); // edge: 1 - 2
        assert_eq!(mst[4], Some(1)); // edge: 1 - 4
        assert_eq!(mst[3], Some(0)); // edge: 0 - 3
    }

    #[test]
    fn test_prims_algorithm_with_all_edges_max_weight_intfinity() {
        let mut g = Graph::new(4);

        // all edges going into 3 are u32::MAX
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, u32::MAX); 
        g.add_edge(2, 3, u32::MAX); 

        let mst = g.prim_mst();

        assert_eq!(mst[1], Some(0)); // 0 - 1 (weight 2)
        assert_eq!(mst[2], Some(0)); // 0 - 2 (weight 1)
        assert_eq!(mst[3], Some(2)); // 1 - 3 (weight u32::MAX)
    }

    #[test]
    fn test_prims_algorithm_with_all_edges_max_weight_no_intfinity() {
        let mut g = Graph::new(4);

        // all edges going into 3 are u32::MAX
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);
        g.add_edge(0, 2, 1);
        g.add_edge(1, 3, u32::MAX); 
        g.add_edge(2, 3, u32::MAX); 

        let mst = g.prim_mst_no_intfinity();

        assert_eq!(mst[1], Some(0)); // 0 - 1 (weight 2)
        assert_eq!(mst[2], Some(0)); // 0 - 2 (weight 1)
        assert_eq!(mst[3], None);    // node 3 is incorrectly not connected to the MST
    }
}
