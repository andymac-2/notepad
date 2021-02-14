---
title: Prove Santa Claus is real using Typescript
category: notes
tags:
- typescript
- Curry-Howard
- type theory
- proof
- Curry's paradox
- Functional Programming
- Javascript
- Santa Claus
---

### Prove Santa Claus is real using Typescript

![We're going to make it happen]({{ '/img/2020-02-14/hell_frozen_over.jpg' | relative_url}})

### Summary

- Curry's paradox allows proofs of any statement in some languages that have the ability to construct a self referential sentence.
- This paradox allows a proof of any statement, no matter how absurd.
- Some statically typed programming languages allow for basic logical statements to be made and subsequently proven. Typescript is one of these languages.

## The Premise

> If this sentence is true, then Santa Claus is real.

This statement seems innocuous enough, but it exposes a fatal flaw in some logical systems. If it is possible for the above sentence to exist as a statement, then it follows that Santa Claus is real. In fact it doesn't matter what the second half of this sentence is. You could prove that the sky is purple, unicorns outnumber humans ten to one, or that ants wear hats. This paradox is called "Curry's paradox" after the famous mathematician Haskell Curry.

So how does the paradox work? Consider a more normal example of a sentence of the form "if A then B". For example consider the following sentence:

> If you add salt and water together, you get salt water

In order to prove a statement like this true, first we assume the first part of the sentence has already happened and then see if the second part came true. In this example, we assume that we have already mixed salt and water together (the first part of the sentence). Then we see if the second part came true. Since we have salt water, we can safely say that the sentence is true.

Cool, let's get back to the original example

> If this sentence is true, then oranges are green

...Close enough. To prove this statement, first we assume the first bit (the sentence is true), then we see if the second part came true as a result. We have assumed that the sentence is true, so that would mean ...oranges are grey. Cool, we proved oranges are grey if the sentence is true, so that means the sentence *is* true.

So we have proven that the sentence is true by following normal logical rules. But if the sentence really is true, then oranges are grey.

## Curry-Howard

Curry's paradox exists in a lot of places including english, and naive set theory. In addition to this we can even write it in a lot of programming languages. To do this, we need to understand the connection between say a formal language used in mathematics, and a programming language. The connection is referred to as the *Curry-Howard correspondence* and it basically states that types are predicates and implementations are proofs. *Note that the Curry in "Curry's paradox" and "Curry-Howard" are in fact the same person!*

In typescript, we are a bit limited, but there are some easy statements that we can write using just types. A type is the same as a *definition* or an *axiom* in mathematics.

```typescript
// Define salt, pepper, meat, potatoes, and ducks.
type Salt = 'salt'
type Pepper = 'pepper'
type Meat = 'meat'
type Potatoes = 'potatoes'
type Duck = 'duck'

// Define Santa Claus as never existing
type SantaClaus = never;

// A type for A and B
type And<A, B> = {
    theFirstPart: A;
    theSecondPart: B;
}

// To get a christmas quacker, we need Santa Claus and a duck
type CristmasQuacker = And<SantaClaus, Duck>;

// A type for A or B
type Or<A, B> = 
    { theFirstOption: A } |
    { theSecondOption: B };

// To get seasoning we need salt or pepper
type Seasoning = Or<Salt, Pepper>

// To get a meal, we need seasoning, and either meat and potatoes
type Meal = And<Seasoning, Or<Meat, Potatoes>>;
```

We have shown that we can write basic statements which contain `And` and `Or`. We can also write statements that contain "if" in them. The logical symbol for "if" is the right arrow (→), which corresponds to the fat arrow (`=>`) of JavaScript:

```typescript
// If both A and B exist, then A exists
type First = <A, B>(both: And<A, B>) => A;

// If both A and B exists, then B exists
type Second = <A, B>(both: And<A, B>) => B;
```

This is all well and good, but these are just definitions. Take `First` as an example. `First` is a sentence that states that if both A and B exist, then A exists. However, `First` is not a proof of that statement, it is just that statement itself.

In order to prove something to be true, we have to create an instance of that type. Here are some examples:

```typescript
const saltExists: Salt = 'salt';
const pepperExists: Pepper = 'pepper';

const seasoningExists: Seasoning = {
    theSecondOption: 'pepper'
};

const mealsExist: Meal = {
    theFirstPart: seasoningExists,
    theSecondPart: { theFirstOption: 'meat' }
}

// A exists because the first part of A and B is A.
const firstIsTrue: First = (both) => both.theFirstPart;

// B exists because the second part of A and B is B.
const secondIsTrue: Second = (both) => both.theSecondPart;
```

Based on our definitions of salt, pepper, meat and potatoes, we have proven the existence of seasoning, an meals! In addition to that we have proven our `First` and `Second` predicates from above.

## Falsehood

Proving something to be true is all well and good, but what if we want to prove something false? Luckily TypeScript provides us with a type which can never be created called `never`. This type is sometimes referred to as `void`, `false`, or `bottom`. The mathematical symbol for `never` is this upside down T: ⊥. We can use "A → ⊥" (if A then false) to mean "A does not exist":

```typescript
// A does not exist
type Not<A> = (false_statement: A) => never;
```

Now we have everything to prove things are not true:

```typescript
// Santa Claus does not exist because we defined him not to exist
const santaClausIsntReal: Not<SantaClaus> =
    (santaClaus) => santaClaus;

// Christmas Quackers do not exist because because the first ingredient
// of a Christmas Quacker (Santa Claus) does not exist.
const christmasQuackersArentReal: Not<ChristmasQuacker> =
    (christmasQuacker) => christmasQuacker.theFirstPart;
```

We can even perform other proofs using `Not`. For example, what about the principle of explosion which states that given a contradiction we can prove anything:

```typescript
// A contradiction is when something is both true
// and false at the same time
type Contradiction<T> = And<T, Not<T>>;

// Lemma: given a contradiction, we can prove anything
type PrincipleOfExplosion =
    <T, U>(contra: Contradiction<T>) => U;

// ...and here's a proof:
const principleOfExplosionIsTrue: PrincipleOfExplosion =
    (contra) => contra.theSecondPart(contra.theFirstPart)
```

Exciting! We can prove basic mathematical theorems using only TypeScript. We now have everything we need to move onto our paradox.

## Curry's paradox

We need some way to express the statement "If this sentence is true, then Santa Claus is real" as a type. To begin, we will give the sentence a name, say `CurrysParadox`.

```typescript
type CurrysParadox = ??
```

This way we can rewrite our sentence to say "If `CurrysParadox` is true, then Santa Claus is real". We know how to write a sentence with "if" in it, and we know how to write "Santa Claus is real", so we can now fill in the blank:

```typescript
type CurrysParadox =
    (paradox: CurrysParadox) => SantaClaus;
```

To prove that the statement `CurrysParadox` is true, we need to create an instance of it:

```typescript
const currysParadoxIsTrue: CurrysParadox =
    (paradox) => ??
```

We are given an instance of `CurrysParadox` an argument, and we want to return an instance of `SantaClaus`. `CurrysParadox` is a function that returns a `SantaClaus`, so if we can call it, then we can get `SantaClaus` as a result. We can call paradox by passing itself as an argument:

```typescript
const currysParadoxIsTrue: CurrysParadox =
    (paradox) => paradox(paradox);
```

Now that we know that `CurrysParadox` is true, we need to get an instance of `SantaClaus` to complete the proof. Same as above, `currysParadoxIsTrue` is an instance of `CurrysParadox`. If we can call `currysParadoxIsTrue`, then we will get `SantaClaus` as a result. We can call `currysParadoxIsTrue` by passing itself as an argument:

```typescript
const santaClausIsReal: SantaClaus =
    currysParadoxIsTrue(currysParadoxIsTrue);
```

This type checks, so once this code runs we will have an instance of `SantaClaus`. We have finally proven that Santa Claus exists using nothing but TypeScript!

The absurdity continues. From the principle of explosion it is possible to prove anything if you have an untrue statement first:

```typescript
const anythingIsTrue: <T>() => T = santaClausIsReal;
```

## Uh, What?

Hold on a second. When we were defining our types, we said `SantaClaus` was equal to `never`. When we run this program, what actually gets stored in the variable we named `santaClausIsReal`? Is it a number? Is it a function? Didi we actually manage to store Santa Claus in a variable? Running the program gives us the answer:

```
Uncaught RangeError: Maximum call stack size exceeded
```

So the code fails with a stack overflow. This is strange considering that we *didn't use any recursion in our entire program*. But all is well. We never actually assign anything to the variable `santaClausIsReal` because the program fails before the function returns. Unfortunately, we don't get to see Santa after all. As consolation, we are able to live in a world where the rules of mathematics still make sense.

## So what happened?

If we write `currysParadoxIsTrue` inline for `santaClausIsReal`, and shorten `paradox` to `p` we get the following statement:

```typescript
(p => p(p))((p: CurrysParadox) => p(p));
```

The astute functional programmer will realize that this is in fact Omega from lambda calculus. It is a famous example of a statement, that when executed, produces a perfect copy of itself. This explains the stack overflow: the function body produces a copy of itself, and then calls the copy repeatedly until the stack runs out of space.

So how do we reconcile the fact that TypeScript allowed us to write these absurd kinds of results in the first place? Is TypeScript broken? Well, no. In order to facilitate writing programs with loops and other non-obvious exit conditions, TypeScript allows us to write non-terminating programs. When we give a type to a function, we declare that when it returns, the value will have the specified type. If it never returns, then we never break any of the typing rules.

## Conclusion

Unfortunately we don't get to prove Santa Claus is real, but we do get an interesting insight into the parallels between mathematics and the type checker. Many languages allows us to write basic logical statements as types, and prove them using an implementation. However, this has it's limitations. The proofs of statements are only applicable when it is known ahead of time that the program will terminate. In other cases, the language is susceptible to contradiction.

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
