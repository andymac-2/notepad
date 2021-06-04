# Working example: FizzBuzz

```typescript
type Fix<A> = (arg: Fix<A>) => A

type Unit = <T>(
  unit: () => T
) => T;

type Bool = <T>(
  $true: () => T,
  $false: () => T,
) => T;

type Num = <T>(
  zero: () => T,
  succ: (n: Num) => T,
) => T;

type Lazy<A> = () => A;
type Def = <D, R>(definition: D, continuation: (arg: D) => R) => R;

// The first definition is a little weird, since we are defining the 'def' function
(define => define((definition, continuation) => continuation(definition)))((def: Def) =>

// definitions are of the form
//
//   def(value, (variableName: type) =>
//
// Notice the unbalanced parenthesis. Theoretically these should all start on their
// own level of indentation, but that's crazy.s

// Primitives
// These are all things we cannot do with only closures
def(console.log,                      log =>
def((a: string, b: string) => a + b,  join =>
def("Fizz",                           fizz =>
def("Buzz",                           buzz =>
def(0,                                zeroNative =>
def((n: number) => ++n,               succNative => 

// Unit data type (void)
def(unit => unit(),                   (Unit: Unit) =>

// Boolean data type
def(($true, _$false) => $true(),      (True: Bool) =>
def((_$true, $false) => $false(),     (False: Bool) =>

// if function. Execute the body if true.
def((b: Bool, func: () => Unit) => b(func, () => Unit),       iff =>

// Number data type
def((zero, _succ) => zero(),                    (Z: Num) =>
def((n: Num): Num => (_zero, succ) => succ(n),  (S: (n: Num) => Num) =>

// Pattern matching
// use iff, if is a reserved word
def(<T>(b: Bool, $true: () => T, $false: () => T): T => b($true, $false),     ifElse =>
def(<T>(n: Num, zero: () => T, succ: (pred: Num) => T): T => n(zero, succ),   match =>

// Recursion
// fix: Not really any good way to explain this
def(
  <A>(func: (self: () => A) => A): A => (
      (rec: Fix<A>) => func(() => rec(rec))
    )(
      (rec: Fix<A>) => func(() => rec(rec))
    ),
    fix =>

// defFix: define recursive functions
def(<A, R>(f: (self: Lazy<A>) => A, cont: (arg: A) => R) => def(fix(f), cont),  defRec =>

// Boolean functions
// not
def((predicate: Bool): Bool =>
  ifElse(predicate,
    () => False,
    () => True,
  ),
  not =>

// Numeric operations
// add
defRec(self => (a, b) => 
  match(a,
    () => b,
    (n) => self()(n, S(b)),
  ),
  (add: (a: Num, b: Num) => Num) =>

// double
defRec(self => (n) => 
  match(n,
    () => Z,
    (pred) => S(S(self()(pred)))
  ),
  (double: (n: Num) => Num) =>

// equals
defRec(self => (a, b) => 
  match(a,
    () => match(b,
      () => True,
      () => False,
    ),
    (predA) => match(b,
      () => False,
      (predB) => self()(predA, predB),
    )
  ),
  (equals: (a: Num, b: Num) => Bool) =>

// greater than
defRec(self => (a, b) => 
  match(a,
    () => False,
    (predA) => match(b,
      () => True,
      (predB) => self()(predA, predB),
    )
  ),
  (gt: (a: Num, b: Num) => Bool) =>

def((a: Num, b: Num) => not(gt(a, b)),                  leq =>

// to native
defRec(self => (n) =>
  match(n,
    () => zeroNative,
    (pred) => succNative(self()(pred))
  ),
  (toNative: (n: Num) => number) =>

// Numbers
def(S(Z),                       n1 =>
def(S(S(Z)),                    n2 =>
def(S(S(S(S(Z)))),              n4 =>
def(double(double(double(n4))), n32 =>
def(double(n32),                n64 =>
def(add(n4, add(n32, n64)),     n100 =>

// fizzBuzz
defRec(self => (n, fizzCount, buzzCount) =>
  iff(leq(n, n100), () =>
    def(
      ifElse(equals(fizzCount, n2),
        () => Z,
        () => S(fizzCount),
      ),
      newFizzCount =>

    def(
      ifElse(equals(buzzCount, n4), 
        () => Z,
        () => S(buzzCount),
      ),
      newBuzzCount =>

    def(
      match(fizzCount,
        () => match(buzzCount,
          () => join(fizz, buzz),
          () => fizz
        ),
        () => match(buzzCount,
          () => buzz,
          (): number | string => toNative(n)
        )
      ),
      line => (

    log(line),
    self()(S(n), newFizzCount, newBuzzCount)

    ))))
  ),
  (fizzBuzz: (n: Num, fizzCount: Num, buzzCount: Num) => Unit) => (

fizzBuzz(n1, n1, n1)
// All of the parentheses from the defs are gathered here
))))))))))))))))))))))))))))))));
```

