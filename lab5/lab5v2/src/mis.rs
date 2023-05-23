use crate::graph::Graph;
use crate::graph::Color;
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

    let processes: Vec<JoinHandle<()>> = 
        (0..n)
        .map(|i| {
            let g = Arc::clone(&g);
            let independent = Arc::clone(&independent);
            thread::spawn(move || {
                loop {
                    let time = time::Duration::from_millis(fastrand::u64(10..100));
                    thread::sleep(time);
                    let mut independent = independent.lock().unwrap();
                    let mut g = g.lock().unwrap();
                    match g.nodes[i].color {
                        Color::Red => {
                            independent.remove(&i);
                            g.update_node_color(i, &independent);
                            g.update_node_colors_of_neighbors(i, &independent);
                        },
                        Color::Yellow => {
                            independent.insert(i);
                            g.update_node_color(i, &independent);
                        },
                        _ => (),
                    }
                    let mut stop = true;
                    for node in &g.nodes {
                        if node.color != Color::Black && node.color != Color::White {
                            stop = false;
                        }
                    }
                    if stop {
                        break;
                    }
                }
            })
        }).collect();
        processes.into_iter().for_each(|p| p.join().unwrap());
        let lock = Arc::try_unwrap(independent).expect("Lock has more than one owner!");
        lock.into_inner().expect("Mutex not unlockable!")
}

pub fn mis_experiment() {
    // println!("Hello! This is a placeholder for Maximum Independent Set experiment!");
    println!("Graph size: {}; Edge frequency: {}", GRAPH_SIZE, EDGE_FREQUENCY);

    let timer = std::time::Instant::now();
    let graph = Graph::new_random(GRAPH_SIZE, EDGE_FREQUENCY);
    println!("Graph generation time: {:?}", timer.elapsed());

    let timer = std::time::Instant::now();
    let independent = maximal_independent_set(&graph);
    println!("Maximal Independent Set: {:?}", independent);
    println!("Size of Independent Set: {}", independent.len());
    println!("Time: {:?}", timer.elapsed());
}
