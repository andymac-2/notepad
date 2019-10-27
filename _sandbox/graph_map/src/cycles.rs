use std::rc::{Rc, Weak};
use std::cell::{RefCell};

#[derive(Clone, Debug)]
enum Elem<T> {
    Head(Rc<ListElem<T>>),
    Node(Weak<ListElem<T>>, T, Rc<ListElem<T>>),
    Tail(Weak<ListElem<T>>),
}

#[derive(Clone, Debug)]
struct ListElem<T>(RefCell<Elem<T>>);
impl<T> ListElem<T> {
    fn new_tail() -> Rc<Self> {
        Rc::new(ListElem(RefCell::new(Elem::Tail(Weak::new()))))
    }
    fn new_head(tail: Rc<Self>) -> Rc<Self> {
        let head = Rc::new(ListElem(RefCell::new(Elem::Head(tail))));
        head.set_next(tail);
        head

    }
    fn between(before: Rc<Self>, data: T, after: Rc<Self>) -> Rc<Self> {
        let node_inner = Elem::Node(Rc::downgrade(&before), data, after.clone());
        let node = Rc::new(ListElem(RefCell::new(node_inner)));
        after.set_previous(node.clone());
        before.set_next(node.clone());
        node
    }
    
    fn is_tail (&self) -> bool {
        match *self.0.borrow_mut() {
            Elem::Tail(_) => true,
            _ => false
        }
    }
    fn is_head (&self) -> bool {
        match *self.0.borrow_mut() {
            Elem::Head(_) => true,
            _ => false,
        }
    }

    fn next (&self) -> Option<Rc<Self>> {
        match *self.0.borrow() {
            Elem::Head(ref next) => Some(next.clone()),
            Elem::Node(_, _, ref next) => Some(next.clone()),
            Elem::Tail(_) => None
        }
    }
    fn previous (&self) -> Option<Rc<Self>> {
        match *self.0.borrow() {
            Elem::Head(_) => None,
            Elem::Node(ref previous, _, _) => Some(previous.upgrade().unwrap()),
            Elem::Tail(ref previous) => Some(previous.upgrade().unwrap()),
        }
    }

    fn set_previous(&self, previous: Rc<Self>) {
        match *self.0.borrow_mut() {
            Elem::Head(_) => panic!("Tried to set previous of head"),
            Elem::Node(ref mut old_previous, _, _) => *old_previous = Rc::downgrade(&previous),
            Elem::Tail(ref mut old_previous) => *old_previous = Rc::downgrade(&previous),
        }
    }
    fn set_next(&self, next: Rc<Self>) {
        match *self.0.borrow_mut() {
            Elem::Head(ref mut old_next) => *old_next = next,
            Elem::Node(_, _, ref mut old_next) => *old_next = next,
            Elem::Tail(_) => panic!("Tried to set next of tail"),
        }
    }
    fn connect(first: Rc<Self>, other: Rc<Self>) {
        first.set_next(other.clone());
        other.set_previous(first);
    }
}

#[derive(Debug)]
struct List<T> {
    head: Rc<ListElem<T>>,
    current: Rc<ListElem<T>>,
    tail: Rc<ListElem<T>>
}

impl<T> List<T> {
    pub fn new () -> Self {
        let tail = ListElem::new_tail();
        let head = ListElem::new_head(tail.clone());
        List {
            head: head.clone(),
            current: head,
            tail: tail,
        }
    }
    pub fn advance (&mut self) {
        if let Some(next) = self.current.next() {
            self.current = next;
        }
        debug_assert!(self.invariant());
    }
    pub fn retreat (&mut self) {
        if let Some(previous) = self.current.previous() {
            self.current = previous;
        }
        debug_assert!(self.invariant());
    }
    pub fn insert_after (&mut self, data: T) {
        self.current.next().map(|next| {
            ListElem::between(self.current.clone(), data, next);
        });
        debug_assert!(self.invariant());
    }
    pub fn insert_before (&mut self, data: T) {
        self.current.previous().map(|previous| {
            ListElem::between(previous, data, self.current.clone());
        });
        debug_assert!(self.invariant());
    }

    pub fn split_after (&mut self) -> Self {
        self.current.next().map_or_else(|| List::new(), |next| {
            let old_tail = std::mem::replace(&mut self.tail, ListElem::new_tail());
            ListElem::connect(self.current.clone(), self.tail.clone());

            let new_head = ListElem::new_head(next.clone());

            debug_assert!(self.invariant());
            List {
                head: new_head.clone(),
                current: new_head,
                tail: old_tail,
            }
        })
    }
    pub fn split_before (&mut self) -> Self {
        self.current.previous().map_or_else(|| List::new(), |previous| {
            let new_tail = ListElem::new_tail();
            ListElem::connect(previous, new_tail.clone());

            let old_head = std::mem::replace(&mut self.head, ListElem::new_head(self.current.clone()));

            debug_assert!(self.invariant());
            List {
                head: old_head.clone(),
                current: old_head,
                tail: new_tail,
            }
        })
    }
    pub fn join (&mut self, other: Self) {
        let last = self.tail.previous.unwrap();
        let first = other.head.next().unwrap();
        ListElem::connect(last, first);
        self.tail = other.tail;
        debug_assert!(self.invariant());
    }

    fn invariant (&self) -> bool {
        let head_is_head = self.head.is_head();
        let tail_is_tail = self.tail.is_tail();
        let self_inner = &*self.current.0.borrow();
        let next_previous_is_self = self.current.next().map_or(true, |next|{
            let self_rc = next.previous().unwrap();
            let self_rc_inner = &*self_rc.0.borrow();
            same_object::<Elem<T>> (self_rc_inner, self_inner)
        });
        let previous_next_is_self = self.current.previous().map_or(true, |previous| {
            let self_rc = previous.next().unwrap();
            let self_rc_inner = &*self_rc.0.borrow();
            same_object::<Elem<T>> (self_rc_inner, self_inner)
        });
        head_is_head && tail_is_tail && next_previous_is_self && previous_next_is_self
    }
}

fn same_object<T>(a: *const T, b: *const T) -> bool {
    a == b
}

fn main() {
    let mut list = List::new();
    list.advance();
    list.insert_before(5);
    list.insert_before(6);

    println!("{:?}", list);

    println!("{:?}", vec![1, 2, 3, 4]);
}