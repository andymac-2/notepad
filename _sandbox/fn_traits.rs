struct MyStruct {
    text: &'static str,
    number: u32,
}

impl MyStruct {
    fn new (text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text: text,
            number: number,
        }
    }

    // We have to specify that 'self' is an argument.
    fn get_number (&self) -> u32 {
        self.number
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_number (&mut self) {
        self.number += 1;
    }
    // There are three different types of 'self'
    fn destructor (self) {
        println!("Destructing {}", self.text);
    }
}

fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {}
fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}

fn main () {
    // NO CONTEXT
    {
        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        let closure1 = |x: &MyStruct| x.get_number() + 3;

        // compiles successfully.
        is_fn(closure1); 
        is_Fn(&closure1);
        is_FnMut(&closure1);
        is_FnOnce(&closure1);

        assert_eq!(closure1(&obj1), 18);
        assert_eq!(closure1(&obj2), 13);

        fn func1 (x: &MyStruct) -> u32 {
            x.get_number() + 3
        }
        assert_eq!(func1(&obj1), 18);
        assert_eq!(func1(&obj2), 13);
    }

    // IMMUTABLE CONTEXT
    {
        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        // obj1 is borrowed by the closure immutably.
        let closure2 = |x: &MyStruct| x.get_number() + obj1.get_number();
        assert_eq!(closure2(&obj2), 25);

        // We can borrow obj1 again immutably...
        assert_eq!(obj1.get_number(), 15);

        // But we can't borrow it mutably.
        // obj1.inc_number(); // ERROR

        // Does not compile:
        // is_fn(closure2);

        // Compiles successfully:
        is_Fn(&closure2);
        is_FnMut(&closure2);
        is_FnOnce(&closure2);
    }

    {
        struct Context<'a>(&'a MyStruct);

        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        let ctx = Context(&obj1);

        fn func2 (context: &Context, x: &MyStruct) -> u32 {
            x.get_number() + context.0.get_number()
        }
        assert_eq!(func2(&ctx, &obj2), 25);

        // We can borrow obj1 again immutably...
        assert_eq!(obj1.get_number(), 15);

        // But we can't borrow it mutably.
        // obj1.inc_number(); // ERROR
    }

    // MUTABLE CONTEXT
    {
        let mut obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        // obj1 is borrowed by the closure mutably.
        let mut closure3 = |x: &MyStruct| {
            obj1.inc_number();
            x.get_number() + obj1.get_number()
        };
        assert_eq!(closure3(&obj2), 26);
        assert_eq!(closure3(&obj2), 27);
        assert_eq!(closure3(&obj2), 28);

        // We can't borrow obj1 mutably or immutably
        // assert_eq!(obj1.get_number(), 18); // ERROR
        // obj1.inc_number(); // ERROR

        // Does not compile:
        // is_fn(closure3);
        // is_Fn(&closure3);

        // Compiles successfully:
        is_FnMut(&closure3);
        is_FnOnce(&closure3);
    }

    {
        struct Context<'a>(&'a mut MyStruct);

        let mut obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        let mut ctx = Context(&mut obj1);

        // obj1 is borrowed by the closure mutably.
        fn func3 (context: &mut Context, x: &MyStruct) -> u32 {
            context.0.inc_number();
            x.get_number() + context.0.get_number()
        };
    }

    // OWNED CONTEXT
    {
        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        // obj1 is owned by the closure
        let closure4 = |x: &MyStruct| {
            obj1.destructor();
            x.get_number()
        };
        // Does not compile:
        // is_fn(closure4);
        // is_Fn(&closure4);
        // is_FnMut(&closure4);
        
        // Compiles successfully:
        is_FnOnce(&closure4);

        assert_eq!(closure4(&obj2), 10);
        
        // We can't call closure4 twice...
        // assert_eq!(closure4(&obj2), 10);             //ERROR

        // We can't borrow obj1 mutably or immutably
        // assert_eq!(obj1.get_number(), 15);           // ERROR
        // obj1.inc_number();                           // ERROR
    }

    {
        struct Context(MyStruct);

        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        let ctx = Context(obj1);

        // obj1 is owned by the closure
        fn func4 (context: Context, x: &MyStruct) -> u32 {
            context.0.destructor();
            x.get_number()
        };
        assert_eq!(func4(ctx, &obj2), 10);
        
        // We can't call closure4 twice...
        // assert_eq!(func4(ctx, &obj2), 10);             //ERROR

        // We can't borrow obj1 mutably or immutably
        // assert_eq!(obj1.get_number(), 15);           // ERROR
        // obj1.inc_number();                           // ERROR
    }
}