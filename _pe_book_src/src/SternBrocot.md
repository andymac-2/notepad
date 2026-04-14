# Stern-Brocot Tree

The Stern-Brocot tree allows us to iterate over all fractions in a given range. It has a few main properties we care about:

- All fractions in the tree are fully reduced
- It will eventually iterate through all fractions
- Fractions with smaller denominators are higher in the tree
- Fractions in the tree are ordered from smallest to largest

We start at \\(\left(\frac{0}{1}\frac{1}{0}\right)\\). For any given node \\(\left(\frac{a}{c}\frac{b}{d}\right)\\), the left child is given by \\(\left(\frac{a}{c}\frac{a + b}{c + d}\right)\\) and the right child is given by \\(\left(\frac{a + b}{c + d}\frac{b}{d}\right)\\)

This can be implemented with relatively few lines of code:

```rust,editable
fn stern_brocot(left: (u64, u64), right: (u64, u64)) {
    let mediant = (left.0 + right.0, left.1 + right.1);

    // terminate if the denominator is greater than 3
    if mediant.1 > 3 {
        return;
    }

    // terminate if the fraction increases past 2
    if mediant.0 > mediant.1 * 2 {
        return;
    }

    stern_brocot(left, mediant);
    println!("Visited fraction: {}/{}", mediant.0, mediant.1);
    stern_brocot(mediant, right);
}

fn main() {
    stern_brocot((0, 1), (1, 0));
}
```

## Best approximation of anything

The Stern-Brocot tree can be used to find the best approximation of anything with a fraction. Say we want to find the best approximation for \\(\sqrt{13}\\) where the denominator is less than \\(1000\\). As we go deeper into the tree, the denominators increase, and the upper and lower bounds become tighter. Once the denominator exceeds 1000, then we stop, since going down the tree will only make the denominator larger than it already is. The tree is ordered left to right, so we only need to check the branch that contains the number we want to approximate.

```rust,editable
fn distance((num, den): (u32, u32)) -> f64 {
    (num as f64 / den as f64 - 13.0_f64.sqrt()).abs()
}

fn best_approximation() -> (u32, u32) {
    let mut left = (0, 1);
    let mut right = (1, 0);

    loop {
        let mediant = (left.0 + right.0, left.1 + right.1);

        // Stop when the denominator is > 1000
        if mediant.1 > 1000 {
            if distance(left) < distance(right) {
                return left;
            } else {
                return right;
            }
        }

        // If sqrt(13) is less than the mediant, go left, otherwise go right
        if (mediant.0 * mediant.0) / (mediant.1 * mediant.1) >= 13 {
            right = mediant;
        } else {
            left = mediant;
        }
    }
}

fn main() {
    let fraction = best_approximation();
    println!("Best approximation for sqrt(13) is {}/{}", fraction.0, fraction.1);
}
```