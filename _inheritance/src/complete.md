# Complete Example

```rust
use ref_cast::RefCast;

struct ConcreteAnimal {
    height: f64,
    name: String,
}

impl ConcreteAnimal {
    // Don't implement this if the base class is abstract
    fn new(height: f64, name: String) -> Self {
        ConcreteAnimal { height, name }
    }
}

trait Animal: AsRef<ConcreteAnimal> + AsMut<ConcreteAnimal> {
    fn parent(&self) -> &dyn Animal;
    fn parent_mut(&mut self) -> &mut dyn Animal;

    fn grow(&mut self, new_height: f64) -> f64 {
        self.parent_mut().grow(new_height)
    }

    fn height(&self) -> f64 {
        self.parent().height()
    }

    fn name(&self) -> String {
        self.parent().name()
    }

    fn rename(&mut self, name: String) {
        self.parent_mut().rename(name)
    }

    fn speak(&self) -> String {
        self.parent().speak()
    }
}

#[derive(Clone, Copy, RefCast)]
#[repr(transparent)]
struct BaseAnimal<T>(T);
impl<T> BaseAnimal<T> {
    fn child(&self) -> &T {
        &self.0
    }

    fn child_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> AsRef<ConcreteAnimal> for BaseAnimal<T>
where
    T: Animal,
{
    fn as_ref(&self) -> &ConcreteAnimal {
        self.child().as_ref()
    }
}
impl<T> AsMut<ConcreteAnimal> for BaseAnimal<T>
where
    T: Animal,
{
    fn as_mut(&mut self) -> &mut ConcreteAnimal {
        self.child_mut().as_mut()
    }
}
impl<T> Animal for BaseAnimal<T>
where
    T: Animal,
{
    fn parent(&self) -> &dyn Animal {
        unimplemented!()
    }
    fn parent_mut(&mut self) -> &mut dyn Animal {
        unimplemented!()
    }

    fn grow(&mut self, _new_height: f64) -> f64 {
        unimplemented!()
    }

    fn height(&self) -> f64 {
        self.as_ref().height
    }
    
    fn name(&self) -> String {
        self.as_ref().name.clone()
    }

    fn rename(&mut self, name: String) {
        self.as_mut().name = name;
    }

    fn speak(&self) -> String {
        format!("Hello, my name is {}", self.child().name())
    }
}

impl AsRef<ConcreteAnimal> for ConcreteAnimal {
    fn as_ref(&self) -> &ConcreteAnimal {
        self
    }
}
impl AsMut<ConcreteAnimal> for ConcreteAnimal {
    fn as_mut(&mut self) -> &mut ConcreteAnimal {
        self
    }
}
impl Animal for ConcreteAnimal {
    fn parent(&self) -> &dyn Animal {
        BaseAnimal::ref_cast(self)
    }
    fn parent_mut(&mut self) -> &mut dyn Animal {
        BaseAnimal::ref_cast_mut(self)
    }
}

struct ConcreteDog {
    animal: ConcreteAnimal,
    nose_length: f64,
}
impl ConcreteDog {
    fn new(animal: ConcreteAnimal, nose_length: f64) -> Self {
        ConcreteDog { animal, nose_length, }
    }
}
impl AsRef<ConcreteDog> for ConcreteDog {
    fn as_ref(&self) -> &ConcreteDog {
        self
    }
}
impl AsMut<ConcreteDog> for ConcreteDog {
    fn as_mut(&mut self) -> &mut ConcreteDog {
        self
    }
}
impl AsRef<ConcreteAnimal> for ConcreteDog {
    fn as_ref(&self) -> &ConcreteAnimal {
        &self.animal
    }
}
impl AsMut<ConcreteAnimal> for ConcreteDog {
    fn as_mut(&mut self) -> &mut ConcreteAnimal {
        &mut self.animal
    }
}

trait Dog: Animal {
    fn bark(&self) -> String {
        unimplemented!()
    }
}
impl Dog for ConcreteDog {}
impl Animal for ConcreteDog {
    fn parent(&self) -> &dyn Animal {
        BaseDog::ref_cast(self)
    }
    fn parent_mut(&mut self) -> &mut dyn Animal {
        BaseDog::ref_cast_mut(self)
    }
}


#[derive(Clone, Copy, RefCast)]
#[repr(transparent)]
struct BaseDog<T>(T);
impl<T> AsRef<ConcreteAnimal> for BaseDog<T>
where
    T: Animal,
{
    fn as_ref(&self) -> &ConcreteAnimal {
        self.0.as_ref()
    }
}
impl<T> AsMut<ConcreteAnimal> for BaseDog<T>
where
    T: Animal,
{
    fn as_mut(&mut self) -> &mut ConcreteAnimal {
        self.0.as_mut()
    }
}
impl<T> Animal for BaseDog<T> where T: Animal {
    fn parent(&self) -> &dyn Animal {
        BaseAnimal::ref_cast(&self.0)
    }

    fn parent_mut(&mut self) -> &mut dyn Animal {
        BaseAnimal::ref_cast_mut(&mut self.0)
    }

    fn name(&self) -> String {
        format!("Mr. {} D. Dog", self.parent().name())
    }

    fn speak(&self) -> String {
        format!("{}, and I'm a dog", self.parent().speak())
    }
}

pub fn main() {
    let animal = ConcreteAnimal::new(180.0, "Alice".to_string());
    println!("{}", animal.speak());

    let dog = ConcreteDog::new(ConcreteAnimal::new(80.0, "Bob".to_string()), 30.0);
    println!("{}", dog.speak());

    // let dog = ConcreteDog {
    //     animal,
    //     nose_length: 10.0,
    // };
    // println!("{}", dog.speak());
}
```