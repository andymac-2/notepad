use std::cmp::Ordering;

#[derive(Debug)]
enum Boolean {
    True,
    False,
}

enum Unit {
    Unit
}

enum Void {}

fn my_void_func () -> Void {
    unreachable!()
}

fn my_func2 (a: u32, b: u32) -> u32 { a + b }

fn my_func<T: Clone + Ord> (a: &T, b: &T) -> T {
    match a.cmp(b) {
        Ordering::Less => b.clone(),
        _ => a.clone(),
    }
}

fn my_func3<T> (arg1: Result<T, T>) -> T {
    match arg1 {
        Ok(elem) => elem,
        Err(elem) => elem,
    }
}

fn my_unit_func () -> Unit {
    Unit::Unit
}


fn main () {
    println!("Hello World!");
    println!("{:?}", my_func(&3, &4));
    println!("true: {:?}, false: {:?}", Boolean::True, Boolean::False);
}

impl Void {
    fn my_func () -> Self {
        unreachable!()
    }
    fn my_func2 (&mut self) {
        unimplemented!()
    }
    fn my_func3 (self) {}
}