fn main() {
    let the_answer = 42;
    println!("The answer is: {}", the_answer);
}

// So, C programmers be like: "Why is integer called 'let'?"
// Javascript programmers: "Huh, dynamic typing. I know that!"
// Haskell programmers: "Yeah, full-program inference!"
//
// You are all wrong.
//
// Rust uses function-local inference to avoid writing boilerplate. Function signatures are still
// explicit because they serve as a documentation, a backward-compatibility promise in libraries
// they make type compilation faster and provide more precise error messages.
//
// Rust doesn't allow variables to change types. They can be shadowed, though! That means if you
// ever introduce new variable with same name, you lose access to the previous one. The new one can
// have different type.
//
// So what does `println!("The answer is: {}", the_answer)` mean?
// The curly braces inside zeroth macro argument tell Rust that a displayable value should go
// there. The first argument is our variable. The variable is actually of type i32 and that type is
// displayable.
//
// Ok, enough talking, let's compute the Question. See ownership_example.
