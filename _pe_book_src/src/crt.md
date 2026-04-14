# Chinese remainder theorem (CRT)

The Chinese remainder theorem states that if we have the solution to an equation modulo \\(m_1\\) and modulo \\(m_2\\), then we can find the solution modulo \\(m_1m_2\\). In other words, if we have two equations of the form:

\\[
\begin{align*}
    x &= a_1 \mod m_1 \\\\
    x &= a_2 \mod m_2 \\\\
\end{align*}
\\]

then we can find \\(x \mod m_1m_2\\).

## When to use this

Often we can find the answer to an equation modulo some small prime, but it is difficult to find the full solution. In this case, we can calculate the answer modulo a series of small primes, and then combine the answers using CRT.

Sometimes a question will directly require the use of CRT to find a solution.

## Calculating a CRT with coprime \\(m_1\\) and \\(m_2\\):

To start, we want to find a pair of numbers \\(n_1\\), \\(n_2\\) such that:

\\[
\begin{align*}
    n_1 &= 1 \mod m_1 \\\\
        &= 0 \mod m_2 \\\\
    n_2 &= 1 \mod m_2 \\\\
        &= 0 \mod m_1
\end{align*}
\\]

Then \\(a_1n_1 + a_2n_2\\) will be a solution. To find \\(n_1\\) we can rearrange:

\\[
\begin{align*}
    n_1 &= 0 \mod m_2 \\\\
        &= m_2k_2 \\\\
        &= 1 \mod m_1 \\\\
        &= 1 - m_1k_1 \\\\
        1 &= m_2k_2 + m_1k_1 \\\\
\end{align*}
\\]


We can use [Bézout's identity](./gcd.md#bézouts-identity) to find a solution to \\(1 = m_2k_2 + m_1k_1\\) as long as \\(m_1\\) and \\(m_2\\) are coprime. In the case that they are not coprime, then we can make them coprime by dividing one of them by the gcd. In this case, solutions only exist if \\(a_1 = a_2 \mod gcd(m_1,m_2)\\)

```rust,editable
// solve ax + by = gcd(a, b). return (x, y, gcd(a, b))
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r = (a, b);
    let mut s = (1, 0);

    while r.1 != 0 {
        let quotient = r.0 / r.1;
        r = (r.1, r.0 - quotient * r.1);
        s = (s.1, s.0 - quotient * s.1);
    }

    (s.0, (r.0 - s.0 * a) / b, r.0)
}

// Returns x such that x % m_1 = a_1 and x % m_2 = a_2. Returns None if there are no solutions
fn crt(a_1: i64, m_1: i64, mut a_2: i64, mut m_2: i64) -> Option<i64> {
    let (mut k_1, mut k_2, gcd) = extended_gcd(m_1, m_2);

    if gcd > 1 {
        // No solution
        if a_1 % gcd != a_2 % gcd {
            return None;
        }

        m_2 = m_2 / gcd;
        a_2 = a_2 % m_2;
        (k_1, k_2, _) = extended_gcd(m_1, m_2);
    }

    Some((m_2 * k_2 * a_1 + m_1 * k_1 * a_2).rem_euclid(m_1 * m_2))
}

fn main() {
    assert_eq!(crt(1, 2, 3, 5), Some(3));
    assert_eq!(crt(1, 10, 0, 2), None);
    assert_eq!(crt(6, 10, 1, 5), Some(6));

    let result = crt(20, 23, 7, 11).unwrap();
    assert_eq!(result % 11, 7);
    assert_eq!(result % 23, 20);
    assert!(result < 11 * 23);
}
```