# Product Types (structs)

The simplest kind of struct (apart from a struct that contains a single value) is a pair of values. To have a pair of values, we need the following:

- A way to create a pair
- A way to get the first value of the pair
- A way to get the second value of the pair

This might look like the following:

```typescript
const pair = Pair(3, 'hello');

console.log(first(pair));   // prints 3
console.log(rest(pair));    // prints 'hello'
```

One implementation of this is as follows:

```typescript
interface Pair<F, R> {
  <T>(visitor: (first: F, rest: R) => T): T
};

const Pair = <F, R>(first: F, rest: R): Pair<F, R> => (visitor) => visitor(first, rest);

const first = <F, R>(pair: Pair<F, R>): F => pair((first) => first);
const rest = <F, R>(pair: Pair<F, R>): R => pair((_first, rest) => rest);
```

While this looks like a lot of code, the majority of it is simply type annotations as can be seen by the compiled output:

```javascript
const Pair = (first, rest) => (visitor) => visitor(first, rest);

const first = (pair) => pair((first) => first);
const rest = (pair) => pair((_first, rest) => rest);
```

Users of lisp may be more comfortable with an alternate naming scheme:

```typescript
const car = first;
const cdr = rest;
```

## Less than two fields

It's possible to create a struct with only one item:

```typescript
interface Single<A> {
  <T>(visitor: (first: A) => T): T
};

const Single = <A>(first: A): Single<A> => (visitor) => visitor(first);
const first = <A>(single: Single<A>): A => single((first) => first);
```

Note that a `Pair<A, B>` is also a `Single<A>` so `first` can be used on both `Single` and `Pair`s:

```typescript
const pair = Pair(3, 'hello');
console.log(first(pair));   // prints 3

const single = Single('single');
console.log(first(single))  // prints 'single'
```

It's also possible to create a struct with zero fields:

```typescript
interface Unit {
  <T>(visitor: () => T): T
}

const Unit = (): Unit => (visitor) => visitor();
```

This is not particularly useful now, but there are functional contexts where a structure that has no fields can be useful.

## More than two fields

The pattern can be extended for mote than two items:

```typescript
interface Triplet<A, B, C> {
  <T>(visitor: (first: A, second: B, third: C) => T): T
};

const Triplet = <A, B, C>(first: A, second: B, third: C): Triplet<A, B, C> => (visitor) => visitor(first, second, third);
const third = <C>(triplet: Triplet<unknown, unknown, C>): C => triplet((_first, _second, third) => third);
```

