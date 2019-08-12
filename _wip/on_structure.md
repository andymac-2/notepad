---
title: On Structure
category: notes
tags:
- rust
- structure
- pattern
- programming
- haskell
---

### Rust and Haskell are hard to program

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

One thing that these two languages have in common is that the languages have strict static guarantees. In human terms this means that you will have a hard time trying to compile them with all the errors they end up throwing at you. I will focus on Rust. Disclaimer: what follows is mostly opinion.

Eventually you will fins yourself in a situation where you have to "fight" the borrow checker in Rust to get your program to compile. You might be trying to program something that you know should work, but the compiler doesn't let you. Is the compiler just being a pain in the ass? or is it trying to tell you something deeper? I argue it's the latter, and I present ways in which to organise your program in a way that makes it easier for you as a programmer to reason about, and to pass rusts strict compile time checks.

Rust tends to introduce "resistance" to certain types of programming which are common in other languages, and subtly tries to encourage you to write your programs in certain ways. This post documents what I think the rust compiler is trying to "encourage" me to do and why.

### Mutability

Many times, you will come across a situation where multiple objects should be able to modify a single object. You might think to quickly reach for `Rc<RefCell<T>>` or worse: `unsafe`. In my opinion, `Rc<RefCell<T>>` indicates that there may be a problem with the program structure, and it can introduce unpredictability in your program. `unsafe` should usually be reserved for optimisations later on, as (most) useful abstractions already have a safe interface.

Typically if someone holds a regular reference to data, they will expect the underlying data not to change. Consider the following example:

```rust
let my_value = &something;
let a = my_value.do_something();
let b = my_value.do_something();
```

We would normally expect `a` and `b` to be the same result, and we expect `do_something()` to do the same thing both times. Normally this is the case, however, due to the interior mutability pattern, we cannot guarantee this. Interior mutability allows programmers to modify the interior of a structure using an immutable reference. In the case of using `RefCell`, the borrow checking rules are checked at runtime. When you use `RefCell`, it implies that you have additional knowledge that the compiler doesn't.

Note that sometimes the code is not as obvious as in the above example. Two different structures in distant parts of the program may have immutable references to the same thing, and each structure may not have any knowledge of the other. In that case, if we were to implement interior mutability, we may break the expectations of the programmer who expects `do_something()` to behave predictably.

In my opinion, I feel the language is trying to tell me that `RefCell` and/or the interior mutability pattern should be used almost exclusively for caching. If there is some expensive calculation that occurs when you call `do_something()`, and you only want to perform it once, you could modify `my_value` to cache the result of the calculation. On the exterior, it appears that the value has remained unchanged, but we have saved time when we call it a second time.

Alternatively, Some data structures can perform structural optimisations during queries. For example, a *disjoint set data structure* using *path compression*. Without going into too much detail, the path compression operation reduces the time it takes for future queries, without modifying the shape, or invariants of the structure. As long as the structure appears unchanged on the exterior, we aren't breaking any programmer expectations.

In short, use `RefCell` and the interior mutability pattern as an optimisation, rather than a go-to solution for inconvenience caused by the rust compiler.

### Cycles

Rust *hates* cycles. If you have reference cycles, seriously consider whether you actually need them. They make code harder to reason about, and can be a source of potential memory leaks in Rust. 

In the most basic case, you will have a parent structure and a child structure. The parent owns the child, and for convenience sake, you will have a reference from the child to the parent. Rust doesn't like this. You could put the parent in an `Rc` or `Arc`, and then give the child a reference, but now you have created a circular reference and a possible memory leak.

To solve the memory leak problem, you could use `Weak` references. This does indeed solve some problems. 

In addition, `Rc` is by default immutable. If you want to modify either the parent or the child. 

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).
