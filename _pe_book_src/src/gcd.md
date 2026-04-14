# GCD and extended GCD algorithms

## Greatest Common Divisor (gcd)

Say we have two integers \\(a\\) and \\(b\\), the gcd is the biggest number that cleanly divides both \\(a\\) and \\(b\\) without a remainder. That is, the gcd is the biggest number \\(m\\) such that \\(a = mx\\) and \\(b = my\\) for some \\(x\\) and \\(y\\). There is a simple algorithm to find this number:

> Subtract the smaller number from the bigger number until both numbers are equal. When they are equal, we have the gcd

For example, if we want to find the gcd of 64 and 26:

| Step | a  | b  |
|------|----|----|
| 1    | 64 | 26 |
| 2    | 38 | 26 |
| 3    | 12 | 26 |
| 4    | 12 | 14 |
| 5    | 12 | 2  |
| 6    | 10 | 2  |
| 7    | 8  | 2  |
| 8    | 6  | 2  |
| 9    | 4  | 2  |
| **10**   | **2**  | **2**  |

So the gcd of 64 and 26 is 2. This is called the Euclidean algorithm. More information, and more efficient algorithms can be found [on Wikipedia](https://en.wikipedia.org/wiki/Euclidean_algorithm)

When \\(gcd(a, b) = 1\\), then \\(a\\) and \\(b\\) are said to be **coprime**. that is, they have no factors in common.

## Optimising the GCD

If one number is much larger the other, then we spend a lot of time repeatedly subtracting it. Repeated subtraction is just division. What we are left with is the remainder. So instead of repeatedly subtracting one number from the other, we can take the remainder of one number from the other and save a lot of time:

```rust,editable
pub fn gcd(mut a: u32, mut b: u32) -> u32 {
    while a != 0 {
        (a, b) = (b % a, a);
    }

    b
}

fn main() {
    println!("gcd({}, {}) = {}", 64, 26, gcd(64, 26));
}
```

## Lowest Common Multiple (lcm)

The lowest common multiple is the smallest number that is a multiple of both \\(a\\) and \\(b\\). The lcm is equal to:

\\[\frac{ab}{gcd(a, b)}\\]

Say we want to find the smallest solution to the equation \\(ax + by = 0\\). Then we have:

\\[
\begin{align*}
    a\frac{b}{gcd(a, b)} - b\frac{a}{gcd(a, b)} &= 0 \\\\
    (x, y) = \left(\frac{b}{gcd(a, b)}, -\frac{a}{gcd(a, b)}\right)
\end{align*}
\\]

# Bézout's Identity

Let \\(a\\) and \\(b\\) be any two integers, then an important middle step for many problems is to find an \\(x\\) and \\(y\\) such that \\(ax + by = gcd(a, b)\\) This is called Bézout's identity. We start with two equations:

\\[
\begin{align*}
    1a + 0b &= a \\\\
    0a + 1b &= b
\end{align*}
\\]

At each step, just like the gcd, we subtract the larger equation from the smaller one until both are the same. Since we start with \\(a\\) and \\(b\\) just like the Euclidean algorithm, we will end up with the gcd at the end. Using 64 and 26 the same example as above:

| Step | equation 1  | equation 2  |
|------|-------------|-------------|
| 1    | \\(1a + 0b = 64\\) | \\(0a + 1b = 26\\) |
| 2    | \\(1a - 1b = 38\\) | \\(0a + 1b = 26\\) |
| 3    | \\(1a - 2b = 12\\) | \\(0a + 1b = 26\\) |
| 4    | \\(1a - 2b = 12\\) | \\(-1a + 3b = 14\\) |
| 5    | \\(1a - 2b = 12\\) | \\(-2a + 5b = 2\\) |
| 6    | \\(3a - 7b = 10\\) | \\(-2a + 5b = 2\\) |
| 7    | \\(5a - 12b = 8\\) | \\(-2a + 5b = 2\\) |
| 8    | \\(7a - 17b = 6\\) | \\(-2a + 5b = 2\\) |
| 9    | \\(9a - 22b = 4\\) | \\(-2a + 5b = 2\\) |
| 10   | \\(11a - 27b = 2\\) | \\(-2a + 5b = 2\\) |

So now we have two solutions to our equation: \\(x = 11, y = -27\\) and \\(x = -2, y = 5\\). This process is called the **extended Euclidean algorithm**. Using the gcd optimisation above, we get the following algorithm:

```rust,editable
// Solve ax + by = gcd(a, b). Returns (x, y, gcd(a, b))
fn extended_gcd(mut a: i32, mut b: i32) -> (i32, i32, i32) {
    let (mut x_1, mut y_1, mut x_2, mut y_2) = (1, 0, 0, 1);

    while a != 0 {
        let quotient = b / a;
        // subtract the first equation from the second and swap
        (x_1, x_2) = (x_2 - x_1 * quotient, x_1);
        (y_1, y_2) = (y_2 - y_1 * quotient, y_1);
        (a, b) = (b % a, a);
    }

    (x_2, y_2, b)
}

fn main() {
    let (x, y, gcd) = extended_gcd(64, 26);
    println!("Solution: {} * 64 + {} * 26 = {}", x, y, gcd);
}
```

We note that in the first column of the table above, the value of \\(x\\) always increases, whilst the value of \\(y\\) always decreases. The opposite is true for the second column. These solutions are somewhat special, the left column contains the smallest positive value of \\(a\\) that satisfies the identity, whilst the right hand column contains the smallest positive value of \\(b\\) that satisfies the identity.

There are infinitely many other solutions that we can make by adding or subtracting zero to the above equation:

\\[
\begin{align*}
    ax + by &= 0 \\\\
    13a - 32b &= 0 \tag*{from "lcm" section above} \\\\
    11a - 27b &= 2 \tag*{from table above} \\\\
    (11a - 27b)  + (13a - 32b) &= 2 \\\\
    24a - 59a &= 2
\end{align*}
\\]