---
title: On Borrowing
tags: 
- Borrow Checker 
- Rust 
- Mutable 
- Immutable 
- Stack 
- Heap
- Memory Management
---

### Understanding borrowing.

Let's assume for a moment that Rust is a language which can only use immutable variables. Instead of changing a variable, we would have to create a new one each time.

```rust
let x = 5;
println!("{}", x);  // prints 5
let y = 4;
println!("{}", y);  // prints 4
let z = 3;
println!("{}", z);  // prints 3
```

We can skirt this problem by "shadowing" variables in Rust. We create a new variable that has the same name as the old one. The old variable still exists, but we can no longer access it by name:

```rust
let x = 5;
println!("{}", x);  // prints 5
let x = 4;
println!("{}", x);  // prints 4
let x = 3;
println!("{}", x);  // prints 3
```

A sufficiently smart compiler could overwrite the memory containing older versions of `x` with something else since they are no longer being used. However the compiler is often not this smart. It would be helpful to have an annotation that tells the compiler that the old version of the variable is no longer in use, so we can reclaim the memory for some other purpose. It would be an error to use this annotation if there is still a way to access the variable.

Let's create our own annotation called `until_shadow` which tells the compiler that a variable is usable until it's name is "shadowed" by another variable. With our hypothetical annotation, we would write our program as follows:

```rust
let until_shadow x = 5;
println!("{}", x);

// "x" has been shadowed, so we can reclaim the old memory for something else
let until_shadow x = 4;
println!("{}", x); 

// "x" has been shadowed again, so we can reclaim the second value
let until_shadow x = 3;
println!("{}", x);
```

Note that if we are smart about this, we can put each new version of `x` into the memory where the old `x` used to be.

It would be an error to have a reference to an old value if we are going to overwrite it's memory:

```rust
let until_shadow x = 5;
let y = &x;  // we take a reference to x here
println!("{}", x);

let until_shadow x = 4;     // ERROR!!!
// y still references the old value, so it is an error to reclaim x's memory.
println!("{}", x);

let until_shadow x = 3;
println!("{}", x);
```

To support references, we could create an additional notation to the compiler to say that a given reference is the *only* way to access a variable. After creating such a reference it would be an error to access the variable directly, or to create another reference. Let's call this hypothetical notation `unique_ref`:

```rust
let until_shadow x = 5;
let unique_ref y = &x;
println!("{}", y);

// We have told the compiler that y is the only way to access x, so
// we should not access it directly.
// println!("{}", x);       // ERROR!

let until_shadow x = 4;
// At this point we can reclaim the old value. We have shadowed the old
// reference, so the old reference is no longer accessable. The old reference
// was the only way to access the old value so we should be able to use the old
// value's memory for something else.
let unique_ref y = &x;
println!("{}", y); 

let until_shadow x = 3;
let unique_ref y = &x;
println!("{}", y);
```

If we have unique references, we can free the memory that a variable uses once the reference goes out of scope or is shadowed.

The rust notation for `unique_ref` and `until_shadow` is called `mut`:

```rust
let mut x = 5;
let mut y = &x;
println!("{}", y);

// We have told the compiler that y is the only way to access x, so
// we should not access it directly:
// println!("{}", x);       // ERROR!

//  We create a new reference and a new value at the same time. The new
//  reference shadows the old reference and the new value shadows the old value.
//  The old reference was the only way to access the old value, so we can safely
//  overwrite the old value with the new value. We overwrite the old reference
//  with the new reference that points to the same location as the old value.
*y = 4;
println!("{}", y); 

*y = 3;
println!("{}", y);
```

Which almost completely explains the behavior of the borrow checker. In summary:

- Consider all variables in rust to be immutable, and all references to be immutable references.
- We do not mutate variables, we just create new ones.
- If a variable has a valid reference to itself, we cannot safely overwrite the variable's memory with something else. (Explains immutable borrows)
- A `mut` annotation on a reference indicates that it is the only way to access a variable. (Explains why you cannot borrow mutably and immutably at the same time, explains why you can only have a single mutable borrow)
- A `mut` annotation on a variable indicates that the memory may be overwritten if there is a unique reference to it and that unique reference is shadowed, or if there are no references to it, and the variable itself is shadowed. (Explains mutable borrows)