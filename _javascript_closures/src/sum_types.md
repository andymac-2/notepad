# Sum Types

A useful but often missing feature of standard languages is *sum types*. Sum types represent something that can be one thing **or** another.

The simplest example of a sum type is a boolean. It can either be `true` or `false`. There are no other valid values. In the [chapter on if statements](./if.md) we explore how to implement booleans using only closures. To recap, we can use the following syntax instead of the built in javascript `if` statement:

```typescript
const test = (condition: Bool) => (
  ifElse(condition, {
    True: () => console.log('condition was true'),
    False: () => console.log('condition was false')
  })
);

test(True);     // prints 'condition was true'
test(False);    // prints 'condition was false'
```

The implementation for `ifElse`, `True` and `False` is given here:

```typescript
type BoolVisitor<T> = {
    True: () => T,
    False: () => T
};
type Bool = <T>(visitor: BoolVisitor<T>) => T;

const True: Bool = ({ True }) => True();
const False: Bool = ({ False }) => False();

const ifElse = <T>(
    condition: Bool,
    visitor: BoolVisitor<T>
): T => (
    condition(visitor)
);
```

## Three possibilities

A boolean has only two possibilities. However what if we want three or more? An example where three possibilities is useful is comparisons. A value can either be less than, equal to, or greater than another value.

Normally, JavaScript handles this with a number. For example, `String.prototype.localeCompare` returns a negative value to mean 'less than', a positive number to mean 'greater than' and zero to mean 'equal to'. Ideally we have something like a boolean, but instead of `true` and `false`, we have `lessThan`, `greaterThan` and `equalTo`.

Using the boolean example above as a template, we can create our own `Comparison` type with three possibilities:

```typescript
interface ComparisonVisitor<T> {
    LessThan: () => T,
    EqualTo: () => T,
    GreaterThan: () => T,
};
interface Comparison {
  <T>(visitor: ComparisonVisitor<T>): T
};

const LessThan: Comparison = ({ LessThan }) => LessThan();
const EqualTo: Comparison = ({ EqualTo }) => EqualTo();
const GreaterThan: Comparison = ({ GreaterThan }) => GreaterThan();
```

Using the `ifElse` from above, we can compare booleans:

```typescript
const compareBooleans = (a: Bool, b: Bool): Comparison => (
  ifElse(a, {
    True: () => ifElse(b, {
      True: () => EqualTo,
      False: () => GreaterThan,
    }),
    False: () => ifElse(b, {
      True: () => LessThan,
      False: () => EqualTo
    })
  })
)
```

We can check that this works as intended:

```typescript
const testComparison = (a: Bool, b: Bool) => (
  match(compareBooleans(a, b), {
    LessThan: () => console.log('less than'),
    EqualTo: () => console.log('equal to'),
    GreaterThan: () => console.log('greater than')
  })
);

testComparison(False, False)  // prints 'equal to'
testComparison(False, True)   // prints 'less than'
testComparison(True, False)   // prints 'greater than'
testComparison(True, True)    // prints 'equal to'
```

## More possibilities

We could implement weekdays for example:

```typescript
interface WeekdayVisitor<T> {
    Monday: () => T,
    Tuesday: () => T,
    Wednesday: () => T,
    Thursday: () => T,
    Friday: () => T,
    Saturday: () => T,
    Sunday: () => T
};
interface Weekday{
  <T>(visitor: WeekdayVisitor<T>): T
};

const Monday: Weekday = ({ Monday }) => Monday();
const Tuesday: Weekday = ({ Tuesday }) => Tuesday();
const Wednesday: Weekday = ({ Wednesday }) => Wednesday();
const Thursday: Weekday = ({ Thursday }) => Thursday();
const Friday: Weekday = ({ Friday }) => Friday();
const Saturday: Weekday = ({ Saturday }) => Saturday();
const Sunday: Weekday = ({ Sunday }) => Sunday();
```

There's not much point showing the same matching operations as before. But we notice that there's a fair amount of boilerplate here. If we want, we could define the weekdays a bit differently:

