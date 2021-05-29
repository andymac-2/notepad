---
title: On Whatever the title is
category: notes
tags:
- here
- are
- some
- tags.
---

### A brief description

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

### Summary

- Point 1
- Point 2

## First heading

When we want to write some program, we want it to work. And what this means is that anywhere you need use a particular piece of code, it will do what it's supposed to do.

In Object Orientated design, the distinction between open and closed reuse is sometimes blurred. In a lot of cases, they are used interchangeably.

In the scenario where we use a closed set, that means that we can count the number of different possibilities your code will encounter. For example, say you have a boolean value. It can only take one of two possible values: true or false. If we handle both cases, then we have covered all possibilities.

Alternatively we could think of an open set. Say you have some interface type. There are potentially infinite possibilities for what the underlying type could be. The only thing that we can do with such a type is to use the interface provided. By using an interface, we declare that we *don't care* what the type is, as long as it implements the interface.

Saying "infinite" here is a bit facetious. There are never going to be infinite implementations for an interface. Rather that, in the case of an interface, it is impossible to exhaustively check every possibility. Even if you checked all existing implementations one by one, someone else could always add another implementation later.

### Example 1: A list interface



In the case of an open set, you need to consider all implementations past present and future. 

Specifically, this is only really relevant if we talk about the code being reusable.

This could be reworded as saying it should be impossible to break your program through a public API. 

Text body. 

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
