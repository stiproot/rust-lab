# &
The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.

Like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

# Result
Result is an enumeration, often called an enum, which is a type that can be in one of multiple possible states. We call each possible state a variant.
Result’s variants are Ok and Err
An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect

# Match expression
A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.

# Shadowing
Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.

# Scaler types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.