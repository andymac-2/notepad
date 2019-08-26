---
title: Smart compilers
category: notes
tags:
- Rust
- Haskell
- Strong typing
---

## I don't like to admit it, but the Rust and Haskell compilers are smarter than I am.

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

 So a compiler is a program which is supposed to turn some code written in some programming language to something that a machine can understand. The compiler is bilingual. It exists halfway between the world of machines and the world of humans. A machine itself, it sometimes has trouble expressing it's intent in ways that humans can understand. Make no mistake. Just because it has trouble expressing itself doesn't mean it isn't smart.

 Normally when you interact with a compiler, the compiler does some bookkeeping, and tells you the result. Normally (e.g. C, JavaScript) it's something along the lines like "you forgot a semicolon", "you forgot to close your brace", or "you're trying to store a value in a structure that doesn't exist". Basically, the compiler ticks off a number of checks on a list and then assumes that the program is correct. It's a common thought that the compiler "trusts" that the programmer is doing the right thing. I have a different opinion. I believe that the compiler doesn't know any better. It assumes that the programmer is correct because it has no way of telling otherwise.

 The interaction is strictly one way. The compiler assumes that the program is correct and only says anything if it cannot understand what it's supposed to do. I tell the compiler to do something, and it obliges every single time without question.

 But what if it was different? What if the compiler knew a thing or two about programming itself? What if instead of the programmer telling the program what to do, we treated the compiler as an equal, and took it's opinions into consideration. Programming would look very different. It took me a long time to swallow my pride, but I strongly believe that we are not equals. The compiler is smarter than me.

## Types.

The Rust compiler does the same bookkeeping that every other compiler does. It checks for syntax errors

$$ math formula $$

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
