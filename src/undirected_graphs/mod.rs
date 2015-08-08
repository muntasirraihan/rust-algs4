use std::iter;
use super::stacks_and_queues::bag::{Bag, Iter};

#[derive(Clone, Debug)]
pub struct Graph {
    v: usize,
    e: usize,
    adj: Vec<Bag<usize>>
}

impl Graph {
    pub fn new(v: usize) -> Graph {
        Graph {
            v: v,
            e: 0,
            adj: iter::repeat(Bag::<usize>::new()).take(v).collect()
        }
    }

    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v, "vertex is not between 0 and {}", self.v - 1)
    }

    pub fn vertices(&self) -> usize {
        self.v
    }

    pub fn edges(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);

        self.e += 1;
        self.adj[v].add(w);
        self.adj[w].add(v);
    }

    // FIXME: should this be a global function
    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn max_degree(&self) -> usize {
        (0 .. self.vertices()).map(|v| self.degree(v)).max().unwrap_or(0)
    }

    pub fn average_degree(&self) -> f64 {
        // (0 .. self.vertices()) .map(|v| self.degree(v)).sum::<usize>() as f64 / self.vertices() as f64
        2.0 * self.edges() as f64 / self.vertices() as f64
    }

    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;
        for v in 0 .. self.vertices() {
            for w in self.adj(v) {
                if v == *w {
                    count += 1;
                }
            }
        }
        count / 2
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("graph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for (v, adj) in self.adj.iter().enumerate() {
            for w in adj.iter() {
                dot.push_str(&format!("  {} -- {};\n", v, w));
            }
        }
        dot.push_str("}\n");
        dot
    }

    pub fn adj(&self, v: usize) -> Iter<usize> {
        self.adj[v].iter()
    }

    pub fn dfs<'a>(&'a self, s: usize) -> DepthFirstPaths<'a> {
        DepthFirstPaths {
            graph: self,
            marked: Vec::new(),
            edge_to: Vec::new(),
            s: s
        }
    }
}

pub struct DepthFirstPaths<'a> {
    graph: &'a Graph,
    marked: Vec<bool>,
    edge_to: Vec<usize>,
    s: usize
}

#[test]
fn test_graph_visit() {
    let mut g = Graph::new(13);
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(0, 6);
    g.add_edge(0, 5);
    g.add_edge(5, 3);
    g.add_edge(5, 4);
    g.add_edge(3, 4);
    g.add_edge(4, 6);


    g.add_edge(7, 8);

    g.add_edge(9, 10);
    g.add_edge(9, 11);
    g.add_edge(9, 12);
    g.add_edge(11, 12);


}


#[test]
fn test_graph() {
    let mut g = Graph::new(10);
    g.add_edge(0, 3);
    g.add_edge(0, 5);
    g.add_edge(4, 5);
    g.add_edge(2, 9);
    g.add_edge(2, 8);
    g.add_edge(3, 7);

    g.add_edge(1, 6);
    g.add_edge(6, 9);
    g.add_edge(5, 8);

    // println!("got => \n{}", g.to_dot());

    assert_eq!(10, g.vertices());
    assert_eq!(9, g.edges());
    assert_eq!(3, g.degree(5));

    for w in g.adj(5) {
        assert!(vec![8, 4, 0].contains(w));
    }

    assert_eq!(g.max_degree(), 3);
    assert!(g.average_degree() < 2.0);
    assert_eq!(g.number_of_self_loops(), 0);
}
