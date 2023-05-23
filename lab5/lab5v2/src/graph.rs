use std::collections::HashSet;
use rand::Rng;

#[derive(Clone, PartialEq)]
pub enum State {
    Independent,
    Dependent,
    Illegal,
    Addable
}

#[derive(Clone)]
pub struct Node {
    pub id:             usize,
    pub state:          State,
    pub neighborhood:   HashSet<usize>,
}

impl Node {
    pub fn new(id: usize) -> Node {
        Node{ id, state: State::Addable, neighborhood: HashSet::new() }
    }

    pub fn add_neighbor(&mut self, neighbor: usize) {
        self.neighborhood.insert(neighbor);
    }

    pub fn update_state(&mut self, independent: &HashSet<usize>) {
        let is_ind = independent.contains(&self.id);
        let neighbor_ind = self.any_independent_neighbor(independent);
        if is_ind {
            if neighbor_ind {
                self.state = State::Illegal;
            } else {
                self.state = State::Independent;
            }
        } else if neighbor_ind {
            self.state = State::Dependent;
        } else {
            self.state = State::Addable;
        }
    }

    fn any_independent_neighbor(&self, independent: &HashSet<usize>) -> bool {
        for neighbor in &self.neighborhood {
            if independent.contains(neighbor) {
                return true;
            }
        }
        false
    }
}


#[derive(Clone)]
pub struct Graph {
    pub n:      usize,
    pub nodes:  Vec<Node>,
}

impl Graph {
    pub fn new(n: usize) -> Graph {
        let mut graph = Graph {
            n, nodes: Vec::new()
        };
        for i in 0..n {
            graph.nodes.push(Node::new(i));
        }
        graph
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.nodes[u].add_neighbor(v);
        self.nodes[v].add_neighbor(u);
    }

    pub fn new_random(n: usize, p: f64) -> Graph {
        let mut graph = Graph::new(n);
        let mut rng = rand::thread_rng();
        let mut q: f64;
        for u in 0..n {
            for v in (u+1)..n {
                q = rng.gen();
                if q < p {
                    graph.add_edge(u, v);
                }
            }
        }
        let independent = HashSet::new();
        for i in 0..n {
            graph.nodes[i].update_state(&independent);
        }
        graph
    }

    pub fn update_node_state(&mut self, u: usize, independent: &HashSet<usize>) {
        self.nodes[u].update_state(independent);
    }

    pub fn update_node_states_of_neighbors(&mut self, u: usize, independent: &HashSet<usize>) {
        let neighs = self.nodes[u].neighborhood.clone();
        for v in neighs {
            self.nodes[v].update_state(independent);
        }
    }
}
