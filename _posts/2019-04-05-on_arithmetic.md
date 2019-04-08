---
title: On Arithmetic
---
{% include math.html %}
### It seems obvious that 2 + 2 ≠ 5...

![Or is it?]({{'/img/on_arithmetic.jpg' | relative_url }})

A rigorous proof that 2 + 2 ≠ 5 is not particularly trivial. In order to be
concise, we need to begin with some definitions:

$$ 0 \in \mathbb{N} \label{axiom0} \\
\forall x \in \mathbb{N}, S(x) \in \mathbb{N} \\
\forall x, y \in \mathbb{N}, x = y \Leftrightarrow S(x) = S(y) \\
\forall x \in \mathbb{N}, \neg (S(x) = 0) \\
\forall x \in \mathbb{N}, x + 0 = x \\
\forall x, y \in \mathbb{N}, x + S(y) = S(x + y) $$

Which is seems more complicated than it is. 

- "∀" means "for all". 
- "∈" means "element of" "belongs to" or "in". 
- "ℕ" is a symbol for the natural numbers (0, 1, 2, 3 ... etc.). 
- "¬" means "not". 
- "⇔" means "if and only if". 

The only other part here that hasn't been described is "S()", the successor
function. The successor of a number is the number after itself, for example,
*S(0) = 1*, and *S(3) = 4*.

The axioms above in English for reference are as follows:

1. *0* is a natural number
2. For every natural number *x*, the number after *x* is also a natural number.
3. For any two natural numbers *x* and *y*, if *x = y, then S(x) = S(y)* and if *S(x) = S(y)* then *x = y*
4. *0* does not come after any natural number.
5. Any natural number plus zero is itself
6. For any two natural numbers *x* and *y*, *x + S(y) = S(x + y)*

This is sufficient to prove *2 + 2 ≠ 5*. For brevity we have left out some
axioms of equality.

We define 2 as *S(S(0))* and we define 5 as *S(S(S(S(S(0)))))*. To prove
that *2 + 2 ≠ 5*, we need to prove *¬(2 + 2 = 5)*. *¬(2 + 2 = 5)* is
equivalent to *2 + 2 = 5 ⇒ ⊥* where "⇒" means "implies" and "⊥" means
"falsehood" or "contradiction". So if we can prove that *2 + 2 = 5* implies
a contradiction or falsehood, then we have our proof.

We start with our definition of 2 and 5:

$$ S(S(0)) + S(S(0)) = S(S(S(S(S(0))))) $$

Then we apply our axioms of addition:

$$ S(S(0)) + S(S(0)) = S(S(S(S(S(0))))) \Rightarrow \\
S(S(S(0)) + S(0)) = S(S(S(S(S(0))))) \Rightarrow \\
S(S(S(S(0)) + 0)) = S(S(S(S(S(0))))) \Rightarrow \\
S(S(S(S(0)))) = S(S(S(S(S(0))))) $$

We use axiom 3 to reduce our problem by removing one layer of successor
functions each time.

$$
S(S(S(S(0)))) = S(S(S(S(S(0))))) \Rightarrow \\
S(S(S(0))) = S(S(S(S(0)))) \Rightarrow \\
S(S(0)) = S(S(S(0))) \Rightarrow \\
S(0) = S(S(0)) \Rightarrow \\
0 = S(0)
$$

For which we already know by axiom 4 that *¬(S(0) = 0)*, or as we have seen
before, *S(0) = 0 ⇒ ⊥*.

$$
0 = S(0) \Rightarrow \perp
$$

Therefore:

$$
2 + 2 = 5 \Rightarrow \perp
$$

or:

$$
2 + 2 \neq 5
$$