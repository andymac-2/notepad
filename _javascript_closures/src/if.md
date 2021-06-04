# `if` statements

A basic `if` statement usually goes something like this:

```javascript
if (condition) {
    console.log("Yay!. The condition was true!");
} else {
    console.log("Oh, no. The condition was false.");
}
```

...but I want it to look something like this:

```javascript
iff(condition, {
    doThisIfTrue: () => console.log("Yay!. The condition was true!"),
    doThisIfFalse: () => console.log("Oh, no. The condition was false.")
})
```

...or even just this:

```javascript
iff(condition,
    () => console.log("Yay!. The condition was true!"),
    () => console.log("Oh, no. The condition was false.")
);
```

Note that I don't have an implementation of `iff` just yet, I'm just trying to figure out what I want it to look like when I'm done. I use the name `iff` instead of `if` since `if` is a reserved word.

## True and False

Let's create two helper functions: `callTrue` and `callFalse`. They both take two arguments: `doThisIfTrue` and `doThisIfFalse`. `callTrue` will just ignore `doThisIfFalse` and call `doThisIfTrue`. `callFalse` will do the opposite:

```javascript
const callTrue = (doThisIfTrue, doThisIfFalse) => doThisIfTrue();
const callFalse = (doThisIfTrue, doThisIfFalse) => doThisIfFalse();
```

We're almost there. I want my `iff` function to call `doThisIfTrue` when it receives true, and `doThisIfFalse` when it receives `false`. This isn't real javascript but it shows me what I want to do:

```javascript
iff(true, doThisIfTrue, doThisIfFalse) => callTrue(doThisIfTrue, doThisIfFalse);
iff(false, doThisIfTrue, doThisIfFalse) => callFalse(doThisIfTrue, doThisIfFalse);
```

Hmm. I somehow need to convert `true` to `callTrue` and `false` to `callFalse`, and then my function works without any if statements. I don't think I can do that, but I can *rename* `callTrue` to `True` and `callFalse` to `False` and now my `iff` function works.

```typescript
// Typescript
type Bool = <T>(trueBranch: () => T, falseBranch: () => T) => T;

const True: Bool = (doThisIfTrue, doThisIfFalse) => doThisIfTrue();
const False: Bool = (doThisIfTrue, doThisIfFalse) => doThisIfFalse();

const iff = <T>(boolValue: Bool, doThisIfTrue: () => T, doThisIfFalse: () => T): T => 
    boolValue(doThisIfTrue, doThisIfFalse);

// prints "Yay!. The condition was true!"
iff(True,
    () => console.log("Yay!. The condition was true!"),
    () => console.log("Oh, no. The condition was false.")
);

// prints "Oh, no. The condition was false."
iff(False,
    () => console.log("Yay!. The condition was true!"),
    () => console.log("Oh, no. The condition was false.")
);
```

So I have something that looks like an `if` statement and quacks like an `if` statement, therefore it must be an `if` statement. Now there are some pesky programmers that still use `true` and `false`, so I can write a quick conversion function:

```typescript
const toBoolean = (bool: boolean) => bool ? True : False;
```

In summary, if I don't use `true` and `false` directly, I can replace `if` statements and boolean values with nothing but functions:

```typescript
// before
if(condition) {
    console.log("Yay!. The condition was true!");
} else {
    console.log("Oh, no. The condition was false.");
}

//after
iff(toBoolean(condition),
    () => console.log("Yay!. The condition was true!"),
    () => console.log("Oh, no. The condition was false.");
);
```

In fact, my version of `iff` has some benefits over the built in `if`:

- `iff` is an expression, so the result can be assigned to a variable e.g.: `const result = iff(...)`.
- `var` will not be hoisted to the surrounding context in an `iff` expression (not that you still use `var` anyway)

*Note: an astute observer will notice that `return` and `throw` will not work the same way between the `if` and `iff`, however we will get to that later in the book*