---
title: Implement inheritance in Rust
category: notes
tags:
- here
- are
- some
- tags.
---

## Implement Inheritance in Rust

Alternative title: the inheritance (anti)pattern in Rust.

![A description of the image]({{ '/img/some_image.jpg' | relative_url}})

### Summary

- Point 1
- Point 2

# Rust does not have inheritance

...at least not natively. This is a problem for a lot of programmers who are migrating from other languages and who want to map their existing ideas or programs one-to-one into Rust. I will go through some of the ways in which existing inheritance patterns can be mapped into Rust. I will go through several use cases of inheritance and how that code should be written in Rust, and finish with a pattern that emulates true inheritance in other languages. The non-rust examples that I will provide are all written in pseudocode. Mind you, this article comes with a disclaimer: some of the code that will be presented is extremely unidiomatic rust, or just plain bad. Some of these patterns should be used with extreme caution and not for beginners.

## Heterogeneous Collections

Let's start off with an easy one.

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

This is a common use case. Inheritance allows multiple different types to be put into the same collection. In Rust we would do something similar with Traits:

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

### Variation: use an `enum`

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

## Concrete inheritance

Example use cases

- "I want to implement a method on all types that have the same fields"
- "My inheritance has no abstract or virtual methods"

```C#
using System;
using System.Collections.Generic;
			
public class Program {
	public class Location {
		public double x;
		public double y;
		
		public Location(double x, double y) {
			this.x = x;
			this.y = y;
		}
	}

	public class City: Location {
		public string name;
		
		public City(double x, double y): base(x, y) {}
	}

	public class Mountain: Location {
		public double height;
		
		public Mountain(double x, double y): base(x, y) {}
	}
	
	static double Distance(Location l1, Location l2) {
		var dx = l1.x - l2.x;
		var dy = l1.y - l2.y;
		return Math.Sqrt(dx * dx + dy * dy);
	}
		
	public static void Main() {
		var mountain = new Mountain(0, 0);
		var city = new City(3, 4);
		
		Console.WriteLine("The distance from the city to the mountain is {0}.", Distance(mountain, city));
	}
}
```

Output:

```
The distance from the city to the mountain is 5.
```

For this, you should use the `AsRef` and `AsMut` and `Into` traits. `AsRef` will allow you to access the base class immutably, `AsMut` will allow you to use the base class mutably, and `Into` will allow you to consume the parent class and return a concrete base class. `Into` is automatically implemented if a struct implements the `From` trait, so `From` should always be preferred:

```rust
struct Location {
    x: f64,
    y: f64,
}

struct City(Location);
impl AsRef<Location> for City {
    fn as_ref(&self) -> &Location {
        &self.0
    }
}
impl AsMut<Location> for City {
    fn as_mut(&mut self) -> &mut Location {
        &mut self.0
    }
}
impl From<City> for Location {
    fn from(city: City) -> Self {
        city.0
    }
}

struct Mountain(Location);
impl AsRef<Location> for Mountain {
    // ...boilerplate
}
impl AsMut<Location> for Mountain {
    // ...boilerplate
}
impl From<Mountain> for Location {
    // ...boilerplate
}

fn distance(l1: &Location, l2: &Location) -> f64 {
    let dx = l1.x - l2.x;
    let dy = l1.y - l2.y;
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let mountain= Mountain(Location { x: 0.0, y: 0.0 });
    let city = City(Location { x: 3.0, y: 4.0 });
    println!("The distance from the city to the mountain is {}.", distance(city.as_ref(), mountain.as_ref()))
}
```

The reason that the boilerplate is necessary is that we need to tell Rust which base class we are going to use. For example, consider the following structure:

```rust
struct Person {
    location: Location,
    home: Location,
    work: Location,
}
```

Rust doesn't know which of the locations to forward the methods to. The boilerplate would specify that the person's location is found in the `location` field, and not `home` or `work`.

If you still hate writing boilerplate, you can use the `derive_more` crate to derive `AsRef`, `AsMut` and `From` automatically. However, this is beyond the scope of the article.

### Variation: Emulating Row Polymorphism

If you're crazy enough, you could create use a trait for each field you want to access and then white functions that are polymorphic over individual fields:

```rust
trait Has<F, V> {
    fn get(&self, field: F) -> &V;
    fn get_mut(&mut self, field: F) -> &mut V;
}

struct X;
struct Y;

struct City(f64, f64);
impl Has<X, f64> for City {
    fn get(&self, _field: X) -> &f64 {
        &self.0
    }
    fn get_mut(&mut self, _field: X) -> &mut f64 {
        &mut self.0
    }
}
impl Has<Y, f64> for City {
    fn get(&self, _field: Y) -> &f64 {
        &self.1
    }
    fn get_mut(&mut self, _field: Y) -> &mut f64 {
        &mut self.1
    }
}

struct Mountain(f64, f64);
impl Has<X, f64> for Mountain {
    // ...boilerplate
}
impl Has<Y, f64> for Mountain {
    // ...boilerplate
}

trait Location: Has<X, f64> + Has<Y, f64> {}
impl<T> Location for T where T: Has<X, f64> + Has<Y, f64> {}

fn distance(l1: impl Location, l2: impl Location) -> f64 {
    let dx = l1.get(X) - l2.get(X);
    let dy = l1.get(Y) - l2.get(Y);
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    let mountain= Mountain(0.0, 0.0);
    let city = City(3.0, 4.0);
    println!("The distance from the city to the mountain is {}.", distance(city, mountain))
}
```

This is highly unidiomatic rust, and you would almost certainly want to write macros to reduce boilerplate. Since the type of the field is generic, you could pass data into the field type. A potential usage for this would be to use the `Has` trait with `usize` as the field type to index into a structure.

## Multiple, single layer inheritance

Whilst the static method above works for many layers "deep", when we introduce virtual methods, things become a little more complex.

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
