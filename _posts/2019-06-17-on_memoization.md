---
title: On Memoization
category: notes
tags:
- rust
- memoization
- recursion
- performance
- continuations
---

**Sumary:**

Using "open" recursion:

* We can separate memoization logic from function logic.
* We can memoize functions without modifying the syntax of the function.
* We can make functions generic across multiple types of memoization.

### Memoization in Rust.

Consider a recursive function. The most basic example is the Fibonacci sequence. Here is a naive implementation:

```rust
fn fib_naive(arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => fib_naive(n - 1) + fib_naive(n - 2),
    }
}

fn main() {
    assert_eq!(fib_naive(40), 102334155);
}
```

![A memoized function]({{ '/img/2019_06_17_fib_naive.jpg' | relative_url}})

Calculating `fib_naive(40)` on my computer takes almost a second. If we look at this function, we can reason about it's performance. The result of `fib_naive` is going to be `0`, `1` or the result of two calls to `fib_naive` added together. If we only have `0`, `1`, and addition at our disposal, then it will take at least `fib_naive(n)` additions to calculate `fib_naive(n)`. This is unacceptable performance for anything but the smallest values of `n`.

### Memoizing the results

One way to improve performance is to *memoize* the results. That means that when we call our function, we record what the result is in some kind of cache. If we need to call the function again with the same arguments, instead of calculating the result, we look the result up in the cache.

The algorithm is a simple modification to the function above: whenever we call our function, we check if the result is already in the cache, if it's already there, we return it. If it's not there, we calculate it, save the result in the cache, and return the value:

```rust
fn fib_memo (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
    match cache.get(&arg).map(|entry| entry.clone()) {
        Some(result) => result,
        None => {
            let result = match arg {
                0 => 0,
                1 => 1,
                n => fib_memo(cache, n - 1) + fib_memo(cache, n - 2),
            };
            cache.insert(arg, result.clone());
            result
        }
    }
}

fn main () {
    assert_eq!(fib_memo(&mut HashMap::new(), 40), 102334155);
}
```

This is still reasonably straightforward, there's nothing overly complicated, and the performance of this function is a drastic improvement over `fib_naive`. Note that the original function body is still there in the `None` arm of the `match` statement. We also have to call the function using an extra argument: `&mut HashMap::new()`.

We can do better. The memoization logic is not specific to `fib_memo`. We can put the memoization logic in a separate function so we can use it for other functions later:

```rust
fn memoize<A, R, F> (cache: &mut HashMap<A, R>, func: F, arg: A) -> R where
    A: Eq + Hash + Clone,
    R: Clone,
    F: Fn(&mut HashMap<A, R>, A) -> R
{
    match cache.get(&arg).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            let result = func(cache, arg.clone());
            cache.insert(arg, result.clone());
            result
        }
    }
}
```

This function allows us to rewrite our `fib_memo` function:

```rust
fn fib_memo2 (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => memoize(cache, fib_memo2, n - 1) + 
             memoize(cache, fib_memo2, arg - 2),
    }
}

fn main() {
    assert_eq!(memoize(&mut HashMap::new(), fib_memo2, 40), 102334155);
}
```

![A memoized function]({{ '/img/2019_06_17_fib_memo.jpg' | relative_url}})

Now we can use `memoize` for any function without having to write it down all the time. This code still has some issues which we will attempt to solve:

First of all, we could pass the wrong cache to memoize. It could be a cache from another function, or it could have been modified by something else to contain incorrect values. The cache and the function are linked together, and should be a single unit.

Secondly, it's inconvenient to always have to call `fib_memo2` through the `memoize` function. Users of this function may become frustrated that we can't call it directly.

Thirdly, we can't change the type of memoization. If we want to call this function without memoization, we have to rewrite it. We'll talk about how to resolve this later.

### Introducing open recursion

Disclaimer: the following currently only works on Rust nightly.

First of all, we put the cache and the function together in a single struct so that we can't accidentally use the wrong cache with our function. We also create a constructor so that we can make a `HashCache` from a function:

```rust
struct HashCache <A, R> {
    data: HashMap<A, R>,
    func: fn(&mut HashCache<A, R>, A) -> R,
}
impl<A, R> HashCache<A, R> where
    A: Eq + Hash
{
    fn from_func(func: fn(&mut Self, A) -> R) -> Self {
        HashCache {
            data: HashMap::new(),
            func: func,
        }
    }
}
```

The signature of `func` is very similar to the signature of `fib_memo`, but instead of a `HashMap` as a cache, we have our own `HashCache`.

Next, let's implement `FnMut` for `HashCache`:

```rust
impl<A, R> FnMut<(A,)> for HashCache<A, R> where
    A: Eq + Hash + Clone,
    R: Clone,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        let arg = args.0;
        match self.data.get(&arg).map(|x| x.clone()) {
            Some(result) => result,
            None => {
                let result = (self.func.clone())(self, arg.clone());
                self.data.insert(arg, result.clone());
                result
            }
        }
    }
}

// We need to implement `FnOnce` to implement `FnMut`.
impl<A, R> FnOnce<(A,)> for HashCache<A, R> where
    A: Eq + Hash + Clone,
    R: Clone,
{
    type Output = R;
    extern "rust-call" fn call_once(mut self, args: (A,)) -> Self::Output {
        self.call_mut(args)
    }
}
```

Apart from some minor implementation details, the code for `call_mut` is the same as the code for `memoize`. Some key points are:

* We can call a `HashCache` struct like a regular function because it implements `FnMut`.
* When we call `HashCache` like a function, it contains a cache and a function pointer already, so the only extra information we need is the argument to `fib_xxx`.

Next, let's rewrite our function to use open recursion:

```rust
fn fib_open<F>(recurse: &mut F, arg: u32) -> u32 where
    F: FnMut(u32) -> u32
{
    match arg {
        0 => 0,
        1 => 1,
        n => recurse(n - 1) + recurse(n - 2),
    }
}
```

This is similar to what our original `fib_naive` looked like. The difference is that where `fib_open` calls itself by name, we call the `recurse` function.

`fib_open` also has similarities to `fib_memo2`, instead of the cache argument, we have an `&mut F` called `recurse`, and instead of calling `memoize`, we call `recurse`.

`HashCache<u32, u32>` implements `FnMut(u32) -> u32`, so it is a suitable value for `recurse`. We can also call it as a standalone function:

```rust 
fn main() {
    let mut memoised = HashCache::from_func(fib_open);
    assert_eq!(memoised(40), 102334155);
}
```

![A memoized function]({{ '/img/2019_06_17_fib_open.jpg' | relative_url}})

### Swapping out recursive methods:

We discussed not having to rewrite the function if we want to use regular recursion. We can create a newtype wrapper around an open recursive function:

```rust 
struct NoCache<A, R>(fn(&mut NoCache<A, R>, A) -> R);
```

Unlike the `HashCache` struct, since we are not memoizing anything, we don't have to store the cache, just the function pointer. We can implement `FnMut` for `NoCache`:

```rust
impl<A, R> FnMut<(A,)> for NoCache<A, R>
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        (self.0.clone())(self, args.0)
    }
}

impl<A, R> FnOnce<(A,)> for NoCache<A, R>
{
    type Output = R;
    extern "rust-call" fn call_once(mut self, args: (A,)) -> Self::Output {
        self.call_mut(args)
    }
}
```

Similar to `HashCache`, `NoCache` is a suitable value for `recurse`, so we can both feed it to `fib_open` and to call it by itself:

```rust
fn main() {
    let mut regular = NoCache(fib_open);
    assert_eq!(regular(40), 102334155);
}
```

In conclusion we manage to solve all three of our original problems:

* Using `HashCache`, we can't mix up a function and it's respective cache. 
* We can call memoized function using regular function call syntax.
* We can swap out memoization with regular recursion without modifying the function itself.