use std::cmp::max;

#[derive(Copy, Clone)]
struct NodeRef(usize);

#[derive(Debug)]
struct AVL<A> {
    data: Vec<Node<A>>;
    cursor: NodeRef;
}
struct Node<A> {
    Null(next: NodeRef),
    Leaf(parent: NodeRef),
    Node(parent: NodeRef, left: NodeRef, value, right: NodeRef, height: i32),
}
impl <A: Ord> Node <A> {
    fn new (parent: NodeRef) {
        AVL::Leaf(parent)
    }
    fn height(&self) -> i32 {
        match *self {
            AVL::Leaf(_) => 0,
            AVL::Node(_, _, _, _, height) => height,
            AVL::Null(_) => panic!("Tried to get height of null tree");
        }
    }
    fn insert (&self, tree: AVL<A>, input: A) -> {
        match *self {
            AVL::Leaf(_) => 0,
            AVL::Node(_, _, _, _, height) => height,
            AVL::Null(_) => panic!("Tried to get height of null tree");
        }
    }
}

impl <A: Ord> AVL<A> {
    pub fn new () -> Self {
        AVL {
            data: Vec::new();
            cursor: NodeRef(0);
        }
    }

    fn new_node (&mut self, parent: NodeRef) -> NodeRef {
        if self.cursor.0 >= data.len() {
            self.data.push()
        }
    }

    pub fn insert (&mut self, input: A) {
        assert!(self.is_avl());
        match *self {
            AVL::Leaf => *self = AVL::singleton(input),
            AVL::Node(ref mut left, ref value, ref mut right, _) => {
                if &input < value {
                    left.insert(input);
                }
                else if &input > value {
                    right.insert(input);
                }
            }
        }
        self.balance();
    }

    /// checks quickly to see if a node hold the avl property, but does not
    /// check recursively.
    fn is_avl(&self) -> bool {
        match *self {
            AVL::Leaf(_) => true,
            AVL::Node(_, ref left, _, ref right, ref height) => {
                let correct_height = max(left.height(), right.height()) + 1 == *height;
                let is_balanced = (left.height() - right.height()).abs() <= 1;
                correct_height && is_balanced
            },
            AVL::Null(_) => panic!("Called is_avl on null"),
        }
    }
}


enum AVL<A> {
    Leaf,
    Node(Box<AVL<A>>, A, Box<AVL<A>>, i32),
}
impl<A: Ord> AVL<A> {
    pub fn new () -> Self {
        AVL::Leaf
    }
    pub fn singleton (value: A) -> Self {
        AVL::node(Box::new(AVL::Leaf), value, Box::new(AVL::Leaf))
    }
    pub fn insert (&mut self, input: A) {
        assert!(self.is_avl());
        match *self {
            AVL::Leaf => *self = AVL::singleton(input),
            AVL::Node(ref mut left, ref value, ref mut right, _) => {
                if &input < value {
                    left.insert(input);
                }
                else if &input > value {
                    right.insert(input);
                }
            }
        }
        self.balance();
    }
    pub fn delete (&mut self, input: &A) {
        assert!(self.is_avl());
        let mut node = AVL::new();
        std::mem::swap(&mut node, self);
        match node {
            AVL::Leaf => (),
            AVL::Node(mut left, value, mut right, _) => {
                if input < &value {
                    left.delete(input);
                    *self = AVL::node(left, value, right);
                }
                else if input > &value {
                    right.delete(input);
                    *self = AVL::node(left, value, right);
                }
                // input == value
                else if let Some(leftmost) = right.remove_left() {
                    *self = AVL::node(left, leftmost, right);
                }
                else if let Some(rightmost) = left.remove_right() {
                    *self = AVL::node(left, rightmost, right);
                }
                // no children, leave self as a leaf.
            }
        }
        self.balance();
    }
    pub fn remove_left(&mut self) -> Option<A>{
        assert!(self.is_avl());
        let mut node = AVL::new();
        std::mem::swap(&mut node, self);
        let result = match node {
            AVL::Leaf => None,
            AVL::Node(mut left, value, right, _) => {
                if let Some(leftmost) = left.remove_left() {
                    *self = AVL::node(left, value, right);
                    Some(leftmost)
                }
                else {
                    *self = *right;
                    Some(value)
                }
            }
        };
        self.balance();
        result
    }
    pub fn remove_right(&mut self) -> Option<A>{
        assert!(self.is_avl());
        let mut node = AVL::new();
        std::mem::swap(&mut node, self);
        let result = match node {
            AVL::Leaf => None,
            AVL::Node(left, value, mut right, _) => {
                if let Some(rightmost) = right.remove_right() {
                    *self = AVL::node(left, value, right);
                    Some(rightmost)
                }
                else {
                    *self = *left;
                    Some(value)
                }
            }
        };
        self.balance();
        result
    }
    pub fn get_left(&self) -> Option<&A> {
        match *self {
            AVL::Leaf => None,
            AVL::Node(ref left, ref value, _, _) => {
                if let Some(leftmost) = left.get_left() {
                    Some(leftmost)
                }
                else {
                    Some(value)
                }
            }
        }
    }
    pub fn get_right(&self) -> Option<&A> {
        match *self {
            AVL::Leaf => None,
            AVL::Node(_, ref value, ref right, _) => {
                if let Some(rightmost) = right.get_right() {
                    Some(rightmost)
                }
                else {
                    Some(value)
                }
            }
        }
    }
    pub fn for_each<F: FnMut(&A)> (&self, func: &mut F) {
        match *self {
            AVL::Leaf => (),
            AVL::Node(ref left, ref value, ref right, _) => {
                left.for_each(func);
                func(value);
                right.for_each(func);
            }
        }
    }
    
    
    
    fn node(left: Box<AVL<A>>, value: A, right: Box<AVL<A>>) -> Self {
        let height = max(left.height(), right.height()) + 1;
        AVL::Node(left, value, right, height)
    }
    fn height(&self) -> i32 {
        match *self {
            AVL::Leaf => 0,
            AVL::Node(_, _, _, height) => height,
        }
    }
    
    /// checks quickly to see if a node hold the avl property, but does not
    /// check recursively.
    fn is_avl(&self) -> bool {
        match *self {
            AVL::Leaf => true,
            AVL::Node(ref left, _, ref right, ref height) => {
                let correct_height = max(left.height(), right.height()) + 1 == *height;
                let is_balanced = (left.height() - right.height()).abs() <= 1;
                correct_height && is_balanced
            }
        }
    }
    /// checks to see if the node holds the avl property
    fn is_avl_full(&self) -> bool {
        match *self {
            AVL::Leaf => true,
            AVL::Node(ref left, ref value, ref right, ref height) => {
                let correct_height = max(left.height(), right.height()) + 1 == *height;
                let is_balanced = (left.height() - right.height()).abs() <= 1;
                let is_sorted_left = left.get_right().map_or(true, |l| l < value);
                let is_sorted_right = right.get_left().map_or(true, |r| r > value);
                let children_are_avl = left.is_avl_full() && right.is_avl_full();
                
                correct_height && is_balanced && is_sorted_left && 
                    is_sorted_right && children_are_avl
            }
        }
    }
    
    /// positive number for right heavy, negative for left heavy. 
    /// Readjusts height too
    fn get_balance(&mut self) -> i32 {
        match *self {
            AVL::Leaf => 0,
            AVL::Node(ref left, _, ref right, ref mut height) => {
                let l_height = left.height();
                let r_height = right.height();
                *height = max(l_height, r_height) + 1;
                right.height() - left.height()
            }
        }
    }
    fn rotate_left(&mut self) {
        let mut node = AVL::new();
        let mut node_child = AVL::new();
        std::mem::swap(&mut node, self);
        
        if let AVL::Node(left, left_val, mut child, _) = node {
            std::mem::swap(&mut node_child, &mut child);
            
            if let AVL::Node(middle, right_val, right, _) = node_child {
                *child = AVL::node(left, left_val, middle);
                assert!(child.is_avl());
                *self = AVL::node(child, right_val, right);
                assert!(self.is_avl());
            }
            else {
                panic!("Tree could not be rotated left")
            }
        }
        else {
            panic!("Tree could not be rotated left")
        }
    }
    fn rotate_right(&mut self) {
        let mut node = AVL::new();
        let mut node_child = AVL::new();
        std::mem::swap(&mut node, self);
        
        if let AVL::Node(mut child, right_val, right, _) = node {
            std::mem::swap(&mut node_child, &mut child);
            
            if let AVL::Node(left, left_val, middle, _) = node_child {
                *child = AVL::node(middle, right_val, right);
                assert!(child.is_avl());
                *self = AVL::node(left, left_val, child);
                assert!(self.is_avl());
            }
            else {
                panic!("Tree could not be rotated right")
            }
        }
        else {
            panic!("Tree could not be rotated right")
        }
    }
    
    /// it is assumed that the children hold the AVL property. This node may not
    /// have the AVL property or the correct height
    fn balance(&mut self) {
        let balance = self.get_balance();
        if balance.abs() <= 1 {
            return;
        }
        else if balance > 1 {
            if let AVL::Node(_, _, ref mut right, _) = *self {
                if right.get_balance() < 0 {
                    right.rotate_right();
                    assert!(right.is_avl());
                }
            }
            else {
                panic!("Node is right heavy but has no right child");
            }
            self.rotate_left();
        }
        else if balance < 1 {
            if let AVL::Node(ref mut left, _, _, _) = *self {
                if left.get_balance() > 0 {
                    left.rotate_left();
                    assert!(left.is_avl());
                }
            }
            else {
                panic!("Node is left heavy but has no left child");
            }
            self.rotate_right();
        }
        assert!(self.is_avl());
    }
}


fn main () {
    let mut tree = AVL::new();
    for x in 0..20 {
        tree.insert(x);
    }
    tree.for_each(&mut |value| println!("{}", value));
    assert!(tree.is_avl_full());
    
    for x in 0..10 {
        tree.delete(&x);
    }
    tree.for_each(&mut |value| println!("{}", value));
    assert!(tree.is_avl_full());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_order_insertion () {
        let mut tree = AVL::new();
        for x in 0..100 {
            tree.insert(x);
        }
        assert_eq!(tree.get_left(), Some(&0));
        assert_eq!(tree.get_right(), Some(&99));
        assert!(tree.is_avl_full());
        
        for x in 0..50 {
            tree.delete(&x);
        }
        assert_eq!(tree.get_left(), Some(&50));
        assert_eq!(tree.get_right(), Some(&99));
        assert!(tree.is_avl_full());
    }
}