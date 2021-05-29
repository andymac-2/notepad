---
title: Implement inheritance in Rust
category: notes
tags:
- here
- are
- some
- tags.
---

## Implement Inheritance in Rust

Alternative title: the inheritance (anti)pattern in Rust.

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

### Summary

- Point 1
- Point 2

# Rust does not have inheritance

...at least not natively. This is a problem for a lot of programmers who are migrating from other languages who want to map their existing ideas or programs one-to-one into Rust. This article comes with a disclaimer: some of the code that will be presented is extremely unidiomatic rust, or just plain bad. This code should be used with extreme caution.

## Requirements

If you're just looking for a solution that maps existing programs on-to-one, then we will need a pattern that supports the bulk of the functionality of inheritance in most languages. In this article I aim for the following requirements:

- A child class must be able to access all of the fields of the parent class.
- A class can access it's parent class implementation. (some equivalent to the `super` keyword)
- Virtual and non-virtual methods are supported
- Abstract methods should be supported. Failing to implement an abstract method should at lest result in a runtime error.
- Multiple inheritance is supported.
- Homogenous collections are supported.

What I won't try to implement are:

- Access specifiers: `private`, `protected`, etc. Rust already has a far better system in place for idiomatic code. If you willingly enter into using this antipattern, then you willingly forego proper access control.
- Static methods: these can be implemented using associated, or just free functions. They're not really worth writing about.
- Macros: There is a lot of boilerplate here which I will make no attempt to reduce. You should write idiomatic code instead. Implementing macros are left as an exercise to the reader.

## Concretions

Concretions are easy in any language. Say we have a base class called `Animal`. The base class has two fields, `height` which us a number, and `name` which is a string. In any class that inherits from `Animal` these members will be present.

```rust
struct ConcreteAnimal {
    height: f64,
    name: String,
}

impl ConcreteAnimal {
    // Don't implement this if the base class is abstract
    fn new(height: f64, name: String) -> Self {
        ConcreteAnimal { height, name }
    }
}
```

A `new` function is present to construct an instance of the base class. If the class is abstract, this should be omitted.

## Inheritance

If the base class is not `final` then it does not represent a single type. It could represent any of it's child types as well.

There should be some way to access the parent class implementation from a child. Normally, this is achieved using using the `super` keyword or similar. In Rust we will have to implement this ourselves. To do this, we create an `Animal` trait with some methods to access the parent implementation:

```rust
trait Animal: AsRef<ConcreteAnimal> + AsMut<ConcreteAnimal> + Into<ConcreteAnimal> {
    type Parent: Animal;
    fn parent(&self) -> &Self::Parent;
    fn parent_mut(&self) -> &mut Self::Parent;
    fn into_parent(self) -> Self::Parent;

    // ... other methods
}
```

I use `parent` instead of `super` because `super` is a reserved word in Rust. The `AsRef`, `AsMut` and `Into` trait bounds are required so that we can access the concrete base implementation if required. Note that the parent is not a `ConcreteAnimal` This is for a number of reasons:

- If you were to have multiple levels of inheritance, the parent will not always be the base `Animal` class.
- This will allow us to have abstract base classes which will be discussed later.

## Virtual methods

Virtual functions are a bit trickier than final ones. In Rust, traits are used to implement a common interface, however, they can only have one default. In an inheritance based system, a virtual function's default implementation is whatever the parent class defined it as. And if that parent has no implementation, then it is the parent's parent's implementation and so on and so forth. That means when we define our trait, all of our methods should have a default implementation that simply calls the parent implementation. Say our `Animal` class has 

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
