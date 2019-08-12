use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Task {
    name: String,
    duration: u32,
}

#[derive(Debug, Clone)]
pub struct Graph<Ix: Eq + Hash> {
    incoming: HashMap<Ix, HashSet<Ix>>,
    outgoing: HashMap<Ix, HashSet<Ix>>,
}
impl<Ix: Eq + Hash + Copy> Graph<Ix> {
    fn new() -> Self {
        Graph {
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }
    fn add(&mut self, start: Ix, end: Ix) {
        match self.incoming.get_mut(&end) {
            None => {
                let mut set = HashSet::new();
                set.insert(start);
                self.incoming.insert(end, set);
            },
            Some(starts) => { 
                starts.insert(start); 
            },
        }

        match self.outgoing.get_mut(&start) {
            None => {
                let mut set = HashSet::new();
                set.insert(end);
                self.outgoing.insert(start, set);
            },
            Some(ends) => { 
                ends.insert(end); 
            },
        }
    }
    fn remove(&mut self, start: Ix, end: Ix) {
        self.incoming.get_mut(&end).map_or((), |starts| {
            starts.remove(&start);
        });
        self.outgoing.get_mut(&start).map_or((), |ends| {
            ends.remove(&end);
        });
    }
    fn get_adjacent(&self, node: Ix) -> (Vec<Ix>, Vec<Ix>) {
        let incoming = self.incoming.get(&node).map_or(Vec::new(), |set| {
            set.iter().cloned().collect()
        });
        let outgoing = self.outgoing.get(&node).map_or(Vec::new(), |set| {
            set.iter().cloned().collect()
        });
        (incoming, outgoing)
    }
    fn remove_node(&mut self, node: Ix) {
        let (incoming, outgoing) = self.get_adjacent(node);
        for start in incoming {
            self.remove(start, node);
        }
        for end in outgoing {
            self.remove(node, end);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
