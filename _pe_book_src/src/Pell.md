# Pell's Equation

We can solve basically every quadratic diophantine equation of two variables by reducing it to the Pell equation

## Solve Pell's equation

Pell's equation has the form \\(x^2 - ny^2 = 1\\) where \\(n\\) is a positive integer that is not a square[^nonsquare]. It has a trivial solution \\(x = \pm1, y = 0\\), but our task is to find non trivial ones. Rearranging gives the following:

\\[
\begin{align*}
    \sqrt{n + \frac{1}{y^2}} &= \frac{x}{y}
\end{align*}  
\\]

Looking at this equation, we can see that as \\(y\\) increases, \\(1/y^2\\) decreases, and \\(x/y\\) becomes a better and better approximation for \\(\sqrt{n}\\).

Probably the easiest way to solve this equation is to use the [Stern Brocot](SternBrocot.md) tree[^continuedFractions], which iterates through all reduced fractions in a range. Any solution \\(x/y\\) must be between \\(\sqrt{n}\\) and \\(\sqrt{n + \frac{1}{y^2}}\\) inclusive. We simply ignore branches outside this bound. As we progress down the tree \\(y\\) increases so the upper bound becomes closer and closer to \\(\sqrt{n}\\). We will find every solution because the Stern-Brocot tree iterates through every reduced fraction.

```rust,editable
use std::cmp::Ordering;

fn find_pell_solutions(n: i64, left: (i64, i64), right: (i64, i64)) {
    let mediant = (left.0 + right.0, left.1 + right.1);

    // terminate if the denominator is greater than 100
    if mediant.1 > 1000 {
        return;
    }

    if mediant.0.pow(2) == 1 + mediant.1.pow(2) * n {
        println!("Found solution! {}^2 - {}*{}^2 = 1", mediant.0, n, mediant.1);
    }

    // if x/y < sqrt(n), then the left branch of the tree is completely out of
    // bounds
    if mediant.0.pow(2) > mediant.1.pow(2) * n {
        find_pell_solutions(n, left, mediant);
    }

    // if x/y > sqrt(n + 1/y^2), then the right branch of the tree is
    // completely out of bounds
    if mediant.0.pow(2) < mediant.1.pow(2) * n + 1 {
        find_pell_solutions(n, mediant, right);
    }
}

fn main() {
    // Find solutions for n = 2
    find_pell_solutions(2, (0, 1), (1, 0));
}
```

## A non recursive solution

Recursive functions can be refactored to use a loop if you manage the stack yourself:

```rust,editable
fn find_pell_solutions(n: i64) {
    let mut stack: Vec<((i64, i64), (i64, i64))> = vec![((0, 1), (1, 0))];

    while let Some((left, right)) = stack.pop() {
        let mediant = (left.0 + right.0, left.1 + right.1);

        if mediant.1 > 1000 {
            continue;
        }

        if mediant.0.pow(2) == 1 + mediant.1.pow(2) * n {
            println!("Found solution! {}^2 - {}*{}^2 = 1", mediant.0, n, mediant.1);
        }

        if mediant.0.pow(2) > mediant.1.pow(2) * n {
            stack.push((left, mediant));
        }

        if mediant.0.pow(2) < mediant.1.pow(2) * n + 1 {
            stack.push((mediant, right));
        }
    }
}

fn main() {
    find_pell_solutions(2);
}
```

## Negative pell equation

The negative pell equation has the form \\(x^2 - ny^2 = -1\\) we rearrange to get

\\[
\begin{align*}
    \sqrt{n - \frac{1}{y^2}} &= \frac{x}{y}
\end{align*}  
\\]

\\(x/y\\) is between \\(\sqrt{n - \frac{1}{y^2}}\\) and \\(\sqrt{n}\\), and we can use the Stern-Brocot tree again:

```rust,editable
fn find_pell_solutions(n: i64, left: (i64, i64), right: (i64, i64)) {
    let mediant = (left.0 + right.0, left.1 + right.1);

    if mediant.1 > 1000 {
        return;
    }

    if mediant.0.pow(2) + 1 == mediant.1.pow(2) * n {
        println!("Found solution! {}^2 - {}*{}^2 = 1", mediant.0, n, mediant.1);
    }

    // if x/y < sqrt(n - 1/y^2), then the left branch of the tree is completely
    // out of bounds
    if mediant.0.pow(2) + 1 > mediant.1.pow(2) * n {
        find_pell_solutions(n, left, mediant);
    }

    // if x/y > sqrt(n), then the right branch of the tree is completely out of
    // bounds
    if mediant.0.pow(2) < mediant.1.pow(2) * n {
        find_pell_solutions(n, mediant, right);
    }
}

fn main() {
    find_pell_solutions(2, (0, 1), (1, 0));
}
```

## Generalised pell equation

The generalised pell equation has the form \\(x^2 - ny^2 = a\\) we rearrange to get

\\[
\begin{align*}
    \sqrt{n - \frac{a}{y^2}} &= \frac{x}{y}
\end{align*}  
\\]

\\(x/y\\) is between \\(\sqrt{n - \frac{a}{y^2}}\\) and \\(\sqrt{n}\\), and we can solve it like the others.

## An aside: solving \\(x^2 - n^2y^2 = a\\)

What happens when \\(n\\) is a square in the generalised Pell equation? Then we have \\(x^2 - (ny)^2 = a\\), which is a difference of squares. Factorising gives:

\\[
    a = (x - ny)(x + ny)
\\]

We can solve this equation by factorising \\(a = pq\\). Suppose \\(p = x + ny\\) and \\(q = x - ny\\), then we get the following:

\\[
    x = \frac{p + q}{2}, y = \frac{p - q}{2n}
\\]

A simple algorithm is to factorise \\(a\\) to find candidates for \\(x\\) and \\(y\\). Then filter out all solutions where \\(x\\) and \\(y\\) are fractions.

## Solve \\(ax^2 + bx + c = dy^2\\)

We apply the quadratic equation directly which gives:

\\[
    x = \frac{-b \pm \sqrt{b^2 - 4a(c - dy^2)}}{2a}
\\]

We note that \\(x\\) is only rational if the part under the square root is a square. Expanding gives:

\\[
\begin{align*}
    z^2 &= b^2 - 4a(c - dy^2) \\\\
    z^2 -4ady^2 &= b^2 - 4ac \\\\
    z^2 -Ny^2 &= A
\end{align*}
\\]

Now we can solve it using any technique for the generalised Pell equation. If \\(N\\) is a square, we can [solve using factorisation instead](#an-aside-solving-x2---n2y2--a) Once we have solutions, we can substitute back to get \\(x\\), filtering out any solutions where \\(x\\) is a fraction.

## Generalised quadratic diophantine equations

Say we have a diophantine equation for which we want to find integer solutions for \\(x, y\\):

\\[ax^2 + bxy + cy^2 + dx + ey + f = 0\\]

We apply the quadratic equation directly to solve for \\(y\\):

\\[
    y = \frac{-(bx + e) \pm \sqrt{(bx + e)^2 - 4c(ax^2 + dx + f)}}{2c}
\\]

We note that \\(y\\) is only rational if the part under the square root is a square. Expanding gives:

\\[
\begin{align*}
    z^2 &= (bx + e)^2 - 4c(ax^2 + dx + f) \\\\
        &= (b^2 + 4ac)x^2 + (2be -4cd)x + (e^2 - 4cf) \\\\
        &= Ax^2 + Bx + C
\end{align*}
\\]

And now we can solve it using [the technique above](#solve-ax2--bx--c--dy2). Once we have candidate solutions, filter out any solutions where \\(y\\) is a fraction.

[^nonsquare]: If \\(n\\) is a square, then we have \\(x^2 - m^2y^2 = 1\\). This is a difference of squares. The only solutions are the trivial solutions \\(x = \pm1, y = 0\\).
[^continuedFractions]: Most literature online says that we can find a solution using continued fractions, however finding good guidance or working code online is surprisingly hard. [*Solving the generalized Pell equation* by John P. Robertson](https://web.archive.org/web/20150216094231/http://www.jpr2718.org/pell.pdf) is a good resource that is terse, but comprehensive.