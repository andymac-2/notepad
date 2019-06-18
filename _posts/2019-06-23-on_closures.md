---
title: Understanding closures, fn, Fn, FnMut and FnOnce.
category: notes
tags:
- rust
- closures
- call
- function
- context
- functional programming.
---

### Summary

* Closures are a combination of a function pointer (`fn`) and a context.
* A closure with no context is just a function pointer.
* A closure which has an immutable context belongs to `Fn`.
* A closure which has a mutable context belongs to `FnMut`.
* A closure that owns it's context belongs to `FnOnce`.

## Understanding the different types of closures in Rust.

![The different kinds of function traits]({{ '/img/2019-06-23-fn_types.jpg' | relative_url}})

Unlike some other languages, Rust is explicit about our use of the `self` parameter. We have to specify `self` to be the first parameter of a function signature when we are implementing a struct:

```rust
struct MyStruct {
    text: &'static str,
    number: u32,
}

impl MyStruct {
    fn new (text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text: text,
            number: number,
        }
    }

    // We have to specify that 'self' is an argument.
    fn get_number (&self) -> u32 {
        self.number
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_number (&mut self) {
        self.number += 1;
    }
    // There are three different types of 'self'
    fn destructor (self) {
        println!("Destructing {}", self.text);
    }
}
```

As a result, the following two styles are identical:

```rust
obj.get_number();
MyStruct::get_number(&obj);
```

This is in contrast to other languages where `self` (or `this`) is often implied. Simply associating a function with an object or structure in these languages can imply that the first argument is `self`. Demonstrated above, we have four options for `self`: an immutable reference, a mutable reference, an owned value, or to not use `self` as an argument at all.

As a result, `self` implies some sort of context for the execution of the function. it is explicit in Rust, but often implicit elsewhere.

Also in this post we will use the following functions:

```rust
fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {}
fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}
```

The only purpose of these functions is to typecheck. For example, if `is_FnMut(&func)` compiles, then we know that `func` belongs to the `FnMut` trait.

## No Context and the `fn` (lowercase f) type

With this in mind, consider some examples of closures using `MyStruct` above:

```rust
let obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

let closure1 = |x: &MyStruct| x.get_number() + 3;
assert_eq!(closure1(&obj1), 18);
assert_eq!(closure1(&obj2), 13);
```

This is about as simple as we can get. This closure adds three to the number of any object of type `MyStruct` it has been given. It can be executed anywhere without any issues, and the compiler will not give you any trouble. We can quite easily write `closure1` like this instead:

```rust
// It doesn't matter what code appears here, the function will behave
// exactly the same.

fn func1 (x: &MyStruct) -> u32 {
    x.get_number() + 3
}
assert_eq!(func1(&obj1), 18);
assert_eq!(func1(&obj2), 13);
```

This function does not depend on it's context. It will behave exactly the same no matter what happens before or after it. We can use `func1` and `closure1` (almost) interchangeably.

When a closure does not depend on context at all, the type of our closure is `fn`:

```rust
// compiles successfully.
is_fn(closure1); 
is_Fn(&closure1);
is_FnMut(&closure1);
is_FnOnce(&closure1);
```

## Immutable context and the `Fn` (Capital F) trait

Compared to the above, we can add a context to a closure.

```rust
let obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

// obj1 is borrowed by the closure immutably.
let closure2 = |x: &MyStruct| x.get_number() + obj1.get_number();
assert_eq!(closure2(&obj2), 25);

// We can borrow obj1 again immutably...
assert_eq!(obj1.get_number(), 15);

// But we can't borrow it mutably.
// obj1.inc_number();               // ERROR
```

`closure2` depends on the value of `obj1` and contains information about the surrounding scope. In this case, `closure2` will borrow `obj1` so that it can use it in the function body. We can still borrow `obj1` immutably, but if we were attempt to mutate `obj1` afterwards, we would get a borrowing error.

If we try to rewrite our closure using `fn` syntax, everything we need to know inside of the function must be passed to it as an argument, so we add an additional argument to represent the context of the function:

```rust
struct Context<'a>(&'a MyStruct);

let obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

let ctx = Context(&obj1);

fn func2 (context: &Context, x: &MyStruct) -> u32 {
    x.get_number() + context.0.get_number()
}
```

Which behaves almost identically to our closure:

```rust
assert_eq!(func2(&ctx, &obj2), 25);

// We can borrow obj1 again immutably...
assert_eq!(obj1.get_number(), 15);

// But we can't borrow it mutably.
// obj1.inc_number(); // ERROR
```

Note that the `Context` struct contains an immutable reference to `MyStruct` indicating that we won't be able to modify it inside the function.

When we call `closure1` it is *implied* that we pass the surrounding context as an argument to the closure, like we had to do it with our `fn`. Like in some other languages where we don't have to specify that we pass `self` as an argument, Rust doesn't need us to explicitly specify that we pass our context as an argument.

When a closure takes a context as an immutable reference, we say that it implements the `Fn` trait. That tells us that we can call our function multiple times without modifying the context:

```rust
// Does not compile:
// is_fn(closure2);

// Compiles successfully:
is_Fn(&closure2);
is_FnMut(&closure2);
is_FnOnce(&closure2);
```

## Mutable context and the `FnMut` trait

If we modify `obj1` inside the closure, we get different results:

```rust
let mut obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

// obj1 is borrowed by the closure mutably.
let mut closure3 = |x: &MyStruct| {
    obj1.inc_number();
    x.get_number() + obj1.get_number()
};
assert_eq!(closure3(&obj2), 26);
assert_eq!(closure3(&obj2), 27);
assert_eq!(closure3(&obj2), 28);

// We can't borrow obj1 mutably or immutably
// assert_eq!(obj1.get_number(), 18);   // ERROR
// obj1.inc_number();                   // ERROR
```

This time we can't borrow `obj1` mutably or immutably. We also have to annotate the closure as `mut`. If we wish to rewrite this function using `fn` syntax, we get the following:

```rust
struct Context<'a>(&'a mut MyStruct);

let mut obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

let mut ctx = Context(&mut obj1);

// obj1 is borrowed by the closure mutably.
fn func3 (context: &mut Context, x: &MyStruct) -> u32 {
    context.0.inc_number();
    x.get_number() + context.0.get_number()
};
```

This behaves the same way as `closure3`:

```rust
assert_eq!(func3(&mut ctx, &obj2), 26);
assert_eq!(func3(&mut ctx, &obj2), 27);
assert_eq!(func3(&mut ctx, &obj2), 28);

// We can't borrow obj1 mutably or immutably
// assert_eq!(obj1.get_number(), 18);       // ERROR
// obj1.inc_number();                       // ERROR
```

Note that we have to pass our context with a mutable reference. This indicates that we may get different results every time we call our function.

When a closure takes it's context using a mutable reference, we say that it belongs to the `FnMut` trait:

```rust
// Does not compile:
// is_fn(closure3);
// is_Fn(&closure3);

// Compiles successfully:
is_FnMut(&closure3);
is_FnOnce(&closure3);
```


## Owned Context: 

For our last example we'll take ownership of `obj1`:

```rust
let obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

// obj1 is owned by the closure
let closure4 = |x: &MyStruct| {
    obj1.destructor();
    x.get_number()
};
```

We have to check the type of `closure4` *before* we use it:

```rust
// Does not compile:
// is_fn(closure4);
// is_Fn(&closure4);
// is_FnMut(&closure4);

// Compiles successfully:
is_FnOnce(&closure4);
```

Now we can check the behavior of it:

```rust
assert_eq!(closure4(&obj2), 10);

// We can't call closure4 twice...
// assert_eq!(closure4(&obj2), 10);             //ERRORz

// We can't borrow obj1 mutably or immutably
// assert_eq!(obj1.get_number(), 15);           // ERROR
// obj1.inc_number();                           // ERROR
```

In this example, we can only call the function once. Once we have called it the first time, we have destroyed `obj1`, so it no longer exists for the second call. Rust gives us an error about using a value after it has been moved. That's why we have to check the types beforehand.

Writing this with an `fn` we get the following:

```rust
struct Context(MyStruct);

let obj1 = MyStruct::new("Hello", 15);
let obj2 = MyStruct::new("More Text", 10);

let ctx = Context(obj1);

// obj1 is owned by the closure
fn func4 (context: Context, x: &MyStruct) -> u32 {
    context.0.destructor();
    x.get_number()
};
```

Which, as expected, behaves the same as our closure:

```rust
assert_eq!(func4(ctx, &obj2), 10);

// We can't call func4 twice...
// assert_eq!(func4(ctx, &obj2), 10);             //ERRORz

// We can't borrow obj1 mutably or immutably
// assert_eq!(obj1.get_number(), 15);           // ERROR
// obj1.inc_number();                           // ERROR
```

When we write our closure using `fn` we have to use a `Context` struct that owns it's value. When a closure takes ownership of it's context, we say that it implements `FnOnce`. We can only call the function once, because after that, the context has been destroyed.

## Conclusion

* Functions that require no context have the `fn` type, and can be called anywhere.
* Functions that only need immutable access to their context belong to the `Fn` trait, and can be called anywhere as long as the context is still in scope.
* Functions that need mutable access to their context implement the `FnMut` trait, which can be called anywhere the context is valid, but may do something different each time.
* Functions that take ownership of their context can only be called once.

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})