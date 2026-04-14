# Modular Arithmetic

This is the big one. Modular arithmetic occurs in almost every difficult competitive programming question, since numbers will almost certainly overflow a fixed width integer. Modular arithmetic forms a finite group. This is good for us since a finite group can be represented with a finite amount of memory.

We note the following:

\\[
\begin{align*}
(a \mod m) + (b \mod m) &\equiv a + b \mod m \\\\
(a \mod m) - (b \mod m) &\equiv a - b \mod m \\\\
(a \mod m) \cdot (b \mod m) &\equiv a \cdot b \mod m \\\\
(a \mod m)^b &\equiv a^b \mod m
\end{align*}
\\]

```rust,editable
fn main() {
    let (a, b, m) = (5, 9, 11);

    assert_eq!(((a % m) + (b % m)) % m, (a + b) % m);
    assert_eq!(((a % m) - (b % m)) % m, (a - b) % m);
    assert_eq!(((a % m) * (b % m)) % m, (a * b) % m);
}
```

This allows us to reduce the values of \\(a\\) and \\(b\\) before we do any calculations, and still end up with the same result, allowing us to work with much smaller numbers than we would normally for most calculations.

Exponentiation is a little different. In the case we have coprime \\(a\\) and \\(p\\), then \\(a^{\phi(p)} \equiv 1\mod p\\) which is given by Euler's theorem. Therefore:

\\[
\begin{align*}
a^{b \mod \phi(p)} \equiv a^b \mod p
\end{align*}
\\]

## Division

The first step to calculate \\(\frac{b}{a} = ba^{-1}\\) is to first calculate \\(a^{-1}\\). We know that \\(aa^{-1} \equiv 1 \mod p\\). which can be rewritten as:

\\[
\begin{align*}
aa^{-1} &= 1 \mod p \\\\
aa^{-1} &= py + 1 \\\\
aa^{-1} - py &= 1 \\\\
\end{align*}
\\]

We know how to solve the similar equation \\(ax + by = gcd(a, b)\\) using the [Bézout's identity](./gcd.md#bézouts-identity). If \\(a\\) and \\(p\\) are coprime, then \\(gcd(a, p) = 1\\) and we can find the solution to \\(aa^{-1} - py = 1 = gcd(a, p)\\). If they are not coprime, then there is no inverse, and we cannot perform modular division. 

So the process to find \\(\frac{b}{a} \mod p\\) is as follows:

- Solve for \\(x\\) in \\(ax - py = 1\\) using Bézout's identity. A solution only exists if \\(gcd(a, p) = 1\\)
- \\(x = a^{-1}\\) is the inverse of \\(a\\)
- The solution is \\(ba^-1\\)

### Example

Let's find \\(4/3 \mod 7\\). Firstly, we solve \\(3x + 7y = gcd(3, 7)\\)

| Step | equation 1  | equation 2  |
|------|-------------|-------------|
| 1    | \\(1\cdot3 + 0\cdot7 = 3\\) | \\(0\cdot3 + 1\cdot7 = 7\\)  |
| 2    | \\(1\cdot3 + 0\cdot7 = 3\\) | \\(-1\cdot3 + 1\cdot7 = 4\\)  |
| 3    | \\(1\cdot3 + 0\cdot7 = 3\\) | \\(-2\cdot3 + 1\cdot7 = 1\\)  |
| 4    | \\(3\cdot3 - 1\cdot7 = 2\\) | \\(-2\cdot3 + 1\cdot7 = 1\\)  |
| 5    | \\(5\cdot3 - 2\cdot7 = 1\\) | \\(-2\cdot3 + 1\cdot7 = 1\\)  |

Which gives us \\(3x + 7y = 3\cdot5 - 7\cdot2 = 1 = gcd(3, 7)\\). The gcd is \\(1\\) so we have a solution \\((x, y) = (5, 2)\\). \\(x\\) is the inverse of \\(a\\). The final step is to multiply by \\(b\\)

\\[
\begin{align*}
4/3 &= 4\cdot3^{-1} \mod 7 \\\\
&= 4\cdot5 \mod 7 \\\\
&= 6 \mod 7\\\\
\end{align*}
\\]

To verify this is correct we can multiply by \\(3\\)

\\[
\begin{align*}
4/3 &= 6 \mod 7 \\\\
4 &= 6\cdot3 \mod 7 \\\\
&= 18 \mod 7 \\\\
&= 4 \mod 7 \\\\
\end{align*}
\\]

## Square roots