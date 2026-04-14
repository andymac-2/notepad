# Dynamic Programming

Dynamic programming is a method of solving problems that can be expressed recursively. Note that, we can always rewrite a non recursive function recursively. Basically, we save the result of intermediate computations. This is called memoization. Consider the Fibbonacci sequence:

\\[
\begin{align*}
    F_0 &= 0 \\\\
    F_1 &= 1 \\\\
    F_n &= F_{n - 1} + F_{n - 2}
\end{align*}
\\]

And a naive implementation:

```rust,editable
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2)
    }
}

assert_eq!(fibonacci(5), 5);
assert_eq!(fibonacci(10), 55);
```

This takes \\(O(F_n)\\) time to calculate, since every addition we perform adds \\(1\\) to the result. This is because in the process of calculating our final results, we perform many duplicate calls.

Instead of calculating these intermediate values many times over, we can store them in a cache and retrieve them when necessary. This allows us to calculate much larger versions of \\(F_n\\)

```rust,editable
use std::collections::HashMap;

fn fibonacci(cache: &mut HashMap<u64, u64>, n: u64) -> u64 {
    // We have already calculated this before, skip the calculation and return
    // the value
    if let Some(result) = cache.get(&n) {
        return *result;
    }

    // Perform the actual calculation
    let result = match n {
        0 => 0,
        1 => 1,
        n => fibonacci(cache, n - 1) + fibonacci(cache, n - 2)
    };

    // Store the result of the calculation for later
    cache.insert(n, result);
    return result;
}

let mut cache = HashMap::new();
assert_eq!(fibonacci(&mut cache, 60), 1548008755920);
assert_eq!(fibonacci(&mut cache, 78), 8944394323791464);
```