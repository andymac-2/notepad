
struct A<'c> {
    value: u32,
    child: &'c std::cell::RefCell<C>
}
impl<'c> A<'c> {
    fn total (&self) -> u32 {
        self.value + self.child.borrow().value
    }
}

struct B<'c> {
    value: u32,
    child: &'c std::cell::RefCell<C>
}
impl<'c> B<'c> {
    fn difference (&self) -> u32 {
        self.value - self.child.borrow().value
    }
}

struct C {
    pub value: u32,
}

struct P {
    a: u32,
    b: u32,
    c: u32,
    a_plus_c: u32,
    b_minus_c: u32,
}
impl P {
    fn new (a: u32, b: u32, c: u32) -> Self {
        P {
            a: a, b: b, c: c,
            a_plus_c: a + c,
            b_minus_c: b - c,
        }
    }
    fn set_c (&mut self, c: u32) {
        self.c = c;
        self.a_plus_c = self.a + self.c;
        self.b_minus_c = self.b - self.c;
    }
    fn total(&self) -> u32 {
        self.a_plus_c
    }
    fn difference(&self) -> u32 {
        self.b_minus_c
    }
}

struct P2 {
    pub a: u32,
    pub b: u32,
    pub c: u32,
}
struct View<'p> {
    _lock: &'p P2,
    total: u32,
    difference: u32,
}
impl<'p> View<'p> {
    fn new(parent: &'p P2) -> Self {
        View {
            _lock: parent,
            total: parent.a + parent.c,
            difference: parent.b - parent.c,
        }
    }
    fn total(&self) -> u32 {
        self.total
    }
    fn difference(&self) -> u32 {
        self.difference
    }
}

fn main () {
    let c = std::cell::RefCell::from(C {value: 10});
    let a = A {value: 15, child: &c};
    let b = B {value: 30, child: &c};

    println!("A's total: {}", a.total());               // 25
    println!("B's difference: {}", b.difference());     // 20

    c.borrow_mut().value = 5;

    println!("A's total: {}", a.total());               // 20
    println!("B's difference: {}", b.difference());     // 25

    let mut p = P::new(15, 30, 10);

    println!("A's total: {}", p.total());               // 25
    println!("B's difference: {}", p.difference());     // 20

    p.set_c(5);

    println!("A's total: {}", p.total());               // 20
    println!("B's difference: {}", p.difference());     // 25
    
    let mut p2 = P2 {a: 15, b: 30, c: 10};
    {
        let view = View::new(&p2);
        println!("A's total: {}", view.total());               // 25
        println!("B's difference: {}", view.difference());     // 20
    }

    p2.c = 5;

    {
        let view = View::new(&p2);
        println!("A's total: {}", view.total());               // 25
        println!("B's difference: {}", view.difference());     // 20
    }
}