### Trees.

Consider the following implementation of a tree. For now, let's say that we can only add elements, not delete them. To keep things simple, this is not a balanced tree.

```rust
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
    pub fn find (&self, input: &A) -> bool {
        match *self {
            Tree::Leaf => false,
            Tree::Node(ref left, ref value, ref right) => {
                if input < value {
                    left.find(input)
                }
                else if input > value {
                    right.find(input)
                }
                else {
                    assert!(input == value);
                    true
                }
            }
        }
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
}
```

The tree has the following invariant:

* The tree is sorted.

Which is true when

* The tree is a Leaf, or
* The tree is a Node, and
    * The left tree is sorted, and
    * The right tree is sorted, and
    * The maximum of the left is less than the value of the node, and 
    * The minimum of the right tree is greater than the value of the node.

As long as I use the functions available there, it should be impossible to break the invariant. However if I could suddenly hand out references to individual child nodes that aren't the root of the tree, I could insert values into the right tree which are smaller than the tree's root value, or insert values into the left tree which are larger than the tree's root value. Essentially, it is implied that only the tree root should have mutable access to it's nodes. Everything else can only have immutable access.

If we want to "explore" the structure of the tree, we need to "freeze" it temporarily to ensure that we can't modify any of the internal nodes. Once we have finished looking at the inside, we can unfreeze it and modify it as necessary. We can freeze something in Rust by taking an immutable reference to it. As long as the immutable reference is valid, we cannot modify the underlying data:

```rust
pub struct View<'a, A>{
    stack: Vec<&'a Tree<A>>,
    tree: &'a Tree<A>,
}
impl<'a, A> View<'a, A> {
    pub fn new(tree: &'a Tree<A>) -> Self {
        View {
            stack: Vec::new(),
            tree: tree,
        }
    }

    pub fn go_left(&mut self) -> bool {
        match *self.tree {
            Tree::Leaf => false,
            Tree::Node(ref left, _, _) => {
                self.stack.push(self.tree);
                self.tree = left;
                true
            },
        }
    }
    pub fn go_right(&mut self) -> bool {
        match *self.tree {
            Tree::Leaf => false,
            Tree::Node(_, _, ref right) => {
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
            Tree::Leaf => None,
            Tree::Node(_, ref value, _) => Some(value),
        }
    }
}
```

This way we can explore the tree structure, and look at it's internals with no fear of accidentally invalidating the tree. Unfortunately there isn't really a simpler way of doing this. You *have* to freeze the tree to have access to individual nodes otherwise you can break invariants. If you can break invariants, then the program doesn't work, and we need not concern ourselves with writing programs that don't work.

### Graphs

In most cases we only want one entity to modify a value at a time, so Rust has made this an error.

So in order to 




### The problem with shared mutability


Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---