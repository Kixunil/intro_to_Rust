#![allow(unused)]

// This is an enum.
enum RemainingTopics {
    // This is a variant called "Enums"
    Enums(u32),
    Matches { on_fire: bool, on_steroids: bool },
    Generics,
    Traits,
    UnsafeSuperpowers,
}

fn main() {
    // If you wonder how big the enum is, from now now you won't.
    println!("Size of RemainingTopics is: {}", std::mem::size_of::<RemainingTopics>());
    // This is how we create values of enum types.
    let enums = RemainingTopics::Enums(42);
    let matches = RemainingTopics::Matches { on_fire: false, on_steroids: true };
    let generics = RemainingTopics::Generics;
    let traits = RemainingTopics::Traits;

    show_me(enums);
    show_me(matches);
    show_me(generics);
    show_me(traits);
}

fn show_me(topic: RemainingTopics) {
    // This is how we use (unwrap) values of enum types.
    match topic {
        // Wanna see compile fail? Comment-out any case in this match!
        RemainingTopics::Enums(n) => println!("We will see {} enums.", n),
        RemainingTopics::Matches { on_fire: fire, on_steroids } => println!("Matches are on fire: {}, on steroids: {}", fire, on_steroids),
        RemainingTopics::Generics => println!("Generics are in <>"),
        RemainingTopics::Traits => println!("Treat them same way!"),
        RemainingTopics::UnsafeSuperpowers => println!("Unsafe stuff is not safe, so we will skip learning it..."),
    }
}

// Enough simple things, let's see really powerful stuff! The traits_example is waiting just for you!
