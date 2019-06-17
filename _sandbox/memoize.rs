#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::collections::HashMap;
use std::time::Instant;
use std::hash::Hash;

//NAIVE IMPLEMENTATION

fn fib_naive(arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => fib_naive(n - 1) + fib_naive(n - 2),
    }
}

// MONOLITHINC MEMOIZZTION

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

// ORIGINAL IMPLEMENTATION:

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

fn fib_memo2 (cache: &mut HashMap<u32, u32>, arg: u32) -> u32 {
    match arg {
        0 => 0,
        1 => 1,
        n => memoize(cache, fib_memo2, n - 1) + memoize(cache, fib_memo2, arg - 2),
    }
}

// FnMut IMPLEMENTATION:

struct NoCache<A, R>(fn(&mut NoCache<A, R>, A) -> R);

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

// Invariants:
// - HashCashe's data corresponds to the memoized return values of the function
//   func.
// - The function func has no side effects (cannot be verified at compile time)
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

impl<A, R> FnOnce<(A,)> for HashCache<A, R> where
    A: Eq + Hash + Clone,
    R: Clone,
{
    type Output = R;
    extern "rust-call" fn call_once(mut self, args: (A,)) -> Self::Output {
        self.call_mut(args)
    }
}

fn fib_open<F>(recurse: &mut F, arg: u32) -> u32 where
    F: FnMut(u32) -> u32
{
    match arg {
        0 => 0,
        1 => 1,
        n => recurse(n - 1) + recurse(n - 2),
    }
}


fn main () {
    let now = Instant::now();
    assert_eq!(fib_memo(&mut HashMap::new(), 40), 102334155);
    println!("time: {}", now.elapsed().as_millis());

    assert_eq!(memoize(&mut HashMap::new(), fib_memo2, 40), 102334155);
    println!("time: {}", now.elapsed().as_millis());

    let mut memoised = HashCache::from_func(fib_open);
    assert_eq!(memoised(40), 102334155);
    println!("time: {}", now.elapsed().as_millis());

    let mut open = NoCache(fib_open);
    assert_eq!(open(40), 102334155);
    println!("time: {}", now.elapsed().as_millis());

    assert_eq!(fib_naive(40), 102334155);
    println!("time: {}", now.elapsed().as_millis());
}