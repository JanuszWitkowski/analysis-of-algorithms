use crate::graph::Graph;
use crate::graph::State;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::thread::JoinHandle;

const GRAPH_SIZE:       usize = 1000;
const EDGE_FREQUENCY:   f64 = 0.1;

fn maximal_independent_set(graph: &Graph) -> HashSet<usize> {
    let n = graph.n;
    let independent = Arc::new(Mutex::new(HashSet::new()));
    let g = Arc::new(Mutex::new((*graph).clone()));

    let threads: Vec<JoinHandle<()>> = 
        (0..n)
        .map(|i| {
            let g = Arc::clone(&g);
            let independent = Arc::clone(&independent);
            thread::spawn(move || {
                loop {
                    let time = time::Duration::from_millis(fastrand::u64(5..100));
                    thread::sleep(time);
                    let mut independent = independent.lock().unwrap();  // LOCK
                    let mut g = g.lock().unwrap();                      // LOCK
                    // BEGIN CRITICAL SECTION
                    match g.nodes[i].state {
                        State::Illegal => {     // Remove this node from IndependentSet
                            independent.remove(&i);
                            g.update_node_state(i, &independent);
                            g.update_node_states_of_neighbors(i, &independent);
                        },
                        State::Addable => {  // Add this node to IndependentSet
                            independent.insert(i);
                            g.update_node_state(i, &independent);
                        },
                        _ => (),
                    }

                    // Check if current configuration satisfies the criteria
                    let mut stop = true;
                    for node in &g.nodes {
                        if node.state != State::Independent && node.state != State::Dependent {
                            stop = false;
                        }
                    }
                    if stop {
                        break;
                    }
                    // END CRITICAL SECTION
                }   // loop
            })
        }).collect();
        threads.into_iter().for_each(|p| p.join().unwrap());
        let lock = Arc::try_unwrap(independent).expect("Lock has more than one owner!");
        lock.into_inner().expect("Mutex not unlockable!")
}

pub fn mis_experiment() {
    println!("MIS EXPERIMENT");

    println!("Graph size: {}; Edge frequency: {}", GRAPH_SIZE, EDGE_FREQUENCY);

    let timer = std::time::Instant::now();
    let graph = Graph::new_random(GRAPH_SIZE, EDGE_FREQUENCY);
    let te = timer.elapsed();
    let mut edges = HashSet::new();
    for node in &graph.nodes {
        let uid = node.id;
        for vid in &node.neighborhood {
            if !edges.contains(&(*vid, uid)) {
                edges.insert((uid, *vid));
            }
        }
    }
    // println!("Edges: {:?}", edges);
    println!("Number of edges: {}", edges.len());
    println!("Graph generation time: {:?}", te);

    let timer = std::time::Instant::now();
    let independent = maximal_independent_set(&graph);
    let te = timer.elapsed();
    println!("Maximal Independent Set: {:?}", independent);
    println!("Size of Independent Set: {}", independent.len());
    println!("Time: {:?}", te);

    println!();
}
