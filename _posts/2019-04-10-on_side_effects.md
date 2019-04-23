---
title: On Side Effects
tags:
- Monad
- Haskell
- Immutable
- Functional Programming
- Side effects
category: Notes
---

In Haskell, all functions are pure, that is they have no side effects. They take arguments, do something with them and return a result.

![Pure and Impure]({{ '/img/2019-04-10-on_side_effects1.jpg' | relative_url}})

Therefore, in order to perform side effects, the data from inputs such as the keyboard, mouse, microphone, internal clock, or global variables cannot be given from the "side" of the function. They must be given as arguments.

Similarly, any outputs cannot leave the "side" of the function. The only output is the return value. To provide data to our outputs such as the screen, speakers, network, or files, we must somehow combine it with the return value.

![Pure and Impure]({{ '/img/2019-04-10-on_side_effects2.jpg' | relative_url}})

We can only have one return value, so it is necessary to create some kind of data structure which holds both the return value, and a representation of the side effects it has performed. For each different kind of side effect, we will need a new data structure to hold the results.

The conclusion to draw here is that if each side effect has a separate data structure to represent it, then they have separate types. From an outsider's standpoint, this means that a function must declare side effects as part of it's type signature. We know exactly what the function can do without any surprises. Intuition tells us that this has to lead to less bugs, and we don't have to look as far to find the source of side effects.