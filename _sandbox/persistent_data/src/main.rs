use std::rc::Rc;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct CloneTracker(u32);
impl Clone for CloneTracker {
    fn clone(&self) -> Self {
        println!("Cloning {}...", self.0);
        CloneTracker(self.0)
    }
}


#[derive(Debug, Clone)]
pub enum ListBox<A> {
    Nil,
    Cons(A, Box<ListBox<A>>),
}
impl<A> ListBox<A> {
    pub fn new() -> Self {
        ListBox::Nil
    }
    pub fn cons(&mut self, elem: A) {
        let tail = std::mem::replace(self, ListBox::Nil);
        let mut list = ListBox::Cons(elem, Box::new(tail));
        std::mem::swap(self, &mut list)
    }
    pub fn uncons(&mut self) -> Option<A> {
        let list = std::mem::replace(self, ListBox::Nil);
        match list {
            ListBox::Nil => None,
            ListBox::Cons(elem, mut tail) => {
                std::mem::swap(self, &mut tail);
                Some(elem)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum List<A> {
    Nil,
    Cons(A, Rc<List<A>>),
}
impl<A: Clone> List<A> {
    pub fn new() -> Self {
        List::Nil
    }
    pub fn cons(&mut self, elem: A) {
        let tail = std::mem::replace(self, List::Nil);
        let mut list = List::Cons(elem, Rc::new(tail));
        std::mem::swap(self, &mut list)
    }
    pub fn uncons(&mut self) -> Option<A> {
        let list = std::mem::replace(self, List::Nil);
        match list {
            List::Nil => None,
            List::Cons(elem, mut tail) => {
                std::mem::swap(self, Rc::make_mut(&mut tail));
                Some(elem)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TreeBox<A> {
    Leaf,
    Node(Box<TreeBox<A>>, A, Box<TreeBox<A>>),
}
impl<A: Ord> TreeBox<A> {
    pub fn new() -> Self {
        TreeBox::Leaf
    }
    pub fn singleton(value: A) -> Self {
        TreeBox::Node(Box::new(TreeBox::Leaf), value, Box::new(TreeBox::Leaf))
    }
    pub fn insert(&mut self, input: A) {
        match *self {
            TreeBox::Leaf => *self = TreeBox::singleton(input),
            TreeBox::Node(ref mut left, ref value, ref mut right) => {
                if &input < value {
                    left.insert(input);
                } else if &input > value {
                    right.insert(input);
                }
            }
        }
    }
    pub fn find (&self, elem: &A) -> bool {
        match *self {
            TreeBox::Leaf => false,
            TreeBox::Node(ref left, ref value, ref right) => {
                if elem < value {
                    left.find(elem)
                } else if elem > value {
                    right.find(elem)
                } else {
                    true
                }
            }
        }
    }
    pub fn remove_smallest(&mut self) -> Option<A> {
        let node = std::mem::replace(self, TreeBox::new());
        match node {
            TreeBox::Leaf => None,
            TreeBox::Node(mut left, value, right) => {
                if let Some(leftmost) = left.remove_smallest() {
                    *self = TreeBox::Node(left, value, right);
                    Some(leftmost)
                }
                else {
                    *self = *right;
                    Some(value)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tree<A> {
    Leaf,
    Node(Rc<Tree<A>>, A, Rc<Tree<A>>),
}
impl<A: Ord + Clone> Tree<A> {
    pub fn new() -> Self {
        Tree::Leaf
    }
    pub fn singleton(value: A) -> Self {
        Tree::Node(Rc::new(Tree::Leaf), value, Rc::new(Tree::Leaf))
    }
    pub fn insert(&mut self, input: A) {
        match *self {
            Tree::Leaf => *self = Tree::singleton(input),
            Tree::Node(ref mut left, ref value, ref mut right) => {
                if &input < value {
                    Rc::make_mut(left).insert(input);
                } else if &input > value {
                    Rc::make_mut(right).insert(input);
                }
            }
        }
    }
    pub fn find (&self, elem: &A) -> bool {
        match *self {
            Tree::Leaf => false,
            Tree::Node(ref left, ref value, ref right) => {
                if elem < value {
                    left.find(elem)
                } else if elem > value {
                    right.find(elem)
                } else {
                    true
                }
            }
        }
    }
    pub fn remove_smallest(&mut self) -> Option<A> {
        let node = std::mem::replace(self, Tree::new());
        match node {
            Tree::Leaf => None,
            Tree::Node(mut left, value, mut right) => {
                if let Some(leftmost) = Rc::make_mut(&mut left).remove_smallest() {
                    *self = Tree::Node(left, value, right);
                    Some(leftmost)
                }
                else {
                    std::mem::swap(self, Rc::make_mut(&mut right));
                    Some(value)
                }
            }
        }
    }
}



fn main() {
    // {
    //     let mut list = ListBox::new();
    //     for i in 0..10 {
    //         list.cons(CloneTracker(i));
    //     }

    //     // prints "Cloning x..." once
    //     let _clone = list.clone();

    //     list.cons(CloneTracker(20));
    //     assert_eq!(list.uncons(), Some(CloneTracker(20)));

    //     for i in (0..10).rev() {
    //         // prints "Cloning i..."
    //         assert_eq!(list.uncons(), Some(CloneTracker(i)));
    //     }

    //     assert_eq!(list.uncons(), None);
    // }

    // {
    //     let mut list = List::new();
    //     for i in 0..10 {
    //         list.cons(CloneTracker(i));
    //     }

    //     // prints "Cloning x..." once
    //     let _clone = list.clone();

    //     list.cons(CloneTracker(20));
    //     assert_eq!(list.uncons(), Some(CloneTracker(20)));

    //     for i in (0..10).rev() {
    //         // prints "Cloning i..."
    //         assert_eq!(list.uncons(), Some(CloneTracker(i)));
    //     }

    //     assert_eq!(list.uncons(), None);
    // }

    {
        extern crate rand;
        use rand::seq::SliceRandom;

        let mut tree = TreeBox::new();

        // even numbers only.
        let mut numbers: Vec<u32> = (0..50).map(|x| x * 2).collect();
        numbers.shuffle(&mut rand::thread_rng());

        for num in numbers.clone() {
            tree.insert(CloneTracker(num));
        }

        // prints "Cloning x..." 50 times.
        let _clone = tree.clone();

        tree.insert(CloneTracker(47));
        tree.insert(CloneTracker(15));

        for num in numbers {
            assert_eq!(tree.find(&CloneTracker(num)), true);
        }
        assert_eq!(tree.find(&CloneTracker(47)), true);
        assert_eq!(tree.find(&CloneTracker(15)), true);
    }

    {
        extern crate rand;
        use rand::seq::SliceRandom;

        let mut tree = Tree::new();

        // even numbers only.
        let mut numbers: Vec<u32> = (0..50).map(|x| x * 2).collect();
        numbers.shuffle(&mut rand::thread_rng());

        for num in numbers.clone() {
            tree.insert(CloneTracker(num));
        }

        let _clone = tree.clone();

        tree.insert(CloneTracker(47));
        tree.insert(CloneTracker(15));

        for num in numbers {
            assert_eq!(tree.find(&CloneTracker(num)), true);
        }
        assert_eq!(tree.find(&CloneTracker(47)), true);
        assert_eq!(tree.find(&CloneTracker(15)), true);
    }
}
