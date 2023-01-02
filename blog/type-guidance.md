# Type guidance on APIs using PhantomData

When writing APIs it's easy for users to make misuses of methods defined within a struct. There are cases when you might want to restrict the methods available downstream depending on the state of an instance.


For example, a structure `Car` can have two engine stages, `On` and `Off`. When the car engine is off the user can start the engine and can't increment it's speed. The user must not be able to start the engine if it is already on. The same goes when the car is on, the user can turn the engine off once and increment it's speed as many times as desired.


It might be tempting to define a member like an enum or a boolean and use it as flag, though, there is a way of implementing something alike with no impact on the final binary, that is zero-cost, plus avoid having to make lots of checks to verify the instance state.


Rust provides `std::marker::PhantomData` which has a variety of uses. In this writeup, we will use it to implement specific methods for each stage of a structure by using unit types as stage markers.

This is an exercise of the section *"Type System Guidance"* at Chapter 3 of Jon Gjengset's *"Rust for Rustaceans"* excellent book.

First, bring into scope `std::marker::PhantomData`.

```rust
use std::marker::PhantomData;
```

Then, define the unit types that will serve as markers.

```rust
struct Off;
struct On;
```

Now declare `Car` with a stage member that contains `PhantomData`, where the generic argument `T` can be either `On` or `Off`.

```rust
struct Car<T> {
    speed: i32,
    stage: PhantomData<T>,
}
```

You can also opt to enforce a default type `T` with `T = Off`.
Then, implement the Default trait to let the users to create new instances of `Car`. 
This instances will all start being of type `Car<Off>`.

```rust
impl Default for Car<Off> {
    fn default() -> Car<Off> {
        Self {
            speed: 0,
            stage: PhantomData,
        }
    }
}
```

Following, implement all methods that can be called when the instance 'stage' is set to `Off`. 
In this case, just `start_engine`. 
Note no un-necessary checks are made to verify if the engine is already off, 
it's 'implicitly' handled when we match `T=Off` by using `Car<Off>`, thus verified at Rust's type system level. 

```rust
impl Car<Off> {
    fn start_engine(self) -> Car<On> {
        println!("The car is now on");
        Car {
            speed: 0,
            stage: PhantomData,
        }
    }
}
```

Also implement methods available when the stage has type `T=On`.
Again, no checks are made, leading to cleaner code.

```rust
impl Car<On> {
    fn stop_engine(self) -> Car<Off> {
        println!("The car is now off");
        Car {
            speed: 0,
            stage: PhantomData,
        }
    }

    fn increase_speed(&mut self) {
        println!("The car is now increasing its speed");
        self.speed += 10;
    }
    fn decrease_speed(&mut self) {
        println!("The car is now decreasing its speed");
        self.speed -= 10;
    }
}
```

There are also cases when actions can be performed with a car no matter its specific engine state,
like checking it's speed, get it's color or the name of the model and year.
Define 'global' methods by matching `T` to any `Stage` type.

```rust
impl<Stage> Car<Stage> {
    fn show_speed(&self) {
        println!("The car speed is {}", self.speed);
    }

    fn clean(&self) {
        println!("The car is now being washed up!");
    }
}
```

Aside of the cleaner implementation blocks with stage logic handled from the type system, an
API that is impossible for they to misuse is provided. For example, the following block will not compile:

```rust
fn main() {
    let car = Car::default().stop_engine();
}
```

With the compilation error: 

```rust
error[E0599]: no method named `stop_engine` found for struct `Car<Off>` in the current scope
--> src/main.rs:60:30
    |
 6  | struct Car<T> {
    | ------------- method `stop_engine` not found for this
 ...
 60 |     let car = Car::default().stop_engine();
    |                              ^^^^^^^^^^^ help: there is an associated function with a similar name: `start_engine`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `phantom` due to previous error
```

The error originates because `Car::default()` returns an instance of type `Car<Off>`, which does not have any implement block where `stop_engine` is defined. 
`stop_engine` is only available when the state of the engine is `On`, hence when we have an instance of `Car<On>`.
This fact makes it impossible for any user to call methods in a wrong order, in the wrong time.


> "Zero-sized type used to mark things that “act like” they own a T."
>
> "Adding a PhantomData field to your type tells the compiler that your type acts as though it stores a value of type T, even though it doesn't really. This information is used when computing certain safety properties."
> 
> (Struct std::marker::PhantomData, doc.rust-lang.org)

Thus, the unit types used on the implementation do not impact the final binary, making it a zero-cost abstraction for better designing APIs. 

## Sources
1. Gjengset, J. (2022) *"Rust for Rustaceans: Idiomatic Programming for Experienced Developers"*. No Starch Press.
2. Doc.Rust-Lang. *"Struct std::marker::PhantomData"*. Retrieved from [**doc.rust-lang.org**](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) on July 6, 2022.
