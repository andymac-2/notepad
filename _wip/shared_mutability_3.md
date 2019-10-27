---
title: "Shared mutability in Rust Part 3: Reference cycles"
category: posts
tags:
- Rust
- Mutability
- Cycles
- References
- Invariants
---

## Summary


## Defining our problem

Rust, in its focus on safety sometimes places some seemingly arbitrary restrictions on the programmer. In particular, it is notoriously difficult to create data structures which have circular references. The previous two parts discussed shared mutability, and arbitrary acyclic graphs. However, it specifically ignored the case of reference cycles. In this part, the discussion will tackle reference cycles head on.

Consider two entities `A` and `B` who both know about each other. We can draw this like so:

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

We can see immediately we have a cycle. `A` and `B` are shown as equals. It is not immediately obvious which one is a parent and which one is a child. If `B` is modified incorrectly, `A` could be put into an invalid state. Similarly if `A` is modified incorrectly, `B` could be put into an invalid state. Whenever methods for `A` are written, it's effect on `B` will have to be considered and vice versa.

At this point it may be useful to consider that `A` and `B` are intertwined, and thus are actually two different parts of the same entity (say `R` for "root"). There is no loss by thinking this way, it's just a different perspective.

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

When methods on `A` are called, the program modifies `R` from *the point of view of* `A`, and methods on `B` are called, the program modifies `R` from *the point of view of* `B`. This thought process can be formalised. It is possible to create an entity which represents a view of some data structure. This entity emulates shared mutability. This is one way to model reference cycles in Rust: have one (or a few) large objects which have multiple ways to interact with them. Note that so far, this is purely a discussion of what the interface will look like rather than the actual implementation.

## Children and parents

Questions regarding 

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
