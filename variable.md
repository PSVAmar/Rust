Create a Variable 
To create a variable, use the let binding followed by the variable name.

What is binding?

Rust refers to declarations as bindings as they bind a name at the time of creation. let is a kind of declaration statement.

Naming Convention: By convention, you would write a variable name in a snake_case i.e.,

All letters should be lower case.
All words should be separated using an underscore ( _ ).


Initialize a Variable 
A variable can be initialized by assigning a value to it when it is declared. The value is said to be bound to that variable.

let language = "rust";
//let is an identifier
//language is a variable name
//rust is a variable value

fn main() {
    let language = "Rust"; // define a variable
    println!("Language: {}", language); // print the variable
}



Assigning Multiple Variables 
It is possible to assign multiple variables in one statement.

fn main() {
    let (course,category) =("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
}

Scope of a variable
The scope of a variable refers to the visibility of a variable, or , which parts of a program can access that variable.

It all depends on where this variable is being declared. If it is declared inside any curly braces {}, i.e., a block of code, its scope is restricted within the braces, otherwise the scope is global.

Types of Variables
There are two types of variables in terms of scope.

Local Variable
A variable that is within a block of code, { }, that cannot be accessed outside that block is a local variable. After the closing curly brace, } , the variable is freed and memory for the variable is deallocated.

Global Variable
The variables that are declared outside any block of code and can be accessed within any subsequent blocks are known as global variables.

The variables declared using the const keyword can be declared in local as well as global scope. You’ll study constant variables in the next chapter.
      
     
