---
title: On Types
category: notes
tags:
- types
- inference
- logic
- rust
_ generics
---

**Summary:**

Attempt to write programs using generic types through trait interfaces as much as possible:

* It tells the next programmer who uses your code more about your function that if you had used concrete types.
* It minimises the number of incorrect programs that you can write.
* It facilitates code reuse.

### Let's play a game...

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

It's called "guess what this function does". And all you have to go by is it's type signature.

Let's start out with some of the most obvious examples. You will find these ones in an `impl` block.

```rust 
fn my_func () -> Self  { ... }
```

Simple enough. This is a constructor for whatever the type `Self` is. `Self` is a concrete type. This function could also panic, perform some IO, or loop forever. Since every function could loop forever, panic, or perform some IO, I'm going to ignore that for the rest of this post. Safe to say, that if this function returns, we have created something of type `Self`.

```rust
fn my_func (&mut self)  { ... }
```

Still very simple. This function is going to modify whatever `self` is. We don't know how it is going to modify it, but we know that it probably will. If it didn't need to modify the `self` argument, then we could've written the function signature with `&self` rather than `&mut self`.


```rust
fn my_func (self) { ... }
```

A little more complicated but still fairly simple. We give `my_func` ownership of `self`, and we don't get anything back. It must be a destructor. Whatever happens inside of that function will break class invariants or destroy `self`, therefore the object will have to be destroyed.

```rust
fn my_func (&self) -> Something { ... }
```

This function takes an immutable reference, and returns `Something`. It's most likely a query unless `self` implements the interior mutablity pattern. It's possible that we could ignore the `self` argument and return a constant value, but doing so will result in compiler warnings.

```rust
fn my_func (&self) { ... }
```

Hmm. This function takes an immutable reference, but doesn't return anything. Most likely this function is going to perform some IO based on the value of `self`. Alternatively, `self` could implement the interior mutability pattern, and we could be modifying values inside `self`. I would be cautious about using the interior mutability pattern, because programmers may expect that if a function takes an immutable reference, it doesn't mutate the value.

### Intermission: empty types.

Let's look at something slightly different. Here's an example of an `enum`:

```rust
enum Boolean {
    True,
    False,
}
```

We don't define booleans this way in rust, but we could if we really wanted to. Boolean has two variants. `True` and `False`. Booleans are useful for lots of things. We could say that the "value" of the `Boolean` type is two because it has two possible values.

```rust
enum Unit {
    Unit
}
```

This seems pretty useless at first. It's an `enum` with only one variant. We call it `Unit` because unit means an individual thing. There is only one way to create it, and it only has one possible value. Unlike `Boolean` which takes up a minimum of one bit of memory, `Unit` might not need to take up *any* memory, since we already know what the value of it is going to be.

If we accidentally try to create a value of `Unit` incorrectly we get an error:

```rust
fn my_unit_func () -> Unit {
    Unit
}
```

leads to:

```
  --> types.rs:19:5
   |
10 |     Unit
   |     ^^^^
   |
   = note: did you mean to use one of the following variants?
           - `Unit::Unit`
```

`rustc` tells us that I need to use `Unit::Unit` instead of just `Unit`. The compiler helps us by providing a suggestion to fix it.

Can we get more extreme? Yes:

```rust
enum Void {}
```

`Void` has *no* values. Unlike `Unit`, we have no way of creating a value that has a type `Void`. The "value" of `Void` is therefore zero, there are zero possible values for `Void`. If we try to instantiate a value of type `Void`, `rustc` (1.33.0) gets confused:

```rust
fn my_void_func () -> Void {
    Void
}
```

leads to:

```
error[E0423]: expected value, found enum `Void`
  --> types.rs:15:5
   |
15 |     Void
   |     ^^^^
   |
   = note: did you mean to use one of the following variants?

    * empty space *

```

The compiler asks us if we meant to use a different variant, but *provides no solutions*. The space where it normally lists it's suggestion is empty.

The conclusion here is that there exists at least one type that cannot be instantiated in Rust.

### Getting back to it.

Using the types above we can try some different examples:

```rust
fn my_func () -> Unit { ... }
// or (native rust type)
fn my_func () -> () { ... }
// or
fn my_func () { ... }
```

As before, this must be a constructor for the `Unit` type. We know what the return value is going to be before it even returns, so we don't really care about it. If this function is going to do something useful, it's going to perform some IO. If it doesn't perform IO, the only thing it can do is just return `Unit::Unit`.

Heres a trickier example:

```rust
fn my_func () -> Void { ... }
```

This is a constructor for the type `Void`, but we have just seen that `Void` cannot actually be instantiated. A paradox. This function must panic, or loop forever. We know that this function can't return, because if it *could* return, then we could construct a value of type `Void`. We can use `Void` as a return type to tell the next person who reads your code, that this function does not terminate.

In rust, the `Void` return type is often annotated with an exclamation point: `!`.

```rust
fn my_func (a: u32, b: u32) -> u32 { ... }
```

This function is interesting. It could do pretty much *anything*. It could add, subtract, multiply or divide `a` and `b`. It could perform bitwise operations. It could be a bit shift. It could ignore `b` and return `a`, it could ignore `a` and return `b`. It could return `3`. It could perform any combination of the above. When we see a function like this, we're relying on the name of the function and it's documentation to tell us what it does.

Normally, when a function signature is populated by concrete types, it doesn't tell us a lot about what it does. It's usually better to have generic parameters and access them through a trait interface, that way we can tell a bit more about what the function is going to do.

```rust
fn my_func<T> () -> T { ... }
```

This function also panics or loops forever. This is a constructor for a type `T`. There are two ways of reasoning about this:

* There is no single way to construct a value of *every* type, so there is nothing you can put in the body of the function that will return an unknown type `T` for *every* `T` that could possibly exist.
* `T` could be `Void`. There is no way to construct `Void`, so the function must panic or loop forever.

```rust
fn my_func<T> (arg: T) -> T { ... }
```

This function just returns `arg`.

This function takes an argument of type `T` and returns something of type `T`, where `T` can be anything. The only implementation of `my_func` is to just return `arg`. We cannot construct an arbitrary `T`, since `T` could be `Void`. We also can't do anything to to `arg`, or modify it, because it could be *any* type, and the only thing we can do with *every* type is move it. If `T` was `Unit`, we wouldn't be able to do anything with it.


```rust
fn my_func<T> (a: T, b: T) -> T { ... }
```

This is similar to a function we've already seen, but with `T` instead of `u32`. There are only two implementations for this function: we could either return `a` or return `b`. We can't do anything else. This function is different to the one using `u32` arguments: The previous function could do almost anything with it's values, this function can only do one of two things.

```rust
fn my_func<T: Clone> (arg: &T) -> T { ... }
```

It's obvious that there's only one thing this function does. We know that `T` is `Clone` but we know nothing else. The only way to make a `T` from a `&T` must be to clone it, therefore `my_func` must be `clone`. Essentially, we give ourselves "permission" to use the `Clone` interface on `T` inside the function body. If we don't declare it as part of the type signature, we can't use it.

### Intermission 2: Writing correct programs.

There's a common theme here: the more generic we make our functions, the less things that our functions can do. If we only have generic parameters with no restrictions, the only thing we can do with them is move them. If we restrict them to certain traits, then we can only use the interface from the traits that we have specified.

Writing programs using generic types also helps with communication to the next programmer. We can tell if our value is going to be modified, cloned, iterated over, or any other behavior. The person who calls the function has some idea of what's going to go on inside, as we don't have to make as many assumptions. 

When we write programs, we want them to be correct. Some of the functions above only have a single valid implementation. As long as the program compiles, the function is valid. Other functions have only two valid implementations. We have a 50/50 chance of getting the implementation right, even if we have no idea what the function is actually supposed to do. We assume that the programmer has some idea of what they are doing, so the probability of being correct given that the program compiles is a lot higher.

Therefore, here are three reasons to make your functions as generic as possible:

* Generic functions can be maximally reused.
* Generics minimises the number of incorrect programs you can write.
* A programmer who sees the type signature knows a lot about the function you are writing.

Conclusion:

Here are some final examples. See if you can guess possible implementations from only the type signature:

```rust
fn my_func<T: Clone + Ord> (a: &T, b: &T) -> T { ... }
fn my_func<T> (arg1: Option<T>) -> T { ... }
fn my_func<T> (arg1: Result<T, T>) -> T { ... }

fn my_func<T, U> (arg: T) -> Result<T, U> { ... }
fn my_func<T: Debug, U: Debug> (arg1: Result<T, U>) { ... }
fn my_func<T> () -> Option<T> { ... }
```