fn main() {
    let mut language = "Rust"; // define a variable  //mut is used to update the value for later use
    println!("Language: {}", language); // print the variable
    language = "Java"; // update the variable
    println!("Language: {}", language); // print the updated value of variable
    //output: 
    // Language: Rust
    // Language: Java

    let (course,category) =("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
    // This is a beginner course in Rust.

    //SCope of a variable
    let outer_variable = 112;
    let inner_variable = 213;
    { // start of code block
        println!("block variable inner: {}", inner_variable);
        println!("block variable outer: {}", outer_variable);
    } // end of code block
        println!("inner variable: {}", inner_variable);    //inner_variable can be accessed here also
    //if we assign a value insie the block then it cannot be accessed outside

    
      
}