#[derive(Debug)]
pub enum Tree<A> {
    Leaf,
    Node(Box<Tree<A>>, A, Box<Tree<A>>),
}
impl<A: Ord> Tree<A> {
    pub fn new () -> Self {
        Tree::Leaf
    }
    pub fn singleton (value: A) -> Self {
        Tree::Node(Box::new(Tree::Leaf), value, Box::new(Tree::Leaf))
    }
    pub fn insert (&mut self, input: A) {
        match *self {
            Tree::Leaf => *self = Tree::singleton(input),
            Tree::Node(ref mut left, ref value, ref mut right) => {
                if &input < value {
                    left.insert(input);
                }
                else if &input > value {
                    right.insert(input);
                }
            }
        }
    }
    pub fn delete (&mut self, input: &A) -> bool {
        assert!(self.is_tree());
        let node = std::mem::replace(self, Tree::new());
        match node {
            Tree::Leaf => false,
            Tree::Node(mut left, value, mut right) => {
                if input < &value {
                    let result = left.delete(input);
                    *self = Tree::Node(left, value, right);
                    return result;
                }
                else if input > &value {
                    let result = right.delete(input);
                    *self = Tree::Node(left, value, right);
                    return result;
                }
                // input == value
                else if let Some(leftmost) = right.remove_leftmost() {
                    *self = Tree::Node(left, leftmost, right);
                    return true;
                }
                else if let Some(rightmost) = left.remove_rightmost() {
                    *self = Tree::Node(left, rightmost, right);
                    return true;
                }
                // no children, leave self as a leaf.
                false
            }
        }
    }
    pub fn remove_leftmost(&mut self) -> Option<A>{
        assert!(self.is_tree());
        let node = std::mem::replace(self, Tree::new());
        let result = match node {
            Tree::Leaf => None,
            Tree::Node(mut left, value, right) => {
                if let Some(leftmost) = left.remove_leftmost() {
                    *self = Tree::Node(left, value, right);
                    Some(leftmost)
                }
                else {
                    *self = *right;
                    Some(value)
                }
            }
        };
        result
    }
    pub fn remove_rightmost(&mut self) -> Option<A>{
        assert!(self.is_tree());
        let node = std::mem::replace(self, Tree::new());
        let result = match node {
            Tree::Leaf => None,
            Tree::Node(left, value, mut right) => {
                if let Some(rightmost) = right.remove_rightmost() {
                    *self = Tree::Node(left, value, right);
                    Some(rightmost)
                }
                else {
                    *self = *left;
                    Some(value)
                }
            }
        };
        result
    }
    pub fn get_leftmost(&self) -> Option<&A> {
        match *self {
            Tree::Leaf => None,
            Tree::Node(ref left, ref value, _) => {
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
            Tree::Leaf => None,
            Tree::Node(_, ref value, ref right) => {
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
            Tree::Leaf => (),
            Tree::Node(ref left, ref value, ref right) => {
                left.for_each(func);
                func(value);
                right.for_each(func);
            }
        }
    }
    
    /// checks quickly to see if a node hold the Tree property, but does not
    /// check recursively.
    fn is_tree(&self) -> bool {
        match *self {
            Tree::Leaf => true,
            Tree::Node(ref left, ref value, ref right) => {
                let is_sorted_left = left.get_rightmost().map_or(true, |l| l < value);
                let is_sorted_right = right.get_leftmost().map_or(true, |r| r > value);
                is_sorted_left && is_sorted_right
            }
        }
    }
    /// checks to see if the node holds the Tree property recursively
    fn is_tree_full(&self) -> bool {
        match *self {
            Tree::Leaf => true,
            Tree::Node(ref left, ref value, ref right) => {
                let is_sorted_left = left.get_rightmost().map_or(true, |l| l < value);
                let is_sorted_right = right.get_leftmost().map_or(true, |r| r > value);
                let children_are_tree = left.is_tree_full() && right.is_tree_full();
                is_sorted_left && is_sorted_right && children_are_tree
            }
        }
    }
}

fn main () {
    let mut tree = Tree::new();
    for x in 0..10 {
        tree.insert(x);
    }
    assert!(tree.is_tree_full());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_order_insertion () {
        let mut tree = Tree::new();
        for x in 0..100 {
            tree.insert(x);
        }
        assert_eq!(tree.get_leftmost(), Some(&0));
        assert_eq!(tree.get_rightmost(), Some(&99));
        assert!(tree.is_tree_full());
        
        for x in 0..50 {
            tree.delete(&x);
        }
        assert_eq!(tree.get_leftmost(), Some(&50));
        assert_eq!(tree.get_rightmost(), Some(&99));
        assert!(tree.is_tree_full());
    }
}