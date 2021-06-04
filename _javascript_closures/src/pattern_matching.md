# Pattern matching

Pattern matching can be implemented by using a function that selects the right match arm for us. For example, say we have a type called `UnitShape` which can either be a unit circle, a unit square, or a line. Ideally we would want to be able to be able to calculate the area using pattern matching that looks something like this:

```typescript
// typescript
const area = (shape: UnitShape) => match(shape, {
    UnitCircle: () => Math.PI,
    UnitSquare: () => 1,
    UnitLine: () => 0
});

console.log(area(UnitCircle)); // we want this to print 3.14...
console.log(area(UnitSquare)); // we want this to print 1
console.log(area(UnitLine)); // we want this to print 0
```

We can implement this by creating some helper functions that choose the right alternative to call from the options given in the `match` function:

```typescript
// typescript
const UnitCircle: UnitShape = ({ UnitCircle }) => UnitCircle();
const UnitSquare: UnitShape = ({ UnitSquare }) => UnitSquare();
const UnitLine: UnitShape = ({ UnitLine }) => UnitLine();
```

In that case, the final piece of the jigsaw puzzle is to implement the `match` function:

```javascript
// javascript
const match = (obj, visitor) => obj(visitor);
```

```typescript
// typescript
const match = <V, R>(obj: (visitor: V) => R, visitor: V) => obj(visitor);
```

For reference, here is the type definition for `UnitShape`:

```typescript
// typescript
interface UnitShape {
    <R>(visitor: {
        UnitCircle: () => R,
        UnitSquare: () => R,
        UnitLine: () => R
    }): R
}
```

#### Adding parameters

Circles have a radius, rectangles have a height and width, and lines have a length. It would be nice to have parameters in the match arms that look something like this:

```typescript
// typescript
const area = (shape: Shape): number => match(shape, {
    Circle: (radius: number) => Math.PI * radius * radius,
    Rectangle: (height: number, width: number) => height * width,
    Line: (_length: number) => 0
});

console.log(area(Circle(3)));           // prints "28.27..."
console.log(area(Rectangle(10, 4)));    // prints "40"
console.log(area(Line(20)));            // prints "0"
```

In order to create a shape with parameters, we can use a closure. A `Shape` is a function that chooses the right match arm to take, so it just needs to close over it's parameters:

```typescript
const Circle = (radius: number): Shape => ({ Circle }) => Circle(radius);
const Rectangle = (height: number, width: number): Shape => ({ Rectangle }) => Rectangle(height, width);
const Line = (length: number): Shape => ({ Line }) => Line(length);
```

The `match` function is the same one as above. For completeness, the type for shape is given below:

```typescript
interface Shape {
    <R>(visitor: {
        Circle: (radius: number) => R,
        Rectangle: (height: number, width: number) => R,
        Line: (length: number) => R
    }): R
}
```

#### Variations

One of the more simple variations is to split the interface for a `Shape` into two parts. This may make it easier to write types for your functions:

```typescript
interface Shape {
    <R>(method: ShapeMethod<R>): R
}

interface ShapeMethod<R> {
    Circle: (radius: number) => R,
    Rectangle: (height: number, width: number) => R,
    Line: (length: number) => R
}
```

Typescript struggles a bit with the level of complexity we have given it so we have to provide it with argument types in the `match` function. If we don't want to do that, we can just call the shape as a function directly and skip `match` altogether:

```typescript
// typescript
const areaRaw = (shape: Shape): number => shape({
    Circle: (radius) => Math.PI * radius * radius,
    Rectangle: (height, width) => height * width,
    Line: (_length) => 0
});
```

If we wanted to simplify things even further, we could use the method object directly instead of making it a function. This allows us to use `shape(method)` syntax instead of `method(shape)`:

```typescript
const areaMethod: ShapeMethod<number> = {
    Circle: (radius) => Math.PI * radius * radius,
    Rectangle: (height, width) => height * width,
    Line: (_length) => 0
};

const circle = Circle(3);
console.log(circle(areaMethod)); // prints "28.27..."
```

In Object orientated languages, it is common to see the object first and the method name second. For example, instead of `area(shape)` you would have `shape.area()`. Using the `shape(area)` syntax may be more familiar to some people.

### Objects

Javascript objects can be used through property access, or destructuring. We'll use a point object with an `x` and `y` coordinate as an example:

Plain objects can be represented by functions that yield their parameters when called. For example say we have a plain object that represents a point:

```javascript
// before
const point = { x: 5, y: 4 };

console.log(point.x);   // prints 5
console.log(point.y);   // prints 4

const { x, y } = point;
console.log(x);         // prints 5
console.log(y);         // prints 4
```

Using closures only, we might want the syntax to look like this:

```javascript
// after
const point = Point(5, 4);

console.log(point(x));   // prints 5
console.log(point(y));   // prints 4

point((x, y) => {
    console.log(x);     // prints 5
    console.log(y);     // prints 4
});
```

As you can see, we have property access, and destructuring with closures only. In order to achieve this, we need a helper interface that takes an `x` and `y` coordinate and returns a value:

```typescript
interface PointMethod<R> {
    (x: number, y: number): R
}
```

With this interface, now we can define two helper functions that return `x` and `y` respectively:

```typescript
const x: PointMethod<number> = (x, _) => x;
const y: PointMethod<number> = (_, y) => y;
```

The final step is to create a `Point` "object" which calls a `PointMethod` with the correct arguments:

```typescript
const Point = (x: number, y: number): Point => (method) => method(x, y);
```

This is similar to the pattern matching example above, but there is only one variant to choose from. There are different kinds of `Shapes`, but only one kind of `Point`. For a `Shape` we had a `ShapeMethod` object that had one function for each kind of `Shape`. For a `Point` we would need an object with only one function. In that case, we may as well use a function directly.

For completeness the type of `Point` is as follows:

```typescript
interface Point {
    <R>(method: PointMethod<R>): R
}
```



$$ math formula $$

Copy from the notepad blog off github pages to medium/wordpress from chrome, it formats better (bolds code blocks).

---

This post was originally posted on [Andrew's Notepad]({{ '/' | relative_url}})
