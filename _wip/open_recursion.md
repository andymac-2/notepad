---
title: Inheritance Versus Open Recursion
category: notes
tags:
- open recursion
- functional programming
- object orientated programming
- inheritance
- memoization
- javascript
- typescript
- java
---

### Inheritance vs Open Recursion

A very functional idea in object orientated programming.

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

### Summary

- Inheritance can be used to implement memoization using dynamic dispatch for example.
- Dynamic dispatch can be desugared to continuation passing for those who want to emulate inheritance in functional languages.

## Memoization Using Inheritance

Using inheritance, you can call a base method from a derived class:

```typescript
class Base {
    a() {
        console.log('Base Method A');
    }
}

class Derived extends Base {
    b() {
        console.log('Derived Method B');
        super.a();
    }
}

const derived = new Derived();
derived.b();
```

It's also possible to do the opposite and call a derived method from a base class:

```typescript
abstract class Base {
    a() {
        console.log('Base Method A');
        this.b();
    }

    abstract b(): void
}

class Derived extends Base {
    b() {
        console.log('Derived Method B');
    }
}

const derived = new Derived();
derived.a();
```

An interesting result is that the base and derived methods can call each other recursively:

```typescript
class Base {
    method() {
        console.log('Base Method');
        this.method();
    }
}

class Derived extends Base {
    method() {
        console.log('Derived Method');
        super.method();
    }
}

const derived = new Derived();
derived.method(); // crashes or runs indefinitely
```

Say I have a recursive function that's particularly slow to execute, for example a naive implementation of the fibonacci sequence:

```typescript
fib(n: number): number {
    return n === 0 || n === 1 ? 1 : fib(n - 2) + fib(n - 1);
}
```

In the tradition of object orientated programming we can put that function into a class:

```typescript
class Fib {
    call(n: number): number {
        return n === 0 || n === 1 ? 1 : this.call(n - 2) + this.call(n - 1);
    }
}

const fib = new Fib();
console.log(fib.call(0));
console.log(fib.call(10));
// console.log(fib.call(100)); // will not terminate in any reasonable amount of time
```

One method to make this a lot faster is to memoize the function. Using inheritance, we can override the function and memoize it using a derived class:

```typescript
class MemoFib extends Fib {
    store: Map<number, number> = new Map();

    call(n: number): number {
        const storedResult = this.store.get(n);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = super.call(n);
        this.store.set(n, result);
        return result;
    }
}

const memoFib = new MemoFib();
console.log(memoFib.call(0));
console.log(memoFib.call(10));
console.log(memoFib.call(100)); // runs almost instantaneously
```

We can also reverse the inheritance hierarchy so the function inherits from memoize. This will have slightly different performance characteristics:

```typescript
class Memoize {
    store: Map<number, number> = new Map();

    call(n: number): number {
        const storedResult = this.store.get(n);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = this.call(n);
        this.store.set(n, result);
        return result;
    }
}

class MemoFib extends Memoize {
    call(n: number): number {
        return n === 0 || n === 1 ? 1 : super.call(n - 2) + super.call(n - 1);
    }
}

const memoFib = new MemoFib();
console.log(memoFib.call(0));
console.log(memoFib.call(10));
console.log(memoFib.call(100));
```

We could stop here, however, this method is severely limited. If `Memoize` inherits `Fib`, the memoize logic is limited to the fibonacci function, and we have to replicate it for every function we want to memoize. If `Fib` inherits `Memoize`, then we can't swap out the kind of memoization we want to use.

## This is an idea borrowed from functional programming

To see this, we need to make some changes to our code. In OOP, a method call `obj.method(arg)` desugars to `method(obj, arg)`. We can desugar the `Fib` class that way:

```typescript
interface Fib {
    call(self: Fib, n: number): number;
}

const newFib = (): Fib => ({
    call: (self: Fib, n: number) => {
        return n === 0 || n === 1 ? 1 : self.call(self, n - 2) + self.call(self, n - 1);
    }
});
```

The same can be done with the `MemoFib` class:

```typescript
interface MemoFib extends Fib {
    superClass: Fib;
    store: Map<number, number>;
}

const newMemoFib = (): MemoFib => ({
    superClass: newFib(),
    store: new Map(),
    call: (self: MemoFib, n: number): number => {
        const storedResult = self.store.get(n);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = self.superClass.call(self, n);
        self.store.set(n, result);
        return result;
    }
});

const memoFib = newMemoFib();
console.log(memoFib.call(memoFib, 0));
console.log(memoFib.call(memoFib, 10));
console.log(memoFib.call(memoFib, 100));
```

We can perform some simplifications to the code:

- The type `Fib` can be simplified to a closure
- The wrapper function `newFib` can be removed, all we care about is the contents
- The only pert of `MemoFib` we want to be publicly available is `call`, so we can return a closure instead of an object
- `superClass` can be removed since it's always going to be the same

After applying these simplifications we get the following;

```typescript
type Fib = (self: Fib, n: number) => number;

const fib = (self: Fib, n: number) => {
    return n === 0 || n === 1 ? 1 : self(self, n - 2) + self(self, n - 1);
}

const newMemoFib = (): Fib => {
    const store = new Map();
    return (self: Fib, n: number): number => {
        const storedResult = store.get(n);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = fib(self, n);
        store.set(n, result);
        return result;
    }
};

const memoFib = newMemoFib();
console.log(memoFib(memoFib, 0));
console.log(memoFib(memoFib, 10));
console.log(memoFib(memoFib, 100));
```

We can perform a further simplification. Closures have access to themselves if we give them a name, so we shouldn't need to pass `self` to `self`. We can remove the `self` argument from `Fib` that way:

```typescript
type Fib = (n: number) => number;

const fib = (self: Fib, n: number) => {
    return n === 0 || n === 1 ? 1 : self(n - 2) + self(n - 1);
}

const newMemoFib = (): Fib => {
    const store = new Map();
    const memoized = (n: number): number => {
        const storedResult = store.get(n);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = fib(memoized, n);
        store.set(n, result);
        return result;
    }
    return memoized;
};

const memoFib = newMemoFib();
console.log(memoFib(0));
console.log(memoFib(10));
console.log(memoFib(100));
```

`newMemoFib` can memoize anything if we pass it as an argument. We can also make a generic version of `Fib` over any argument or return value:

```typescript
type Callable<A, R> = (arg: A) => R
type Memoizable<A, R> = (self: Callable<A, R>, arg: A) => R;

const fib: Memoizable<number, number> = (self, n) => {
    return n === 0 || n === 1 ? 1 : self(n - 2) + self(n - 1);
}

const factorial: Memoizable<number, number> = (self, n) => {
    return n === 0 ? 1 : self(n - 1) * n;
}

const newMemo = <A, R>(baseClass: Memoizable<A, R>): Callable<A, R> => {
    const store = new Map<A, R>();
    const memoized = (arg: A): R => {
        const storedResult = store.get(arg);
        if (storedResult !== undefined) {
            return storedResult;
        }

        const result = baseClass(memoized, arg);
        store.set(arg, result);
        return result;
    }
    return memoized;
};

const memoFib = newMemo(fib);
console.log(memoFib(100));

const memoFact = newMemo(factorial);
console.log(memoFact(20));
```

We see that `newMemo` is just a function that takes something that is memoizable and turns it into something callable. If I realize that my factorial function doesn't perform any better with memoization, I can swap out `newMemo` with something else as long as it turns my memoizable function into something callable:

```typescript
const factorial: Memoizable<number, number> = (self, n) => {
    return n === 0 ? 1 : self(n - 1) * n;
}

const newRecursive = <A, R>(baseClass: Memoizable<A, R>): Callable<A, R> => {
    const recursive = (arg: A): R => baseClass(recursive, arg);
    return recursive;
};

const memoFact = newRecursive(factorial);
console.log(memoFact(20));
```

Excellent. I can memoize any function that is memoizable, and I can swap out the kind of recursion I use. I no longer have any of the limitations of the inheritance version.

## Conclusion

It turns out maybe object orientated and functional programming have something in common. When methods are overridden in an inheritance hierarchy, this desugars to continuation passing. Functional programs that use continuation passing can be converted to objects using inheritance, and inheritance can be converted to open recursion.

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
