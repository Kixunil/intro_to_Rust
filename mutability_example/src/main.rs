#![allow(unused)] // <-- Dear rustc, shut up, I know what I'm doing!

fn main() {
    let a = 42;
    // Not again!
    // Please, change it to something else...
    // Like this:
    //a = 47;

    // Why u no change? I want to **mut**ate it!
    let mut b = 42;
    b = 47;
    // Now it's fine. Let's borrow it.
    {
        let c = &b;

        // Is this possible?
        //*c = 24;

        // How it can be immutable?! I said it's mutable, see:
        //b = 24;

        // Of course, it wasn't returned yet... Wait, we can print it!
        println!("The value of b is: {}", b);
        println!("The value of c is: {}", c);

        // There's something fishy going on.

        // Let's request the right to mutate.
        //let d = &mut b;

        // Ok, return it then...
    }

    {
        let d = &mut b;
        // Can we change it now?
        *d = 24;

        println!("The value of d is: {}", d);
        // So, if d is refernce and we used dereference (*) operator, then original must've changed
        // too. Let's see:
        //println!("The value of b is: {}", b);

        // Just one mutable borrow, remember?
    }
    println!("The value of b is: {}", b);

    // Yeah, it works!
}

// You can find enumeration of what else we'll try out in enum_example.
