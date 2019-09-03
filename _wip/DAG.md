---
title: Shared mutability and directed acyclic graphs. (Part 2/3)
category: notes
tags:
- Rust
- graph
- acyclic
- mutability
- references
---

### Interfaces, encapsulation and references.

First of all a disclaimer. I'm going to use the terms such as "interface" and "encapsulation", and I'm going to define them a little bit differently to normal. It may be a bit strange, but I promise that it will make sense.

Instead of writing some code and trying to fix it, let's try creating some good code first try. I won't guarantee that it's performant, but I'll settle for (mostly) correct. To do that, we have to somehow prove that our code works. Generally speaking, we start off logical derivation with some definitions.

We imagine an entity `A` which is part of your program. `A` holds some data, which can either be in a valid state or invalid. If `A` ever manages to get into an invalid state and stay there, then our program is broken. Sometimes we need to break `A` temporarily when we are transitioning from two valid states. To ensure that `A` doesn't stay broken, we draw a box around it. Code inside the box can break `A` and code outside the box can't. `A` is always valid outside the box. The barrier between the two areas, we'll call the **interface**. To make it easy to understand, I'll call the area inside the box the **danger zone** and code outside the box the **safe zone**.

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

Now I won't discuss how to construct an interface, or what it should look like. It's going to be different for every language. Safe to say that we can run any code we like in the safe zone and it will not break `A`, because that's what we defined the interface to be.

Code from the safe zone can call code in the danger zone, but it must go through the interface. When the code from the danger zone returns, `A` must be valid. It must be valid because that's how we defined our interface. In between, `A` could be made invalid temporarily, as long as when we return to the safe zone, we have a valid `A`.

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

Functions that we can call from the safe zone interface but exist inside the danger zone, we will call public functions. A public function is allowed to assume that the entity is valid when it starts, and has a responsibility to ensure that the entity is valid when it exits.

So now we have some idea of what an interface is. Let's talk about **encapsulation**. Let's say that we want to make the danger zone as small as possible so that the amount of code that can break `A` is minimised. I'll define **encapsulation** as an attempt to make the danger zone as small as reasonably possible.

If the danger zone extends out forever and encompasses our entire program, then we always have to worry if we might break `A` by accident when we write code. We don't want to worry about breaking every other part of our program when we modify any other part. That's too much thinking. If we make the danger zone small, then we only have to worry about breaking `A` when we're inside it.

We'll also introduce one more definition. The definition of a **reference**. Lets say we have two entities, a parent and a child. If you can break the parent by modifying the child or making it invalid, we say that the parent holds a reference to the child. We'll define a reference this way whether or not an entity holds a physical pointer to the other entity. I might get slack and say that an entity "knows about" another.

In this article we will only deal with the case where there are no circular references.

### Shared mutability

Consider that we have two entities in out program `A` and `B`. `A` knows about `B`, but `B` doesn't know about `A`. We'll draw this like so:

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

`B` doesn't know about `A`, so the interface for `B` could just be around `B`. The opposite is not true. `A` knows about `B` and could be made invalid if `B` were invalid. To encapsulate `A`, we must surround the danger zone for `B` as well.

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

We can now figure out the correct size for the interface for any entity using the following rules.

* The danger zone for `A` contains `A` itself.
* If an entity `A` knows about another entity `B`, then the danger zone for `A` includes the danger zone for `B`.
* If an entity `A` does not know about another entity `B`, then `B` is outside the danger zone for `A`.

This is not usually a problem, except in the case of shared mutability. Consider two objects `A` and `B` which do no know about each other, but have a shared child called `C` that they both know about. The danger zone for `A` encompasses `A` and `C`, and the danger zone for `B` encompasses `B` and `C`. Because `A` and `B` don't know about each other, they don't belong in each other's danger zones. If it were possible that `B` could cause `A` to break, then it would imply that `A` knows about `B`.

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

Now we can see this issue. The interfaces of `A` and `B` cross over. In order to modify `C` we must go into both the danger zones for `A` and `B` first. This problem only becomes worse if we have more parents.

### Example problem.

For the purposes of this article we will look at a series of entities called **tasks**. Each **task** has a **duration** (in days) and a **name**. A **task** can only be completed after all of it's dependencies have been completed. A task with no dependencies starts straight away. For this article, We'll use the following example:

![Untitled]({{ '/img/invalid.jpg' | relative_url}})

We'll examine two different approaches to this problem.

### Directed Acyclic Graphs (DAG's)

In Safe rust, to have shared mutability in a single threaded context, we can use `RefCell`. `RefCell` allows mutability through shared references, which can be given to any other part of the program. That means that anyone who holds a reference to a child can potentially make invalid any other entity which also holds a reference to it.

Therefore, a `RefCell` interface is a part of a common interface to *every single entity that holds a reference to it*. That means that none of the parents are allowed to *care* what value the child contains, because it could have been modified by something they don't know about. They don't care what the child's value is, as long as it *has* a value.

Essentially this means that we are forced to calculate, on the fly any result that depends on children. This is the most important point if we want to use a `RefCell` based implementation:




Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---