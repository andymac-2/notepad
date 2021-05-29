# Heterogeneous Collections

One feature of inheritance is that different types that share a common base class can be stored in the same collection together.

Example use cases:

- "I want to but a bunch of different types in the same collection together"
- "I won't know the type of my class until runtime"

```C#
using System;
using System.Collections.Generic;
			
public class Program {
	public abstract class Fruit {
		public abstract void Describe();
	}

	public class Apple: Fruit {
		public override void Describe() {
			Console.WriteLine("I'm an Apple");
		}
	}

	public class Banana: Fruit {
		public override void Describe() {
			Console.WriteLine("I'm a Banana");
		}
	}
	
	static void DescribeFruitBowl(List<Fruit> bowl) {
		foreach (var fruit in bowl) {
			fruit.Describe();
		}
	}
		
	public static void Main() {
		var fruits = new List<Fruit> { new Apple(), new Banana() };
		DescribeFruitBowl(fruits);
	}
}
```

Output:

```
I'm an Apple
I'm a Banana
```

This is a common use case. Inheritance allows multiple different types to be put into the same collection. In Rust we would do something similar with traits:

```rust
trait Fruit {
    fn describe(&self);
}

struct Apple;
impl Fruit for Apple {
    fn describe(&self) {
        println!("I'm an Apple")
    }
}

struct Banana;
impl Fruit for Banana {
    fn describe(&self) {
        println!("I'm a Banana")
    }
}
```

The naive solution doesn't compile since it has mismatched types:

```rust
fn describe_fruit_bowl(bowl: Vec<impl Fruit>) {
    for fruit in bowl {
        fruit.describe()
    }
}

fn main() {
    describe_fruit_bowl(vec![Apple, Banana]);
}
```

Whilst `describe_fruit_bowl` will allow you to use any one fruit, it restricts the bowl to only having the same type of fruit at all times. A fruit bowl full of bananas is fine, but a fruit bowl with an apples and bananas mixed together is a problem. Not mixing fruit is often what you want, so Rust makes you choose explicitly what behavior you're going to use.

To mix the fruit in the bowl, we ned to use `dyn Fruit`, which tells Rust to use dynamic dispatch for each fruit. We also need to add a layer of indirection: the size of `dyn Fruit` is unknown because it could be any type of fruit. Therefore, we put each fruit in a `Box`:

```rust
fn describe_fruit_bowl(bowl: Vec<Box<dyn Fruit>>) {
    for fruit in bowl {
        fruit.describe()
    }
}

fn main() {
    describe_fruit_bowl(vec![Box::new(Apple), Box::new(Banana)]);
}
```

## Variation: use an `enum`

Example use cases:

- "I know ahead of time that I will only have bananas or apples in my fruit bowl"
- "The user of my library can't implement new kinds of fruits"
- "I'm going to use the visitor pattern to add functionality to a class without modifying the class itself"

If there are a limited number of types of fruit, you can use an `enum`:

```rust
enum Fruit {
    Apple,
    Banana,
}
impl Fruit {
    fn describe(&self) {
        match self {
            Fruit::Apple => println!("I'm an Apple"),
            Fruit::Banana => println!("I'm a Banana"),
        }
    }
}

fn describe_fruit_bowl(bowl: Vec<Fruit>) {
    for fruit in bowl {
        fruit.describe()
    }
}

fn main() {
    describe_fruit_bowl(vec![Fruit::Apple, Fruit::Banana]);
}
```

As a side note, a function that takes the `Fruit` enum as an argument and performs a `match` on it, is essentially doing the same job as a visitor.