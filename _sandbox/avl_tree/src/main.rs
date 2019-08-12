use std::cmp::max;

#[derive(Debug)]
pub enum AVL<A> {
    Leaf,
    Node(Box<AVL<A>>, A, Box<AVL<A>>, i32),
}
pub struct AVLView<'a, A>{
    stack: Vec<&'a AVL<A>>,
    tree: &'a AVL<A>,
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
                else if let Some(leftmost) = right.remove_leftmost() {
                    *self = AVL::node(left, leftmost, right);
                }
                else if let Some(rightmost) = left.remove_rightmost() {
                    *self = AVL::node(left, rightmost, right);
                }
                // no children, leave self as a leaf.
            }
        }
        self.balance();
    }
    pub fn remove_leftmost(&mut self) -> Option<A>{
        assert!(self.is_avl());
        let node = std::mem::replace(self, AVL::new());
        let result = match node {
            AVL::Leaf => None,
            AVL::Node(mut left, value, right, _) => {
                if let Some(leftmost) = left.remove_leftmost() {
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
    pub fn remove_rightmost(&mut self) -> Option<A>{
        assert!(self.is_avl());
        let node = std::mem::replace(self, AVL::new());
        let result = match node {
            AVL::Leaf => None,
            AVL::Node(left, value, mut right, _) => {
                if let Some(rightmost) = right.remove_rightmost() {
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
    pub fn get_leftmost(&self) -> Option<&A> {
        match *self {
            AVL::Leaf => None,
            AVL::Node(ref left, ref value, _, _) => {
                if let Some(leftmost) = left.get_leftmost() {
                    Some(leftmost)
                }
                else {
                    Some(value)
                }
            }
        }
    }
    pub fn get_rightmost(&self) -> Option<&A> {
        match *self {
            AVL::Leaf => None,
            AVL::Node(_, ref value, ref right, _) => {
                if let Some(rightmost) = right.get_rightmost() {
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
    fn unwrap (self) -> (Box<AVL<A>>, A, Box<AVL<A>>, i32) {
        match self {
            AVL::Node(left, value, right, height) =>
                (left, value, right, height),
            AVL::Leaf => panic!("Unexpected leaf"),
        }
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
                let is_sorted_left = left.get_rightmost().map_or(true, |l| l < value);
                let is_sorted_right = right.get_leftmost().map_or(true, |r| r > value);
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
        let node = std::mem::replace(self, AVL::new());
        let (left, left_val, mut child, _) = node.unwrap();

        let node_child = std::mem::replace(&mut *child, AVL::new());
        let (middle, right_val, right, _) = node_child.unwrap();

        *child = AVL::node(left, left_val, middle);
        assert!(child.is_avl());
        *self = AVL::node(child, right_val, right);
        assert!(self.is_avl());
    }
    fn rotate_right(&mut self) {
        let node = std::mem::replace(self, AVL::new());
        let (mut child, right_val, right, _) = node.unwrap();

        let node_child = std::mem::replace(&mut *child, AVL::new());
        let (left, left_val, middle, _) = node_child.unwrap();

        *child = AVL::node(middle, right_val, right);
        assert!(child.is_avl());
        *self = AVL::node(left, left_val, child);
        assert!(self.is_avl());
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

impl<'a, A> AVLView<'a, A> {
    pub fn new(tree: &'a AVL<A>) -> Self {
        AVLView {
            stack: Vec::new(),
            tree: tree,
        }
    }

    pub fn go_left(&mut self) -> bool {
        match *self.tree {
            AVL::Leaf => false,
            AVL::Node(ref left, _, _, _) => {
                self.stack.push(self.tree);
                self.tree = left;
                true
            },
        }
    }
    pub fn go_right(&mut self) -> bool {
        match *self.tree {
            AVL::Leaf => false,
            AVL::Node(_, _, ref right, _) => {
                self.stack.push(self.tree);
                self.tree = right;
                true
            },
        }
    }
    pub fn go_up(&mut self) -> bool {
        match self.stack.pop() {
            None => false,
            Some(tree) => {
                self.tree = tree;
                true
            },
        }
    }
    pub fn value(&self) -> Option<&A> {
        match *self.tree {
            AVL::Leaf => None,
            AVL::Node(_, ref value, _, _) => Some(value),
        }
    }
}

pub enum AVLListView<'a, A> {
    Cons(&'a AVL<A>, Box<AVLListView<'a, A>>),
    Single(&'a AVL<A>)
}
impl<'a, A> AVLListView<'a, A> {
    pub fn new(tree: &'a AVL<A>) -> Self {
        AVLListView::Single(tree)
    }
    fn head (&self) -> &'a AVL<A> {
        match *self {
            AVLListView::Single(head) => head,
            AVLListView::Cons(head, _) => head,
        }
    }
    fn uncons(self) -> Option<(&'a AVL<A>, Self)> {
        match self {
            AVLListView::Single(_) => None,
            AVLListView::Cons(head, tail) => Some((head, *tail)),
        }
    }
    fn push (&mut self, tree: &'a AVL<A>) {
        let tail = std::mem::replace(self, AVLListView::Single(tree));
        let mut list = AVLListView::Cons(tree, Box::new(tail));
        std::mem::swap(self, &mut list);
    }
    fn pop (&mut self) -> Option<&'a AVL<A>> {
        let list = std::mem::replace(self, AVLListView::new(self.head()));
        if let Some((head, mut tail)) = list.uncons() {
            std::mem::swap(self, &mut tail);
            Some(head)
        }
        else {
            None
        }
    }

    pub fn go_left(&mut self) -> bool {
        match *self.head() {
            AVL::Leaf => false,
            AVL::Node(ref left, _, _, _) => {
                self.push(left);
                true
            },
        }
    }
    pub fn go_right(&mut self) -> bool {
        match *self.head() {
            AVL::Leaf => false,
            AVL::Node(_, _, ref right, _) => {
                self.push(right);
                true
            },
        }
    }
    pub fn go_up(&mut self) -> bool {
        match self.pop() {
            None => false,
            Some(_) => true,
        }
    }
    pub fn value(&self) -> Option<&A> {
        match self.head() {
            AVL::Leaf => None,
            AVL::Node(_, ref value, _, _) => Some(value),
        }
    }
}


fn main () {
    let mut tree = AVL::new();
    for x in 0..10 {
        tree.insert(x);
    }
    assert!(tree.is_avl_full());

    let mut view = AVLView::new(&tree);
    println!("{:?}", view.tree);
    view.go_left();
    println!("{:?}", view.tree);
    view.go_left();
    println!("{:?}", view.tree);
    view.go_left();
    println!("{:?}", view.tree);
    view.go_up();
    println!("{:?}", view.tree);
    view.go_up();
    println!("{:?}", view.tree);
    view.go_up();
    println!("{:?}", view.tree);

    let mut view = AVLListView::new(&tree);
    println!("{:?}", view.head());
    view.go_left();
    println!("{:?}", view.head());
    view.go_left();
    println!("{:?}", view.head());
    view.go_left();
    println!("{:?}", view.head());
    view.go_up();
    println!("{:?}", view.head());
    view.go_up();
    println!("{:?}", view.head());
    view.go_up();
    println!("{:?}", view.head());
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
        assert_eq!(tree.get_leftmost(), Some(&0));
        assert_eq!(tree.get_rightmost(), Some(&99));
        assert!(tree.is_avl_full());
        
        for x in 0..50 {
            tree.delete(&x);
        }
        assert_eq!(tree.get_leftmost(), Some(&50));
        assert_eq!(tree.get_rightmost(), Some(&99));
        assert!(tree.is_avl_full());
    }
}