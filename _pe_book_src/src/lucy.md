# Lucy's algorithm

Lucy's algorithm can help us find sums of multiplicative functions. I will discuss several variants in this article

## Not Quite Lucy Algorithm

Say we have the following problem:

- We have a function \\(f(n)\\)
- \\(f(n)\\) is **completely multiplicative**, i.e, \\(f(1) = 1$ and $f(a)f(b) = f(ab)\\)
- We have an easy way to calculate \\(F(l) = \sum_{n=1}^{l}{f(n)}\\)
- We want to find the sum of \\(f(p)\\) for every prime \\(p\\) less than a limit \\(l\\)

Then we can use Lucy's algorithm to calculate that sum.

We denote \\(lpf(n)\\) as the **l**owest **p**rime **f**actor of \\(n\\). We define the function \\(S(l, p)\\) as the sum of all \\(f(n)\\) less than or equal to \\(l\\) where the lowest prime factor of \\(n\\) is greater than \\(p\\).[^nonStandardS]

We note that \\(S(l, \lfloor \sqrt{l} \rfloor)\\) is the sum of \\(f(n)\\) for all primes between \\(\sqrt{l}\\) and \\(l\\). \\(1\\) is also included in this sum, so we will need to subtract that out near the end.

The set of \\(n\\) with \\(lpf(n)\\) greater than \\(p\\) is set of \\(n\\) with \\(lpf(n)\\) greater than \\(p - 1\\) minus the set of \\(n\\) with \\(lpf(n)\\) exactly equal to \\(p\\). We can write this as follows:

\\[S(l, p) = S(l, p - 1) - f(p) \cdot S(\lfloor l/p \rfloor, p - 1)\\]

This formula relies on \\(f(n)\\) being completely multiplicative. We also note that in order for the lowest prime factor to be exactly \\(p\\), \\(p\\) must be a prime. If \\(p\\) is composite, \\(S(\lfloor l/p \rfloor, p - 1)\\) is zero. This gives us a recursive way to find \\(S(l, p)\\). The base case is \\(S(l, 1) = F(l)\\).

This is just enough information to write code for the algorithm. In this example, \\(f(n) = 1\\), which will allow us to count the number of primes less than or equal to \\(l\\):

```rust,editable
use std::collections::{HashMap, HashSet};

fn capital_s(
    cache: &mut HashMap<(u64, u64), u64>,
    primes: &HashSet<u64>,
    l: u64,
    p: u64)
-> u64 {
    if let Some(result) = cache.get(&(l, p)) {
        return *result;
    }

    if p <= 1 {
        // return sum of f(n) for all n
        l
    } else if p >= l {
        // smallest prime factor is greater than limit, return 0
        1
    } else if !primes.contains(&p) {
        // p is composite, keep going
        capital_s(cache, primes, l, p - 1)
    } else {
        // normal recursive case
        let result = capital_s(cache, primes, l, p - 1) - capital_s(cache, primes, l / p, p - 1);
        cache.insert((l, p), result);
        result
    }
}

fn main() {
    let l = 1000;
    // Find these primes using a sieve
    let mut small_primes = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    let mut cache = HashMap::new();
    let result = capital_s(&mut cache, &small_primes, l, 31) + small_primes.into_iter().count() as u64 - 1;

    println!("There are {} primes less than or equal to {}", result, l);
}
```

## Multiplicative Lucy

\\[\begin{align*}
    S(l, p) &= \sum_{\substack{n \leq l \\\\ lpf(n) > p}}{f(n)} \\\\
        &= \sum_{\substack{n \leq l\\\\lpf(n) > p - 1}}{f(n)} - \sum_{\substack{n \leq l\\\\lpf(n) = p}}{f(n)}\\\\
        &= S(l, p - 1) - \sum_{\substack{pn \leq l \\\\ lpf(n) > p - 1}}{f(pn)} \\\\
        &= S(l, p - 1) - \left(\sum_{\substack{pn \leq l \\\\ lpf(n) > p - 1  \\\\ p \nmid n}}{f(pn)} + \sum_{\substack{pn \leq l \\\\ lpf(n) > p - 1 \\\\ p \mid n}}{f(pn)} \right)\\\\
        &= S(l, p - 1) - \left(f(p)\sum_{\substack{n \leq \lfloor l/p \rfloor \\\\ lpf(n) > p}}{f(n)} + \sum_{\substack{p^2n \leq l \\\\ lpf(n) > p - 1}}{f(p^2n)} \right)\\\\
        &= S(l, p - 1) - \left(f(p)S(\lfloor l / p \rfloor, p) + f(p^2)S(\lfloor l/p^2 \rfloor, p) + ... \right)\\\\
        &= S(l, p - 1) - \sum_{n \geq 1}{f(p^n)S(\lfloor l / p^n \rfloor, p)}\\\\
\end{align*}
\\]

---

We denote \\(lpf(n)\\) as the **l**owest **p**rime **f**actor of \\(n\\). We define the function \\(S(l, p)\\) as the sum of all \\(f(n)\\) less than or equal to \\(l\\) where the lowest prime factor of \\(n\\) is greater than \\(p\\).[^nonStandardS]

\\[S(l, p) = \sum_{\substack{n \leq l \\\\ lpf(n) > p}}{f(n)}\\]

We note that for any composite less than \\(l\\) the lowest prime factor is always smaller than \\(\sqrt{l}\\). Therefore, any number between \\(\sqrt{l}\\) and \\(l\\) that has a lowest prime factor greater than \\(\sqrt{l}\\) must be prime. Another way of writing this is as follows:

\\[S(l, \lfloor \sqrt{l}\rfloor) = \sum_{n\in \mathbb{P},\ \lfloor \sqrt{l}\rfloor<n\leq l} n\\]

So we can sum all primes less than \\(l\\) by calculating \\(S(l, \lfloor \sqrt{l}\rfloor)\\), and then adding the primes below \\(\sqrt{l}\\). We can find these small primes with a simpler method such as the sieve of Eratosthenes.

We want to calculate \\(S(l, p)\\) Recursively. The base case is \\(S(l, 1)\\) which we can calculate easily:

\\[
\begin{align*}
    S(l, 1) &= \sum_{n \leq l}{n} \\\\
        &= \frac{n(n + 1)}{2}
\end{align*}
\\]

\\(S(l, p)\\) has the following recurrence relation[^recurrenceRelation] which is all the information we need to get started:

\\[
\begin{align*}
    S(l, p) &= S(l, p - 1) - p \cdot S(\lfloor l/p \rfloor, p - 1)
\end{align*}
\\]

We also note that when \\(p\\) is composite then \\(S(l, p) = S(l, p - 1)\\), because it is impossible to have a lowest prime factor equal to a composite.

We can write an algorithm as follows:

- Calculate 












[^nonStandardS]: This is not the usual definition of \\(S\\) that you find for lucy's algorithm, but it simplifies the recursive formulas a lot, which is why I use it.
[^recurrenceRelation]: The recurrence relation is proved as follows: \\[\begin{align*}
    S(l, p) &= \sum_{\substack{n \leq l\\\\lpf(n) > p - 1}}{n} - \sum_{\substack{n \leq l\\\\lpf(n) = p}}{n}\\\\
        &= S(l, p - 1) - \sum_{\substack{pn \leq l \\\\ lpf(n) > p - 1}}{pn} \\\\
        &= S(l, p - 1) - p\sum_{\substack{n \leq \lfloor l/p \rfloor \\\\ lpf(n) > p - 1}}{n} \\\\
        &= S(l, p - 1) - p \cdot S(\lfloor l/p \rfloor, p - 1)
\end{align*}
\\]