---
title: Nothing is an anti-pattern
category: notes
tags:
- here
- are
- some
- tags.
---

### Your programming style and paradigms don't really matter

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

### Summary

- Point 1
- Point 2

## Let's start

We're all programmers here, and we can often tell if one piece of code is better or worse than another. But how would we go about proving that some code is equivalent quality to some other code?

If we only consider speed, that's easy. We look at the big O runtime of two algorithms and compare them. For example, *O(2^n)* is worse than *O(n)*. Two algorithms that are *O(n)* are "about the same". When they have the same algorithmic complexity, we start looking for other reasons to choose one over the other.

You might choose the slightly worse algorithm once you have taken into consideration what your colleagues use, or what is the simpler to write. Alternatively you could just flip a coin, because it *doesn't really matter that much*.

Let's say instead of comparing algorithms, we compare the difficulty of a problem. Say we can choose to solve problem A, or problem B. If there is an easy way to convert problem A to problem B, the we can say problem A isn't *that much harder* than problem B. Similarly, if problem B can be converted into problem A without much effort, then problem B isn't *that much harder* than problem A.

So now let's talk about this idea called **isomorphism**. If we can convert between the two "things" easily, then neither "thing" is going to be much harder than the other. We can say they are "about the same", or **isomorphic**. If I was being rigorous, when you convert one "thing" to another and back, you have to get the original "thing" in order to to be isomorphic. However, I don't need to be rigorous, I just need to be "about right".

## Code quality

So how do we know if one method of writing code is better than another? Well, I don't. All I need to do is look at the two methods of writing code and if I could easily convert one style into the other and back, then I can say they're about the same. If it turns out you made the wrong decision, you can always easily change it later.

Here is an easy example: Tabs vs spaces? There are automated tools that change between one and the other, and there are successful programs written using both. I can't prove that one is better than the other, so it lands firmly in the "about the same category". I'll probably end up using whatever the linter defaults are because I have no compelling reason to change it. Similar arguments apply for where brackets should go, placing semicolons in dynamic languages etc.

As an aside, I'm not trying to say that styling doesn't matter at all. Rather, I think that it's more important to have a *consistent* style than it is to have "good" style. Whatever "good" really means. If you ended up writing a codebase that had "bad" style such as (gasp) three space indentation, the code would still get written, and you would have a hard time convincing me that the code has more bugs, or costs more money to write. However if you mixed random levels of indentation, then code might suddenly become very difficult to read.

So in a hand-wavy sense, two pieces of code have a similar "quality" if they can be converted to each other in a straightforwards way. If you are in a situation where you have to choose between code one way or the other, then you should consider reasons other than just code quality. Don't spend too long thinking, because you can always change it later.

## Organising your files



## Getting wacky

I've come across a number of articles recently that say something along the lines of "switch statements are an anti-pattern, you should use dynamic dispatch instead". I suppose this sentiment is fine. I've also come across the rarer but more extreme position "if statements are an anti-pattern" which is probably going a bit overboard.

The general principle is that you can abstract over the `switch` or `if` statement using an abstract factory. The factory is a black box so it is irrelevant if it uses `switch` or `if` internally. Once the class is constructed using the factory, further calls use dynamic dispatch rather than any kind of branching code.

Note that branching code is not isomorphic to dynamic dispatch. Dynamic dispatch allows for a finite set of operations that can be applied to an open (potentially infinite) set of types.



Text body. 

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
