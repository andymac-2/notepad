## Mobius inversion

Say we have the following problem:

- We have two related functions \\(f\\) and \\(g\\) such that
    \\[
    \begin{align*}
        g(1) &= f(1) + f(2) + f(3) ... \\\\
        g(2) &= f(2) + f(4) + f(6) ... \\\\
        g(3) &= f(3) + f(6) + f(9) ... \\\\
        \vdots \\\\
        g(n) &= \sum_{m = 1}^{\infty}f(mn)
    \end{align*}  
    \\]
- We want to calculate \\(f(1)\\)
- We have a good way to find \\(g(n)\\) but no easy way to find \\(f(n)\\)

Then we can use a technique called Mobius inversion.

We start with a simpler problem. Let's say we want to calculate the sum of all \\(f(n)\\) when \\(n\\) is divisible by \\(2\\) or \\(3\\). We can calculate this using \\(g(2) + g(3) - g(6)\\). We subtract \\(g(6)\\) because we counted the multiples of \\(6\\) twice.

Say we add another prime and we want to find the sum of all \\(f(n)\\) when \\(n\\) is divisible by \\(2, 3\\) or \\(5\\). Then we can calculate this with \\(g(2) + g(3) + g(5) - g(6) - g(10) - g(15) + g(30)\\). We subtract \\(g(6), g(10)\\) and \\(g(15)\\) because we counted them twice. We add \\(g(30)\\) because we add it three times for \\(g(2), g(3)\\) and \\(g(5)\\) and then subtract it three times for \\(g(6), g(10)\\) and \\(g(15)\\). This leaves us counting it zero times, so we have to add it back.

Adding more primes we can see a pattern emerge:

- Add all \\(g(n)\\) when \\(n\\) is divisible by any prime
- Subtract all \\(g(n)\\) when \\(n\\) is divisible by at least two primes
- Add all \\(g(n)\\) when \\(n\\) is divisible by at least three primes
- Subtract all \\(g(n)\\) when \\(n\\) is divisible by at least four primes
- And so on

To simplify the process we can introduce the Mobius function \\(\mu(n)\\) which allows us to know when to add or subtract a value of \\(g(n)\\). It has the following definition:

\\[
\mu(n) = 
    \begin{cases}
    -1 & \text{if n is the product of an odd number of distinct primes} \\\\
    1 & \text{if n is the product of an even number of distinct primes} \\\\
    0 & \text{otherwise} \\\\
    \end{cases}
\\]

Knowing this, we can calculate \\(f(1)\\) using \\(g(1)\\) and then subtracting all the multiples of \\(3, 5, 7, 11 ...\\) until we are left with just \\(f(1)\\). We assume that \\(f(n) = 0\\) when \\(n\\) is larger than some big number. The final expression for \\(f(1)\\) is as follows:

\\[\boxed{f(1) = \mu(1)g(1) + \mu(2)g(2) + \mu(3)g(3) ... = \sum_{m = 1}^{\infty}\mu(m)g(m)}\\]


## Variations

We can usually define some \\(g'(x)\\) and \\(f'(x)\\) to solve a range of related problems. Here are some common examples, but other substitutions also work.

### Calculate \\(f(a)\\)

Say you want to calculate \\(f(a)\\) instead of \\(f(1)\\). Then define \\(f'(m) = f(ma)\\) and \\(g'(m) = g(ma)\\) then we have \\(g'(n) = \sum_{m = 1}^{\infty}f'(mn)\\) which is our original problem. Solving and substituting back gives the general solution:

\\[f(n) = \sum_{m = 1}^{\infty}\mu(m)g(mn)\\]

### Calculate \\(f(N)\\) where \\(g(n) = \sum_{m = 1}^{\infty}f\left(\left\lfloor\frac{N}{mn}\right\rfloor\right)\\)

We define \\(f'(x) = f\left(\left\lfloor\frac{N}{x}\right\rfloor\right)\\). Then we have \\(g(n) = \sum_{m = 1}^{\infty}f'(mn)\\) which is our original problem. Solving and substituting back gives the solution:

\\[f(N) = \sum_{m = 1}^{\infty}\mu(m)g(m)\\]

This also works similarly for other scenarios, such as \\(g(n) = \sum_{m = 1}^{\infty}f\left(\frac{N}{mn}\right)\\)

### Calculate \\(f(N)\\) where \\(g(n) = \sum_{m = 1}^{\infty}f\left(\left\lfloor\frac{n}{m}\right\rfloor\right)\\)?

We define \\(f'(x) = f\left(\left\lfloor\frac{N}{x}\right\rfloor\right)\\), and \\(g'(x) = g\left(\left\lfloor\frac{N}{x}\right\rfloor\right)\\) Then we have the following:

\\[
\begin{align*}
    g'(n) &= \sum_{m = 1}^{\infty}f\left(\left\lfloor\frac{\left\lfloor\frac{N}{n}\right\rfloor}{m}\right\rfloor\right) \\\\
    &= \sum_{m = 1}^{\infty}f\left(\left\lfloor\frac{N}{mn}\right\rfloor\right) \\\\
    &= \sum_{m = 1}^{\infty}f'(mn)
\end{align*}  
\\]

Which is our original problem. Solving and substituting gives:

\\[
\begin{align*}
    f'(1) &= \sum_{m = 1}^{\infty}\mu(m)g'(m) \\\\
    f(N) &= \sum_{m = 1}^{\infty}\mu(m)g\left(\left\lfloor\frac{N}{m}\right\rfloor\right)
\end{align*}  
\\]

Which is a more "traditional" looking Mobius inversion.

### What if \\(g(n)\\) doesn't sum to infinity?

Say you have \\(g(n) = \sum_{m = 1, mn \leq k}f(mn)\\) instead of \\(g(n) = \sum_{m = 1}^{\infty}f(mn)\\). Then define \\(f'(x)\\) as follows:

\\[
f'(n) = 
    \begin{cases}
    f(x) & x \leq k \\\\
    0 & \text{otherwise} \\\\
    \end{cases}
\\]

Then you have \\(g(n) = \sum_{m = 1}^{\infty}f'(mn)\\) which is our original problem. We also have \\(g(n) = 0\\) when \\(n > k\\) by definition. Therefore the solution is given by:

\\[f(1) = \sum_{m = 1}^{k}\mu(m)g(m)\\]