fn main(){
    println!("Hi");
    //Single Placeholder
    println!("{} is a programming language","Rust"); //using strings
    //output: Rust is a programming language
    println!("{} is a number",1); //using numbers
    //output: 1 is a number

    //Multiple Placeholder
    println!("{} is an {} society", "Vishnu", "Educational");
    //output: Vishnu is an Educational society

    //Positional Arguments
    println!("Improve your coding skills from {0} courses.  {0} courses are very {1}", "Educative", "interactive");
    //output: Improve your coding skills from Educative courses.  Educative courses are very interactive

    //Named Arguments
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
    //output: Educative provides interactive courses

    //Placeholder Traits
    println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}", 10, 10, 10);
    //output:
    // Number : 10
    // Binary:1010 Hexadecimal:a Octal:12

    //Basic Math
    println!("{} + {} = {}",10, 10, 10 + 10);
    //output: 10 + 10 = 20

    
    //Placeholder for a Debug Trait#
    println!("{:?}", ("This is a Rust Course", 101));
    //output: ("This is a Rust Course", 101)

    

}