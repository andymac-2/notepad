## Javascript is too complicated...

Let's simplify it. There are lots of complicated things in modern javascript like if statements, objects, booleans, and classes. Wouldn't it be nice if we could get rid of all of this complicated stuff and just write clean code using only the basics?

This book will cover how to write almost any code using nothing but closures in pure JavaScript. Note that all of these principles apply to other languages as well.

## But why?

Primarily, out of morbid curiosity. But there are legitimate reasons to read this book:

- You're curious too.
- You want to learn functional programming principles in the worst possible way.
- A language doesn't have the feature you want but has closures.
- An existing feature might be implemented more efficiently or with greater flexibility using the closure approach.
- You want to understand lambda calculus in a way that can actually be applied anywhere.

## Typescript and JSX:

This book actually uses TypeScript. This is to make some of the annotation easier to understand. Some of the parts of this book can get quite technical, so TypeScript makes it a lot easier to make less mistakes.

In some cases, the user may be unable to compile the typescript examples if they have JSX enabled. Disabling JSX makes it possible to write examples such as the following:

```typescript
const Unit = <T>(f: () => T): T => f();
```

JSX users will find that this will not compile. TypeScript thinks that the `<T>` is a JSX element rather than a type annotation. One way to get around this issue is to use old style functions:

```typescript
const Unit2 = function<T>(f: () => T): T { return f() };
```

Another way around this limitation is to define the type of the function first and then use it:

```typescript
type Unit = <T>(f: () => T) => T;
const Unit3: Unit = (f) => f();
```
