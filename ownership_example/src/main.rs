fn main() {
    let a = vec![2, 3, 3, 3, 3]; // vec! creates a vector - a resize-able heap-allocated array. Why square brackets you ask? Didn't I say that macros use obscure rules?
    //  ^
    //  |
    //  This owns the Vec now
    let b = a; // Vec moves from `a` to `b`.
    //  ^
    //  |
    //  This owns the Vec now
    //  `a` doesn'ŧ own the Vec now.

    // Uncomment the line bellow, if you wanna see compile fail.
    //println!("a is now: {:?}", a); // {:?} means show as debug instead of normal, user-facing
    
    // Try it again with i32
    let c = 42;

    let d = c; // Ok, another move...

    // Let's make compilation fail!
    println!("c is now: {:?}", c);

    // Whaat?! Why it works?
    //
    // i32 is copyable type, so it will copy itself instead of moving.
    // Many primitive types are copyable.
    
    // Let's borrow!
    let b_borrowed = &b;
    let d_borrowed = &d;

    // What will happen now?
    println!("b is now: {:?}", b);
    println!("b_borrowed is now: {:?}", b_borrowed);
    println!("d is now: {:?}", d);
    println!("d_borrowed is now: {:?}", d_borrowed);

    // Huh?
    //
    // Borrowing means creating a reference. As with AFK borrowing, the borrowed stuff must be
    // returned to the owner. It will be returned at the end of scope.
    
    { // <-- beginning of made up anonymous scope
        let b_borrowed_again = &b;
        println!("b_borrowed_again is now: {:?}", b_borrowed_again);
    } // <-- end of scope, borrow ends here.

    // Wanna see a compile fail? Try using returned borrows!
    //println!("b_borrowed_again is now: {:?}", b_borrowed_again);

    // There is never enough compile fails. Uncomment next block comment.
    /*
    let e; // Don't worry, this is completely fine.
           // Rust checks if we are programmers and not pigs, so there is uninitialized value is
           // never used.
    {
        let f = vec![1, 0, 1, 0, 1, 0]; // We create a Vec
        e = &f; // We borrow it
    } // <-- The owner of the Vec dies here before we had a chance to return it. Rustc complains.
    */

    println!();
    println!("The question is: \"What is the product of {:?}\"?", b);
}

// Constant variables are no fun, let's mutate them in `mutability_example`!
