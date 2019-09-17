use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use uuid::Uuid;

#[derive(Clone, Eq, PartialEq, Hash, Debug, PartialOrd, Ord)]
pub struct Task {
    name: &'static str,
    duration: u32,
}
impl Task {
    fn new (name: &'static str, duration: u32) -> Self {
        Task {
            name: name,
            duration: duration,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GraphNode<T> {
    data: T,
    incoming: HashSet<Uuid>,
    outgoing: HashSet<Uuid>,
}
impl<T> GraphNode<T> {
    fn new (data: T) -> Self {
        GraphNode {
            data: data,
            incoming: HashSet::new(),
            outgoing: HashSet::new(),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Graph<T: Eq + Hash> (
    HashMap<Uuid, GraphNode<T>>
);
impl<T: Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph(HashMap::new())
    }
    pub fn add_edge(&mut self, start: &Uuid, end: &Uuid) {
        self.0.get_mut(start).map(|node| {
            node.outgoing.insert(*end);
        });
        self.0.get_mut(end).map(|node| {
            node.incoming.insert(*start);
        });
    }
    pub fn remove_edge(&mut self, start: &Uuid, end: &Uuid) {
        self.0.get_mut(start).map(|node| {
            node.outgoing.remove(end);
        });
        self.0.get_mut(end).map(|node| {
            node.incoming.remove(start);
        });
    }
    pub fn remove_node(&mut self, node_id: &Uuid) -> T {
        let node = self.0.remove(node_id).expect("remove_node: invalid key");
        for start in node.incoming.iter() {
            self.0.get_mut(start).map(|start_node| {
                start_node.outgoing.remove(node_id);
            });
        }
        for end in node.outgoing.iter() {
            self.0.get_mut(end).map(|end_node| {
                end_node.incoming.remove(node_id);
            });
        }
        node.data
    }
    pub fn add_node(&mut self, node: T) -> Uuid {
        let key = Uuid::new_v4();
        self.0.insert(key, GraphNode::new(node));
        key
    }
    pub fn get(&self, key: &Uuid) -> &T {
        &self.0.get(key).expect("get: invalid key.").data
    }
    pub fn get_outgoing(&self, key: &Uuid) -> &HashSet<Uuid> {
        &self.0.get(key).expect("get_outgoing: invalid key.").outgoing
    }
    pub fn get_incoming(&self, key: &Uuid) -> &HashSet<Uuid> {
        &self.0.get(key).expect("get_incoming: invalid key.").incoming
    }
}

struct GraphView<'a> {
    graph: &'a Graph<Task>,
    start_times: HashMap<Uuid, u32>,
    end_times: HashMap<Uuid, u32>,
}
impl<'a> GraphView<'a> {
    fn new (graph: &'a Graph<Task>) -> Self {
        GraphView {
            graph: graph,
            start_times: HashMap::new(),
            end_times: HashMap::new(),
        }
    }
    fn end_time(&mut self, key: &Uuid) -> u32 {
        if let Some(result) = self.end_times.get(key) {
            return result.clone();
        }
        
        let result = self.graph.get(key).duration + self.start_time(key);

        self.end_times.insert(key.clone(), result);
        result
    }
    fn start_time(&mut self, key: &Uuid) -> u32 {
        if let Some(result) = self.start_times.get(key) {
            return result.clone();
        }

        let result = self.graph.get_incoming(key)
            .into_iter()
            .map(|key_out| self.end_time(key_out))
            .max()
            .unwrap_or(0);

        self.start_times.insert(key.clone(), result);
        result
    }
}

struct GraphView2<'a> {
    graph: &'a Graph<Task>,
    start_times: HashMap<Uuid, Option<u32>>,
    end_times: HashMap<Uuid, Option<u32>>,
}
impl<'a> GraphView2<'a> {
    fn new (graph: &'a Graph<Task>) -> Self {
        GraphView2 {
            graph: graph,
            start_times: HashMap::new(),
            end_times: HashMap::new(),
        }
    }
    fn end_time(&mut self, key: &Uuid) -> Option<u32> {
        if let Some(result) = self.end_times.get(key) {
            return result.clone();
        }
        self.end_times.insert(key.clone(), None);
        
        let result = self.start_time(key)
            .map(|time| time + self.graph.get(key).duration);

        self.end_times.insert(key.clone(), result);
        result
    }
    fn start_time(&mut self, key: &Uuid) -> Option<u32> {
        if let Some(result) = self.start_times.get(key) {
            return result.clone();
        }
        self.start_times.insert(key.clone(), None);

        let result = self.graph.get_incoming(key)
            .into_iter()
            .map(|key_out| self.end_time(key_out))
            .fold(Some(0), |max_time, end_time| Some(max_time?.max(end_time?)));

        self.start_times.insert(key.clone(), result);
        result
    }
}

fn main() {
    let mut graph = Graph::new();

    let lay_foundation = graph.add_node(Task::new("Lay foundation", 1));
    let build_walls = graph.add_node(Task::new("Build walls", 2));
    graph.add_edge(&lay_foundation, &build_walls);

    let build_roof = graph.add_node(Task::new("Build roof", 4));
    graph.add_edge(&build_walls, &build_roof);

    let paint_walls = graph.add_node(Task::new("Paint walls", 8));
    graph.add_edge(&build_walls, &paint_walls);

    let furnish_house = graph.add_node(Task::new("Furnish house", 16));
    graph.add_edge(&paint_walls, &furnish_house);

    graph.add_edge(&furnish_house, &build_walls);

    let mut view = GraphView2::new(&graph);
    println!("Days require to finish house: {:?}", view.end_time(&furnish_house));
}

mod test {
    use super::*;

    #[test]
    fn graph_view () {
        let mut graph = Graph::new();
        let n1 = graph.add_node(Task::new("Lay foundation", 1));
        let n2 = graph.add_node(Task::new("Build walls", 2));
        graph.add_edge(&n1, &n2);
        let n3 = graph.add_node(Task::new("Build roof", 4));
        graph.add_edge(&n2, &n3);
        let n4 = graph.add_node(Task::new("Paint walls", 8));
        graph.add_edge(&n2, &n4);
        let n5 = graph.add_node(Task::new("Furnish house", 16));
        graph.add_edge(&n4, &n5);

        let mut view = GraphView::new(&graph);
        assert_eq!(view.start_time(&n5), 11);
        assert_eq!(view.end_time(&n5), 27);
    }
}
