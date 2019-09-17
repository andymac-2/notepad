---
title: Structuring your Rust program from first principles.
category: notes
tags:
- Rust
- induction
- proof
- structure
- graph
- references
---

### Or "6 ways to create a data structure with cycles in Rust"

## Introduction

Rust is a language with a strong focus on safety. However, due to it's strictness, sometimes it can be difficult to write the kinds of programs we want to. Specifically, structures which involve shared mutability, or reference cycles. This article discusses from first principles some methods to write complex objects which have arbitrary structure, including some special cases, and the reasoning behind these design choices.

First and foremost: we shouldn't concern ourselves with programs that don't work. Simply, if we were to know that it would take 100 lines of code to perform a task correctly, but we have only written 50, we know that the program doesn't do what it is supposed to do. In any reasonable circumstance, programmers do not have the time or resources to comprehensively prove that a given piece of software works, so we will not examine this possibility in much detail here. However we will discuss the *opposite* possibilities: The possibility where we can prove that a given piece of software *doesn't* work, or the possibility that we can't prove that the software *does* work.

For the purposes of this article, we will use Rust, since it is a language with a design focus on safety, making it fitting for this purpose, however, the ideas should be able to be carried over into other languages as desired.

## Required knowledge: Invariants.

As the name suggests, an invariant is something that never changes. For example, "The program works" should be an invariant of your program, if it isn't an invariant, either you, or your program has done something wrong. We can only rely on something being true if it's an invariant, and we can not rely on something being true if it isn't an invariant.

The "this program works" invariant is going to depend on other invariants in your program. If you know that your program could sometimes violates it's invariants, then there is not guarantee that "this program works". As an every day programmer, you will not be expected to formally and mathematically prove that the invariants hold; however, it's usually sufficient to know that you *could* prove it if you wanted, and avoid circumstances where you *couldn't* prove it even if you wanted.

For example, consider a binary tree. To keep things simple, this will not be a balanced tree:

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

We say that something is a tree if it is of the type `Tree` and if it upholds the tree invariants. The invariant here is that the tree is always sorted. If we wanted to prove that a tree is sorted we could use the following rules which we will refer to later:

1. An empty tree is already sorted.
2. A Tree with a single value is already sorted.
3. Any other node is sorted if:
    * The node's left child is sorted, and 
    * The node's right child is sorted, and
    * All of the values in the left child are less than the value of the node, and
    * All of the values in the right child are greater than the value of the node.

We can write a function to check the above automatically, note that `get_leftmost` is omitted for brevity, it's an almost identical copy of `get_rightmost`:

```rust
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
// Checks to see if this is a proper tree recursively.
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
// Checks to see if the current node is correct, but does not check recursively.
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
```

If being sorted is a true invariant of trees, then we should be able to run `is_tree_full` or `is_tree`, and it should always return `true`.

## Required knowledge: Encapsulation

Using our example above of a tree, there are two functions available to create a new tree from scratch: `new` which creates an empty tree, and `singleton` which creates a tree with a single element. Using rules 1 and 2 above, we know that trees created using `new` or `singleton` are already sorted.

The only other function left is `insert`, for which we can write the following:

* If we have an sorted tree, and we add another element to it, then the result should also be a sorted tree.

In code we could write this as

```rust 
pub fn insert (&mut self, input: A) {
    // At the beginning of the function, the tree should be sorted.

    // Between the start and the end of the function, we may temporarily 
    // break invariants
    *** Perform the actual insertion. ***

    // At the end of the function, the tree should also be sorted.
}
```

Comments are not ideal for this purpose. Comments can lie, be ignored, or become outdated. There is however, one kind of comment that *cannot* be ignored: an assertion.

```rust
pub fn insert (&mut self, input: A) {
    assert!(self.is_tree());

    *** Perform the actual insertion. ***

    assert!(self.is_tree());
}
```

We notice that we actually *cannot* create a tree from outside this module that isn't sorted. Either we have to use `singleton` or `new` which only produce sorted trees, or we have to use `insert` which will only return successfully if the assertion passes. If my `insert` function is correct, then we modify the tree successfully and it's sorted. If I make a mistake inside the `insert` function, then the program terminates before it can make an invalid tree.

If I cannot make an invalid tree from outside of this module, then the only place I can make an invalid tree is if I make a mistake *inside* the module. If any of my assertions fail, or my code has a bug that involves a malformed tree, I know exactly where in my code I have to look.

In addition, we can also say that code outside the module has no responsibility to ensure that the invariants hold; it is purely the responsibility of code inside the module to ensure that the invariants are true at all times.

Inside the module we can break the invariants, outside we can't. The boundary between the inside and the outside, we call the **interface**. We say that the tree is **encapsulated** because we have limited places where we can create incorrect code.

***img***

If this sounds familiar to you, then you're right. Safe Rust already promises to uphold several invariants for you. Specifically, these are memory safety, freedom from data races, and no undefined behaviour. Code written in unsafe Rust can violate these invariants. If these invariants are violated, we know that the offending code must be inside an `unsafe` block.

Rust therefore provides encapsulation around `unsafe`. There is an area of code where we can't violate the invariants, there is an are of code where we can, and we make an attempt to minimise that area of code. In addition, it is not the responsibility of code inside of a safe rust to check if code inside `unsafe` is correct.

Note that we don't actually need any objects to have encapsulation, we just need invariants.

## A quick note about Graphs

A `struct` is comprised broadly of two different things: data that it references, and data that it owns. References are unidirectional: if `struct` A holds a reference to `struct` B, then `struct` B does not need to have a reference to `struct` A. The data of a Rust program is comprised of these `structs`.

Broadly speaking, a data structure which is comprised of a number of individual components which are connected with unidirectional edges is called a directed graph. Therefore, if we can create an arbitrary directed graph data structure, then we should be able to create any data structure that we could create in Rust.

## Trees

Armed with knowledge about In the simplest case, Our data forms a tree, and has no cyclic references. Rust makes writing these kinds of structures easy. In this case, the root node owns all of it's direct children. The children own all of their direct children and so on and so forth. If you have the opportunity to structure your program this way, consider doing so.

For any function that requires data from both the parent and the child, you can either make it a method of the parent, or you can pass the required data from the parent as an argument to the child. The following code shows a basic example of both variants:

```rust
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
```

You should pass data from the parent as arguments to the child when

* You need the code to work if the child has a different type of parent, or no parent at all, or
* You need to temporarily break the invariant of the child to perform the functionality required.

You should make the method belong to the parent if

* The method is specific to the parent/child combination, or
* You don't need to temporarily break the invariants of the child.

## Interlude: circular references.

In the simplest case, consider a parent object and a child object. The parent object holds a reference to the child, and the child holds a reference to the parent. The parent depends on the child; if we were to modify the child, we may break the invariants of the parent. Similarly, the child depends on the parent; if we modify the parent, we may break the invariants of the child.

***img***

In this case, we may as well concede that we haven't really limited the area of code where we can break invariants. Every time I modify the child object, to know that my code works, I would have to check the implementation of the parent to make sure I haven't broken invariants and vice versa. If I haven't checked the implementation of the parent object each time I modify the child, then I can't tell that my code works.

In this particular example, we concede that the parent and the child are strongly coupled. They should probably be made into a single object.

## Trees (again)

There is a special case of trees, where some children may have a reference to a parent.


## General case: graphs.    






![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
