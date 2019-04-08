---
title: On Applicative Functors
---

### Rather than explain what an applicative is, we are only truly concerned with how to use them.

Skip to the end to read the "golden rules" of applicative functors. 

![A mysterious symbol appears.]({{ '/img/on_applicative.jpg' | relative_url }})

Consider a datatype which belongs to the `Aplicative` typeclass in Haskell. `Applicative` allows us to combine actions together, as long as the return result of the actions do not need to be used by other actions. Consider the following example:

```haskell
main :: IO ()
main = do
    str <- getFourLines
    putStrLn str

getFourLines :: IO String
getFourLines = go <$> getLine <*> getLine <*> getLine <*> getLine where
    go line1 line2 line3 line4 = line1 ++ line2 ++ line3 ++ line4
```

 `getFourLines` can also be written as:

```haskell
getFourLines :: IO String
getFourLines = go
    <$> getLine
    <*> getLine
    <*> getLine
    <*> getLine
    where
        go line1 line2 line3 line4 = line1 ++ line2 ++ line3 ++ line4
```

`getFourLines` will get four lines from `stdin` and concatenate them together. We have four calls to `getLine`. The first is prepended by `<$>`, and the rest are prepended by `<*>`. Then we have a function called `go`, which is a generic name. `go` takes four arguments, one for each call to `getLine`. If we remove a call to `getLine`...

```haskell
getThreeLines :: IO String
getThreeLines = go 
    <$> getLine 
    <*> getLine 
    <*> getLine 
    where
        go line1 line2 line3 = line1 ++ line2 ++ line3
```

...then `go` only needs to take three arguments instead of four. The number of `<$>`'s and `<*>`'s equals the number of arguments to `go`. If they don't match, we get a compilation error.

If we only want the second and fourth lines, we could do the following:

```haskell
getLinesTwoAndFour :: IO String
getLinesTwoAndFour = go
    <$> getLine
    <*> getLine
    <*> getLine
    <*> getLine
    where
        go _ line2 _ line4 = line2 ++ line4
```

We can deliberately ignore the return values of the first and third calls to `getLine` once they get to the `go` function. However, there is a cleaner way to do the same thing:

```haskell
getLinesTwoAndFour :: IO String
getLinesTwoAndFour = go
    <$ getLine
    <*> getLine
    <* getLine
    <*> getLine
    where
        go line2 line4 = line2 ++ line4
```

If we remove the `>` preceding the call to `getLine`, then we perform the action, but toss the result. This works for both `<$>` and `<*>`.

Consider a different `Applicative` instance using `State`. We implement a basic stack. The implementation is not important, suffice to say that `push` and `pop` do exactly as expected:

```haskell
import Control.Monad.State

push :: a -> State [a] ()
push a = modify (a:)

pop :: State [a] a
pop = state $ \(x: xs) -> (x, xs)

doSomething :: State [Int] Int
doSomething = go
    <$> pop
    <* pop
    <*> pop
    <* push 15
    where go x y = x * y
```

`doSomething` will pop three numbers off the top of the stack, ignore the second number, then push `15` onto the stack, We can test this:

```haskell
ghci> runState doSomething [5, 2, 10]
(50,[15])
```

where `5` is the top of the stack and `10` is the bottom. `doSomething` pops `5`, `2` and `10` off the stack, and pushes `15` onto it, leaving our stack to be `[15]`. The return value is `50` (`5` * `10`), therefore the result of `runState` will be `(50, [15])`.

### Golden rules

We can now use some easy mnemonics for the basic use of `Applicative` if we are to write functions in the following way:

```haskell
applicativeAction = go
    <$ action1
    <*> action2
    <* action3
    <*> action4
    where
        go line2 line4 = doSomething line1 line2
```

1. Each applicative action must be individually prepended with one of either `<$>`, `<*>`, `<$`, or `<*`.
2. Precede the action with `<*` or `<$` if you want to perform the action, but ignore the result.
3. The first action must be preceded by `<$>` or `<$`, and not by `<*>` or `<*`.
4. The number of arguments to the `go` function (or whatever you want to call it) must match the number of `<$>` plus the number of `<*>`.

This is a simple and generic way to use the basic functionality of `Applicative`.
