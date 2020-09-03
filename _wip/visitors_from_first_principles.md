---
title: On Whatever the title is
category: notes
tags:
- here
- are
- some
- tags.
---

Include this line if you want to have math formulas on the page.
{% include math.html %}

### Summary

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

To understand what 

Note that I use typescript in this article. While the article is not specifically about Typescript, I chose it because most programmers are familiar with JavaScript. Those that are familiar with Java, C#, or C++ may find the syntax easy to understand, even if they have no prior experience with it.

## Isomorphism

Isomorphism is a quality of "sameness" of two things. In programming, there are lots of these qualities, but the most common is the idea of equality. Two things are equal if they have the same value. Isomorphism is a similar concept, but Isomorphism is a applied to *types* rather than the values contained within them.

One type is isomorphic to another type if you can convert between them without losing any information. More specifically, if `A` and `B` are isomorphic, then you can convert a value from `A` to `B` and back, and the result should be what you started with (and vice versa going from `B` to `A`).

This does not mean that two types are the same, but it does mean that they have a quality of "sameness" to them. Any operation you can do to a value of type `A`, you can do to a value of type `B` by converting it first and vice versa. This kind of means that the two types are interchangeable, wherever you can use one, it would be equally valid to use the other.

A simple example of this would be a function that takes a boolean as an argument and returns some value:

```typescript
function MyFunc(arg: boolean): MyType {
    // ... do something
}
```

Intuitively, this could be written as two functions, each that takes no argument. One for a `false` argument and one for a `true` argument:

```typescript
function trueFunc(): MyType {
    //... do something
}

function falseFunc(): MyType {
    //... do something
}
```

We could write the two separate functions in terms of the first function:

```typescript
function trueFunc(): MyType {
    return myFunc(true);
}

function falseFunc(): MyType {
    return myFunc(false);
}
```

Similarly, we can just as easily write the combined function is terms of the separate ones:

```typescript
function myFunc(arg: boolean): MyType {
    if (arg) {
        return trueFunc();
    } else {
        return falseFunc();
    }
}
```

If these were real types, we can write a conversion functions between the two:

```typescript
interface OneFunc {
    (arg: boolean): MyType;
}

interface TwoFuncs {
    trueFunc: () => MyType;
    falseFunc: () => MyType;
}

function convertToTwo (func: OneFunc): TwoFuncs {
    return {
        trueFunc: () => func(true),
        falseFunc: () => func(false)
    }
}

function convertToOne (two: TwoFuncs): OneFunc {
    return (arg) => arg ? two.trueFunc() : two.falseFunc();
}
```

It does not take much convincing that `convertToOne(convertToTwo(object))` (or `convertTotwo(convertToOne(object))`) is going to give you something that behaves the same way as `object` does. Ignoring performance, we can say that converting from one to the other and back is a no-op. That is, we can say `OneFunc` and `TwoFuncs` are *isomorphic*

## Open and closed sets

A set is a collection of things. In our case, we will only consider collections of types. In our case, we will say that a closed set is a set that cannot be added to after it is created. An open set can have any number of elements, and have any number of elements added to it after it is created.

- visitors fundamentally operate on a closed set
- A visitor is 
- dynamic dispatch is used to operate on open sets

## Zero sized variables 

Regardless of their isomorphism, `OneFunc` and `TwoFunc` are not the same for a number of reasons. For example, you might only need to use `myFunc(true)` in your program, but never `myFunc(false)`. In which case, `trueFunc` may be more efficient. `myFunc` would have code internally that handles the `false` case, even though it is never used; this code is not necessary in `trueFunc`. In addition, `myFunc` has to perform an unnecessary runtime check of the argument; we already know that it will be `true` before the program even runs.

Alternatively, we know that `myFunc` may be a better choice when we could expect either `true` or `false`, but we don't know which until the program is running. Wether or not one function or two functions will be better depends on the application.

We know that one function versus two functions may have some performance or usage implications intuitively, but we can formalize this idea with the concept of a zero sized type. At certain points in our program, we have enough information just from looking at the location of our code to determine exactly what the value of a given variable is. The most obvious example of this is an `if` statement on a `boolean` value. We can use an example from before:

```typescript
function myFunc(arg: boolean): MyType {
    if (arg) {
        // arg is true between these curly brackets. There
        // is no need to pass it as an argument
        return trueFunc();
    } else {
        // arg is false between these curly brackets. There
        // is no need to pass it as an argument
        return falseFunc();
    }
}
```

If the program reaches `trueFunc()` it implies that `arg` was true, therefore `arg` does not need to be passed as an argument. As long as code is being run within the scope of the `if` statement, there is no need to ever refer to `arg` directly (ignoring mutation). Because there is no need to refer to `arg`, there is no need to allocate memory for it for code that is run between the curly brackets. Effectively `arg` has become zero sized.

What does it mean to be zero sized? Does it mean that `arg` completely disappears from the program? Not really. It just means that for a certain segment of the program, `arg` and `true` are interchangeable. The value `true` may implicitly appear in the machine code for `trueFunc()`, but storing a reference to `arg` is unnecessary, and therefore does not need to be present in working memory.

An any point in a given program, we should be able to list all of the implicit information we have about the environment. Something like "If I reach line 355 of my program, then x must be greater than 5, and y is even". Intuitively we do this all the time. Note that if we were to record these assumptions down on a piece of paper, it would require a non-zero amount of memory; the quote above takes 78 bytes to write in English. However, because this information is implicit when executing a program, it does not require any specific memory while to program is actually running.

This basically means that all of the information that isn't explicitly stated, but can be inferred purely from the location in code is zero sized.

## The Program Counter

So in any given program, there is an implicit (or in some cases explicit) global variable called the program counter which, for the purposes of this article, specifies the line number of the code that is currently running. Each time the execution of the program moves to a new line, the program counter is incremented. The program counter must therefore be an integer with at least enough precision to store the number of lines of code in your program.

Note that because the program counter is just a proxy for the number of lines written, if the maximum value for the program counter increases, it would mean that new code has been written, and vice versa: if the maximum value of the program counter decreases, then code has been removed.

Theoretically, it would be possible to write a function that takes the program counter as an argument, and returns a list of all the information you could infer from that line of code. For example, using the example above, our theoretical function `inferences(355)` might return `"x is greater than 5, and y is even"`. There's not much point writing this function, but knowing that it *could* be written is enough.

In the case of the "zero sized" `arg` above, `arg` didn't really disappear, but it did become part of the implicit execution context (i.e. `inferences` for the line with `trueFunc` on it might return `"arg is true"`). In a kind of roundabout way, the information stored by `arg` has not disappeared, but has become part of the information stored by the program counter.

The opposite is true as well. Take a value that is zero sized and we can turn it into a sized variable. Remember that the functions above were isomorphic, so we can simply reverse the transformation without losing any information.

This implies some kind of universal "program complexity invariant", a fundamental minimum combination of memory usage and program size for any given program. Assuming that the bodies of each arm of an `if` statement do not occupy the same line of code, this places a lower bound on the size of code that you can write. If the number of unique inferences (return values from the `inferences` function) is `n`, the number of lines of code must also be at least `n`. If we make our code more generic, then the code size can be smaller, but may require more memory.

Essentially different isomorphisms will have different memory/program size tradeoffs.

## Dynamic dispatch

Sometimes we would like to minimize the size of machine code at the cost of some runtime memory. One thing that a compiler can infer at compile time is the set of operations that can be performed on a given type (it's interface). What we can do is create an object that represents the interface, and provide the data itself as a pair of values.

C programmers may be familiar with passing a function pointer with a void pointer for context:

$$ math formula $$

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
