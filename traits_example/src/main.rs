#![allow(dead_code)]
// I'm running out of jokes, so I guess I'll need to run OOJ killer.

// This is a generic function
fn id<T>(value: T) -> T { // Arrow (->) denotes return type.
    // This is very ugly Rust code. My eyes hurt, but I wanted you to see it and understand.
    // I'll show you nicer code bellow.
    return value;
}

// This is exactly same function but it's not ugly
fn id2<T>(value: T) -> T {
    // Last expression is always return value.
    // This is much nicer.
    value
}

// These functions are boring, let's do something more interesting.
// To do it we'll need to learn traits.

// This is a trait
// It is basically a specification of an interface.
// Like interface in Go, or protocol in Objective-C(++)/Swift
// Haskell calls it "typeclass"
trait Yell {
    // This is a trait method.
    // &self means that it takes immutable borrow of Self.
    // It's a abbreviation of self: &Self.
    // Self is type that implements this trait.
    fn yell(&self);
}

// Let's use the trait in generics
// T: Yell means that T must implement Yell trait.
fn yell_twice<T: Yell>(x: &T) {
    // This is how we call yell method.
    x.yell();
    x.yell();
}

// OK, so how do we implement a trait?
impl Yell for i32 {
    fn yell(&self) {
        println!("I'M AN I32 WITH VALUE OF {}", *self);
    }
}

// Let's see if it works
fn main() {
    yell_twice(&42);
}

// Ok, now you'r ready to read unsafe_notes.md
