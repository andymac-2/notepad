# Inheritance

So why is inheritance used in programming at all? If we were to take a particularly bare language that had nothing resembling object orientation, would inheritance provide any substantial benefit? And if it does provide some benefit, is the benefit large enough to accept it as a first class feature into a language?

In it's most basic form, inheritance allows for some code reuse between classes. In this example, the `describe` method is shared between a book and an id badge:

```javascript
class Id {
    constructor(id, name) {
        this.id = id;
        this.name = name;
    }
    describe() {
        console.log(`${this.name} has the id ${this.id}`);
    }
}

class Book extends Id {}
class Badge extends Id {}

const book = new Book('1234', 'Moby Dick');
const badge = new Badge('5678', 'Andrew');

book.describe();
badge.describe();
```

Consider a language that has no object orientated features, but has structs and functions. The most commonly discussed alternative to inheritance is composition: instead of a subclass inheriting from a super class, the super class is a member of the subclass. Any function written for the super class can still be used on the subclass by accessing the field:

```javascript
class Book {
    id: Id;

    constructor(id: string, name: string) {
        this.id = new Id(id, name);
    }
}

class Badge {
    id: Id;

    constructor(id: string, name: string) {
        this.id = new Id(id, name);
    }
}

const book = new Book('1234', 'Moby Dick');
const badge = new Badge('5678', 'Andrew');

book.id.describe();
badge.id.describe();
```

In any scenario, the most that inheritance offers over composition in this area is some syntactic sugar. This is not the kind of groundbreaking idea that languages are designed around.

There is however, an additional benefit that would be worth designing your language around. I argue what gives inheritance real power is the ability to override methods and perform dynamic dispatch. Inheritance provides us with the ability to perform the operations of the super class, without knowing what kind of base class it is.


The most simple and obvious task that can be performed with inheritance is to share members. Say you had a lot of different data types with some general fields such as `name` and `id`, and would like to write a function that can apply to any of these