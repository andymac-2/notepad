# `if` statements

```typescript
// An `if` statement usually goes something like this:

const normalIf = (condition: boolean) => {
    if (condition) {
        return "Condition was true";
    } else {
        return "Condition was false";
    }
};

console.log(normalIf(true));    // "Condition was true"
console.log(normalIf(false));   // "Condition was false"

// Note that I use an else statement. This function returns a string, so I have to handle
// the case where the condition is false.

// Instead on an `if` statement, I need some kind of `if` function that will do the same
// job. I use `otherwise` because `else` is a reserved word:

const ifElse = <T>(
    condition: boolean,
    visitor: { then: () => T, otherwise: () => T }
) => (
    condition ? visitor.then() : visitor.otherwise()
);

// I can use the function like so:

const ifFunction = (condition: boolean) => ifElse(condition, {
    then: () => "Condition was true",
    otherwise: () => "Condition was false"
});

console.log(ifFunction(true));    // "Condition was true"
console.log(ifFunction(false));   // "Condition was false"

// I still have to use booleans and the ternary operator for this to work. I can replace
// the booleans however with two helper functions

type Bool = <T>(trueBranch: () => T, falseBranch: () => T) => T;

const True: Bool = (doThisIfTrue, doThisIfFalse) => doThisIfTrue();
const False: Bool = (doThisIfTrue, doThisIfFalse) => doThisIfFalse();

// now I can redefine my `ifElse` function:

const ifElse2 = <T>(
    condition: Bool,
    visitor: { then: () => T, otherwise: () => T }
): T => (
    condition(visitor.then, visitor.otherwise)
);

// There are still some things that cannot be done without native booleans. For example,
// the expression `a < b` will return a native boolean. For now, it's not possible to
// compare two numbers without getting a native boolean. 

// Variations:

// I can omit the visitor, and pass `then` and `otherwise` directly as arguments:

const ifElse3 = <T>(
    bool: Bool,
    then: () => T,
    otherwise: () => T
): T => (
    bool(then, otherwise)
);

const ifFunction2 = (condition: Bool) => ifElse3(condition,
    () => "Condition was true",
    () => "Condition was false"
);

console.log(ifFunction2(True));         // "Condition was true"
console.log(ifFunction2(False));        // "Condition was false"

// It's less verbose that way, but not as explicit.

// The `ifElse3` function seems a bit useless: all it does is apply a function to it's arguments
// You can use the boolean directly if you like

const ifFunction3 = (bool: Bool) => bool(
    () => "Condition was true",
    () => "Condition was false"
);

console.log(ifFunction3(True));         // "Condition was true"
console.log(ifFunction3(False));        // "Condition was false"

// This is the most terse out of the examples and the least explicit. The threee variants are
// similar enough that it boils down to personal preference which one you use.

```