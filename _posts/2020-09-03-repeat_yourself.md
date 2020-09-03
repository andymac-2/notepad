---
title: DRY from first principles
category: notes
tags:
- programming
- Don't Repeat Yourself
- DRY
- Typescript
- Rust
- Haskell
- Refactoring
---

### Summary

- "Don't repeat yourself" (DRY) is a widely accepted programming principle, but it has several limitations
- An alternative derivation of DRY is presented that aims to alleviate some of these limitations
- Programs are made up of functions, and to make functions maximally reusable they should
    - Have the most lenient preconditions
    - Have the most strict postconditions
    - Perform the smallest amount of non-trivial work

If you think this article is too long, you can skip to **[Should I refactor my code?](#Should-I-refactor-my-code?)** below

# Don't Don't Repeat yourself

In software engineering, "clean code" is a sought after ideal. Code that is "clean" is easy to read, understand and maintain. Unfortunately, what clean code looks like is often disputed. Some commonly accepted software engineering best practices have lost sight of their original goals, or do not represent a complete picture. I believe Don't Repeat Yourself (DRY) may be one of these principles.

DRY is a widespread and generally well accepted best practice in software engineering. The main stated advantage to DRY is that code does not need to be modified in multiple locations. If code has been de-duplicated, then bugfixes and new features only need to be added once. In addition, code only needs to be tested once rather than multiple times. The end result is that code is **less error prone** and **faster to write**.

Unfortunately, it is also well known that the overzealous application of DRY can lead to poor abstractions, and code that is difficult to modify. If DRY were to be taken to the extreme, then it's aim is just to increase the entropy of written code and turn it into code golf. In some instances this is a good thing, for example: obfuscation, or minified JavaScript, but in most cases, code written in this style is obviously detrimental. This indicates that DRY has some limitations as a principle.

This article will explore some of the limitations of DRY and provide an alternative perspective that does not have the same limitations.

## Limitations of DRY

One of the oft discussed limitations of DRY is that it can lead to poor abstractions. Heuristic solutions like the "rule of three" exist to alleviate this problem, where only code that is duplicated three or more times is worthwhile refactoring. This is not at all perfect. For example, the fourth replication may require an additional parameterization that the first three applications did not.

Often ignored in this discussion, plain, non generic functions are the basic unit of code reuse, and relatively often these functions only have a single call site. Following a strict "rule of three" application, it would follow that a function with a single call site should instead be written inline, and only factored out once it has been written three times. Therefore there appear to exist some worthwhile abstractions, even if they do not reduce the repetition of code:

```typescript
// Large functions that have comments in them are
// considered good candidates for refactoring
function bigFunc() {
    // Do part A
    console.log("Did part A");
    // Do Part B
    console.log("Did part B");
    // Do Part C
    console.log("Did part C");
}

// It is recommended to split such a function into
// smaller parts. Cited reasoning includes:
//  - Easier testing
//  - Self documenting code
//  - Reusability

function doPartA() {
    console.log("Did part A");
}

function doPartB() {
    console.log("Did part B");
}

function doPartC() {
    console.log("Did part C");
}

function newBigFunc() {
    doPartA();
    doPartB();
    doPartC();
}
```

Another argument against applying too much DRY is that overly generic code can be more difficult to understand and maintain. I'm a bit skeptical of how significant this really is, as 'Difficulty' is a very subjective concept and would vary from person to person. but I suppose there may be some truth to the claim that this negatively affects software development in some measurable capacity.

In addition, I dispute the assertion that using DRY is necessarily faster to write. Copy and paste is quick, and I suspect that the vast majority of developer time is not spent actually typing in the first instance. In the case of modification, a global find and replace works for actually writing the code (and for copying over any relevant tests).

<table><tr>
    <td width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></td>
    <td> Obviously copy and paste, and global find and replace are not the recommended tools for refactoring a program, but the concept here is that writing the code itself is not the bottleneck.</td>
</tr></table>

To prove me wrong (or right) in this regard, someone would have to measure the time it takes to write out duplicated code versus non duplicated code, then show that there was a significant difference between them whilst proving that the difference wasn't just due to chance. I doubt that such a study will be done for some time.

## From First Principles

Considering how widespread the principle of DRY is, there is little empirical evidence to say to what extent it improves code quality. The promotion of DRY is largely based on the subjective experiences of software engineers over the course of their work. As such it is unknown exactly how effective it really is.

Certainly, DRY is not an end goal. End goals include things such as reducing time to market, reducing cost, or increasing developer productivity. The quality of code can also play a part; it could be measured by the frequency and severity of bugs that affect users.

Since the empirical evidence is not of sufficient quality, the next best thing would be to start with some small, but fundamental assumptions about how code works, and then *derive* what good code should look like from those assumptions. Note that I will not discuss how writing code one way will fix errors that writing code in another way exposes, nor will I discuss lessons that I have learned writing software. The conclusions drawn in this article will be drawn *without* the need to have experienced real world code.

I will assume that **a developer never deliberately introduces bugs into their programs**, and would choose to write a program with no bugs if at all possible. This is actually quite an ambitious assumption. Some bugs are just not worth fixing under the scrutiny of a cost-benefit analysis; bugs that are extremely mild in nature, or are rare enough that they are never experienced by an end user might not be worth going through the trouble to fix. However, I will persist with this assumption for the purposes of this article.

In addition, I will assume that **the harder it is to prove a program correct, the more likely it contains errors**. I define a program as 'hard' to prove correct if that program requires a lot of symbols in some theorem proving language to ensure correctness. Conversely, a program that requires few symbols is therefore 'easy' to prove correct. I note then that the number of symbols required for a proof is proportional to the number of errors you would find along the way.

<table><tr>
    <td width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></td>
    <td> There's an implicit assumption that during the proving process, bugs can be fixed without adding or removing symbols, so in this way it makes sense to talk about an incorrect program whilst also talking about proving it correct.</td>
</tr></table>

In the biggest logical leap of this article, I will mention something something Curry-Howard, and state without much proof at all that the number of symbols required to prove a program correct is proportional to the number of symbols in the code itself. This is a huge assumption, and probably not at all correct in a lot of instances. However, it does give us a convenient proxy on the number of errors that are in a given piece of code: if the code is longer, or is more complex, then it has more errors.

<table><tr>
    <td width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></td>
    <td> Note that in a functional language with dependent types, where the Curry-Howard isomorphism can actually apply, the proof of a function's correctness and it's implementation is literally the same code. So saying that the length of the proof is the length of the code is indeed a reasonable assumption to make.</td>
</tr></table>

So at least one way to reduce the number of errors in code will be to reduce the number of symbols in the code itself. This sounds a lot like DRY. Note that code that is *terse* does not fit the bill for having less errors, as it still contains the same number of symbols. In this way, naming, styling, and indentation are ignored.

## My recommendation

Programs regardless of paradigm are made out of functions. Well, maybe not in some cases, but they must be made of *something*. And in order for a discussion about code reuse to be meaningful there must be *something* in a programming language that can be reused. I'll use "function" to mean the smallest reusable part of a program

Perhaps to the chagrin of an Object Orientated programmer, I will specifically not talk about objects being reusable. Objects can have multiple methods or members, so if an object is reusable, then that means that there is something *smaller* that is reusable too.

So if we consider a function to be the smallest reusable part of a program, then in order for our program to have the maximum reuse, any given function should

- be **able** to be reused as much as possible, and
- it should be as **useful** as possible, and
- it should be **used** as much as possible.

In order to achieve these goals respectively:

- A function should have the smallest possible set of preconditions
- A function should have the largest set of postconditions
- A function should perform the smallest non trivial amount of work

## The smallest possible set of preconditions

A maximally reusable function should be able to be called wherever it is applicable. Functions can only be called when the caller can fulfill the preconditions of the function. If the caller cannot fulfill the preconditions of a function, but calls the function anyway, then this is an error.

I'll provide a short and not at all rigorous proof that maximally reusable functions have minimal preconditions using contradiction. Assume that there exists a maximally reusable function `f` which specifies a precondition that is unnecessary. That means that there exist potentially valid calling contexts where the function cannot be called, because the precondition cannot be satisfied. `f` can be made more reusable by removing the unnecessary precondition. Because `f` is already maximally reusable, this is a contradiction.

In fact, this must be the *only* thing that determines if a function is able to be used in as many places as possible. The only other places that `f` could potentially be used cannot satisfy necessary preconditions. Since it is already known that the function `f` has a minimal preconditions, any call to `f` in these locations is invalid.

In other words a function with minimal preconditions can be used anywhere it would be valid to call such a function, and the only places it cannot be called are all invalid anyway.

### Example 1: Generics

Say you have a function that takes the maximum of two integers. I use Haskell and Rust, because in a lot of cases, you can specify the preconditions as part of the function type signature.

```haskell
-- Haskell
maxInt :: Int -> Int -> Int
maxInt a b = if a > b then a else b
```

```rust
// Rust
fn max_int (a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
```

In this example, the precondition that the numbers must be integers is unnecessary. The only preconditions that are truly required are that both arguments have the same type, and that they can be compared to each other. We can specify that both arguments have the same type with polymorphism, and specify that they can be compared using a type constraint:

```haskell
maxAny :: (Ord a) => a -> a -> a
maxAny a b = if a > b then a else b
```

```rust
fn max_any<T: Ord> (a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

The second examples are more reusable. This should be fairly obvious considering the second example is generic, where the first example is not. Not to say that I advocate making absolutely everything as generic as possible, only that more generic functions are more reusable. More specifically, the reason that generic functions are more reusable is because they specify a smaller set of preconditions.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Use generics to make code more reusable.</th>
</tr></table>

Although the above reasoning is agnostic regarding the programming paradigm used, those who are familiar with Object Orientated design idioms may find this familiar. Reducing the preconditions by making a function more generic corresponds to the 'I' and 'D' of SOLID. The 'Interface Segregation' principle recommends directly reducing the interface size, whilst the 'Dependency Inversion' principle recommends using interfaces in the first instance.

Those that are familiar with functional programming may see that the second example has been universally quantified. If the type signature of the above example can be though of as a predicate in first order logic, then a universally quantified predicate is a stronger statement than an unquantified one.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Use best practice for your preferred programming paradigm</th>
</tr></table>

### Example 2: Unnecessary input

Although making functions more generic will make them more reusable, it does not cover every use case. Most notably passing too much data into a function limits it's reuse. I use TypeScript in this example as it is usually easy to understand:

```typescript
// Typescript
class Person {
    private name: string;
    private phoneNumber: string;
    private age: number;

    constructor(name: string, phoneNumber: string, age: number) {
        this.name = name;
        this.phoneNumber = phoneNumber;
        this.age = age;
    }

    textMessage(message: string) {
        console.log(`Message sent to ${this.phoneNumber}`);
        console.log(`The message was: "${message}"`);
    }
}

const person = new Person("John Doe", "0491 570 110", 35);
person.textMessage("Save on a new car with out promotional deal!");
```

In this case, the `textMessage` method takes a `Person` object as `this`, and a string representing the message. The `textMessage` function only uses the `phoneNumber` property of `Person`. Requiring a `Person` is an unnecessary precondition. The `textMessage` function can be made more reusable by removing that precondition:

```typescript
function textMessage(phoneNumber: string, message: string) {
    console.log(`Message sent to ${phoneNumber}`);
    console.log(`The message was: "${message}"`);
}
textMessage("0491 570 110", "Save on a new car with out promotional deal!")
```

It should be possible to send a text message to any valid phone number, not just phone numbers that have been assigned to a particular person. It should also be possible to send a text message if the name or age of the person is unknown.


<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th>Recommendation: pass the minimum required data into a function.</th>
</tr></table>

The larger the class is, the more unnecessary data will be passed to any given method. For example: if a class is responsible for two separate items, then methods which regard only the first item or only the second will be passed unnecessary state as part of `this`.

Note that this ties back in with the 'S' of SOLID. The 'Single Responsibility' principle recommends breaking classes with many responsibilities into smaller classes.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Keep classes and data structures small.</th>
</tr></table>

### Other Examples

- Functions that require more arguments than are necessary.
- Non `static` methods that can be made `static`.
- Making a function `async` when it doesn't need to be.
- Using mutable references when immutable references would suffice.

## The largest set of postconditions

In order for a function to be maximally reusable, it must be useful in as many places as possible. Each potential call site may place requirements on the result of the function, so it would make sense that the more strict the postconditions, the more places the function can satisfy the requirements of the call sites.

### Example 1: Return the parent class

In Object Orientated programming, a parent class can be used wherever a child class can be used, but not the other way around. To make the function as reusable as possible, return the parent class:

<table><tr>
    <td width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></td>
    <td> For functional programs, returning a base class would be the equivalent of returning an existentially quantified data type.</td>
</tr></table>

```typescript
// Typescript
class Child {
    childFunc() {
        console.log("I'm a Child");
    }
}

class Parent extends Child {
    parentFunc() {
        console.log("I'm a Parent");
    }
}

function factoryChild (): Child {
    return new Parent();
}

function factoryParent (): Parent {
    return new Parent();
}

const p1: Parent = factoryParent();
// Error: Child does not implement parentFunc
// const p2: Parent = factoryChild();

const c1: Child = factoryParent();
const c2: Child = factoryChild();
```

Note that it is often recommended elsewhere to use abstract factories which return interfaces, or virtual base classes instead of concretions. Returning the parent like I recommend in this case is therefore discouraged. If this makes you uncomfortable, you can always write a wrapper function to turn a concretion into an abstraction. This can't be done the other way around:

```typescript
// Convert a concretion to an abstraction
function factoryChild2 (): Child {
    return factoryParent();
}

// Cannot be done in reverse
// function factoryParent2 (): Parent {
//     return factoryChild();
// }
```

Alternatively if the factory function could return multiple concretions, then the return type has to be an abstraction, so the above code doesn't even apply:

```typescript
class Parent2 extends Child {
    parent2Func() {
        console.log("I'm the second Parent");
    }
}

// Can't return a Parent type here. Returning
// a Child is the most minimal postcondition.
function abstractFactoryChild(): Child {
    if (Math.random() > 0.5) {
        return new Parent();
    } else {
        return new Parent2();
    }
}
```

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Return the most specific type possible</th>
</tr></table>

### Example 2: Validating arguments

In the general case, throwing exceptions is a normal part of programming. Some things are just not in our control, so we just do the best we can, handle it and clean up the mess. If a function validates arguments and returns early on failure, then this indicates that the preconditions are too lenient:

```typescript
// Typescript
function mayHaveBadArgument(argument: number | null): string {
    if (argument === null) {
        throw "Bad argument";
    }
    
    return argument.toString();
}
```

Perhaps this is too obvious an example and would not appear in the real world, I suspect something like the following is written more commonly:

```typescript
function mayHaveBadArgument(argument: number | null): string | null {
    if (argument === null) {
        return null;
    }
    
    return argument.toString();
}
```

In this case, `null` is an invalid value, so why accept it as an argument at all? Maybe it would be better to just take a number and return a string, and make it up to the caller to check for the precondition.

```typescript
function printNumber(argument: number): string {
    return argument.toString();
}
```

Note that this is a bit controversial for two reasons: Firstly it gives stricter preconditions and stricter postconditions, when we would prefer stricter postconditions and more lenient preconditions. Secondly, argument checking code might end up being duplicated at a lot of call sites.

Both of these problems can be fixed by creating a second function that only validates the arguments. Note here that in this toy example it doesn't really provide any benefits, but the benefit may become more obvious in larger examples.

```typescript
function printNumberChecked(argument: number | null): string | null {
    if (argument === null) {
        return null;
    }
    
    return printNumber(argument);
}
```

On one hand, the callers of `printNumber` do not have to check the return value for `null`. On the other hand, callers using `printNumberChecked` do not have to check `argument` for `null`. If `argument` is known not to be `null`, then use `printNumber`, otherwise, use `printNumberChecked`.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Don't write a function that compromises lenient preconditions for strict postconditions or vice versa. Write two functions instead.</th>
</tr></table>

This is the first recommendation that states that something is specifically a *bad* abstraction. Not every abstraction will be a good one, so there should be rules for things that specifically shouldn't be done.

#### Advanced usage for functional programs

Note that for most languages `printNumberChecked` may not be that good an abstraction. It's not reusable enough and does some fairly trivial checking. I wouldn't stress if this appears in your code a lot. A functional programmer may notice that `T | null` is a functor, and `printNumberChecked` is a poorly written version of `map` or `fmap` specialized for `printNumber`:

```haskell
-- Haskell
printNumber :: Int -> String
printNumber = show

printNumberChecked :: (Functor f) => f Int -> f String
printNumberChecked = fmap printNumber
```

```typescript
// Typescript
function mapOptional<T, R>(func: (arg: T) => R): (arg: T | null) => R | null {
    return (arg: T | null) => {
        if (arg === null) {
            return null;
        }

        return func(arg);
    }
}

const printNumberChecked = mapOptional(printNumber);
console.assert(printNumberChecked(null) === null);
console.assert(printNumberChecked(5) === '5');
```

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Don't use a function with side effects when a pure function will do.</th>
</tr></table>

Note that this is generally not idiomatic code except for functional languages. I wouldn't use this unless you and all of you co-contributors are on the same page.

## Perform the smallest amount of non-trivial work

In order for a program to actually realize the benefits of reusable code, the program must actually reuse it's components. Whilst the above principles will help determine what abstractions are good to take, this principle determines how they should be applied.

If a function provides no benefit over writing the code inline, then I would consider it a trivial amount of work. That means trivial functions do not improve the ergonomics, prevent errors or improve any other measurable (or subjective) element of code.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Don't write trivial abstractions.</th>
</tr></table>

A function does not perform the smallest possible non-trivial work if it can be broken down into at least two smaller non-trivial pieces. For the sake of argument, say that a non-trivial function called `f` could be refactored into two smaller non-trivial functions called `a` and `b` that could be recombined to form `f`. `f` must either contain duplicates of `a` and `b` inside it's body, or it must be defined in terms of `a` and `b`.

Assume `f` is defined in terms of `a` and `b`. `a` and `b` would have smaller preconditions than `f`, so they would be able to be used in more places. If `f` is defined in terms of `a` and `b`, it either contains the the smallest amount of non trivial work to combine `a` and `b`, or it can be broken down further, in which case we do so and re-examine the components.

Either way, a function either contains duplicate code, contains preconditions that are too strict, or it performs the smallest amount of non-trivial work. In other words, large functions can be broken down into pieces to make them more reusable.

### Examples

I won't talk much more about this topic as there appears to be a broad consensus that smaller composable functions are better than larger ones for a number of reasons, and I'm not sure I can add much more than what is already available. I'll link to [NASA's ten rules](http://web.eecs.umich.edu/~imarkov/10rules.pdf) for safety critical software, which states that functions written in C should not have more than a single page's worth of code in them.

<table><tr>
    <th width="64"><img src="{{ '/img/info_icon.png' | relative_url}}" alt="information" width="32" /></th>
    <th> Recommendation: Break large functions into smaller ones.</th>
</tr></table>

## Should I refactor my code?

- I have two similar functions, should I refactor them into one?
    - If you can combine the two functions without weakening the postconditions, or strengthening the preconditions: then go ahead. If the function is too big, consider splitting it into parts.
- I have two identical functions, should I refactor them into one?
    - Yes.
- I have a function that I think could be refactored based on the above principles but it isn't duplicated anywhere, should I do it?
    - If you can weaken the preconditions, strengthen the postconditions, or if you think the function is too big, it should be OK to refactor. This assumes that any requirement could change anywhere in your program at any time.
- I have a function that I think might be big enough that it can be split into parts, but it's borderline. Should I split it?
    - The triviality of a function is subjective. If you think that it improves ergonomics, makes the code easier to read, or reduces the possibility of errors compared to writing it inline, do it.
- I have a function that could be more generic, should I do it?
    - If you don't weaken the postconditions in doing so, or make the function too big, then it should be fine. Caveat: sometimes the function already does it's job and you don't need it to be generic. Then it might not be worth the effort.
- I have a function where the return type could be made more specific, should I do it?
    - If you don't weaken the preconditions, or make the function too big, then go ahead.
- What about return type polymorphism? doesn't that make code more reusable while making the postconditions weaker?
    - Return type polymorphism is equivalent to passing an (often zero sized) type as an argument, therefore weakening the preconditions. The three principles do not state whether or not this tradeoff is a good or a bad thing. Consider having both a specialized variant, and a generic one for different circumstances.
- I have an object that might be too large, should I split it into two?
    - If at least one of the methods does not need access to the whole object, consider splitting it. Exception: delegation, getters/setters.
- Is XXX is a good abstraction?
    - If you can define the preconditions, postconditions and invariants well, then it should be OK, otherwise: probably not.
- Is XXX **object** a good abstraction?
    - Examine the methods individually, and apply the principles above, If all of the methods individually are OK, then the whole object should be too assuming that you only access the object through its methods.


## Conclusion

DRY is a programming principle that has many limitations. An often cited limitation is the capacity of DRY to create inefficient abstractions. Heuristic solutions have been adopted to try to find a good medium. This article presents a different way of deriving DRY from some basic assumptions. It also provides three principles for writing abstractions that aim to be reusable. These three principles allow programmers to write code with no information about the calling context whilst providing some guarantees that the code can and will be reused as many times as possible.

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
