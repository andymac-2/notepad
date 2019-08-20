struct Parent {
    name: String,
    child: Child,
}
struct Child {
    name: String
}
impl Parent {
    fn display_as_parent_method(&self) {
        println!("parent's name: {}", self.name);
        println!("child's name: {}", self.child.name);
    }
    fn display_by_passing_arguments(&self) {
        self.child.display_with_parent_name(&self.name);
    }
}
impl Child {
    fn display_with_parent_name(&self, parent_name: &String) {
        println!("parent's name: {}", parent_name);
        println!("child's name: {}", self.name);
    }
}

fn main() {
    let child = Child {name: "Child".to_string()};
    let parent = Parent {name: "Parent".to_string(), child: child};
    
    parent.display_as_parent_method();
    parent.display_by_passing_arguments();
}