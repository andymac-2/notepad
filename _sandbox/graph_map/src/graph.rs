use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
pub struct InnerTask {
    name: &'static str,
    duration: u32,
    dependencies: Vec<Task>,
}
#[derive(Clone, Eq, PartialEq, Debug, PartialOrd, Ord)]
struct Task(Rc<RefCell<InnerTask>>);
impl Task {
    fn new (name: &'static str, duration: u32, dependencies: Vec<Task>) -> Self {
        Task(Rc::new(RefCell::new(InnerTask {
            name: name,
            duration: duration,
            dependencies: dependencies,
        })))
    }
    fn set_duration (&self, duration: u32) {
        self.0.borrow_mut().duration = duration;
    } 
    fn start_time (&self) -> u32 {
        (&self.0.borrow().dependencies)
            .into_iter()
            .map(|dependency| dependency.end_time())
            .max()
            .unwrap_or(0)
    }
    fn end_time(&self) -> u32 {
        self.start_time() + self.0.borrow().duration
    }
}

fn main () {
    let lay_foundation = 
        Task::new("Lay Foundation", 10, vec![]);
    let build_walls = 
        Task::new("Build Walls", 5, vec![lay_foundation.clone()]);
    let build_roof = 
        Task::new("Build Roof", 11, vec![build_walls.clone()]);
    let paint_walls = 
        Task::new("Paint Walls", 2, vec![build_walls.clone()]);
    let furnish_house = 
        Task::new("Furnish House", 3, vec![build_roof.clone(), paint_walls.clone()]);

    println!("Days require to finish house: {}", furnish_house.end_time());

    build_walls.set_duration(10);
    println!("Days require to finish house: {}", furnish_house.end_time());
}

