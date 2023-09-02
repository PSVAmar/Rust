The println is said to be as a Macro and usually a macro is written before a ! mark and () 
The rust is a meta programming

A macro is an expression that has
an exclamation mark (!) before the parenthesis () , i.e.,
                
                 macro_name ! ( );


What are macros used for?

They are used in metaprogramming, i.e., code that writes code. They look like functions in other system programming languages like C and C++, but instead of generating a function call like functions, they are expanded into source code that gets compiled with the rest of the program. In this way, they provide more run-time features.

Rust provides us with some built-in macros, like the println!() above, and users can define their own macros as well.



Single Placeholder
A single placeholder is used when it is required to print a single value.

fn main() {
    println!("Number: {}", 1);
}

Multiple Placeholders
We can use multiple placeholders within the println!() macro.

The number of placeholders should be equal to the number of values to be printed.

fn main() {
    println!("{} is a {} company", "Educative", "Software");
}


Named Arguments
A placeholder can take a named argument and assign it a value explicitly. It does this by specifying a name within the placeholder and assign that name the value to be inserted in the string.

fn main() {
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
}


Positional Arguments
Positional arguments specify the positions of the values in a sentence.

Each value is assigned a number based on the order of occurrence. The first value is assigned 0, the next is assigned 1, and so on and so forth. The placeholder takes an integer positive number (greater than or equal to 0) which indicates the value to be inserted in the placeholder is to be picked from the list of values in a given order.


Placeholder Traits
If we want to convert the value to binary, hexadecimal, or octal write:
{:b},{:x},{:o}

In the placeholder for binary, hexadecimal, or octal respectively and in the value specify the number.
You can use all three of them or one of them in a single expression.


Basic Math
We can perform basic math and the placeholder gets replaced with the result.

fn main() {
    println!("{} + {} = {}",10, 10, 10 + 10);
}


Placeholder for a Debug Trait
It is possible to display multiple values using a single placeholder with the help of the debug trait (a colon followed by a question mark {:?}).

This prevents having to write placeholders for each value.

You can use a debug trait and write as many values as desired within the parentheses.

fn main() {
    println!("{:?}", ("This is a Rust Course", 101));
}