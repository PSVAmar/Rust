What are Tuples?
Tuples are heterogeneous sequences of elements, meaning, each element in a tuple can have a different data type. Just like arrays, tuples are of a fixed length.

Define a Tuple
A tuple can be defined by writing let followed by the name of the tuple and then enclosing the values within the parenthesis.

Syntax 1
The syntax below defines a tuple without specifying the type. However, the compiler can infer the type.

let tuple_name = (1,"23",'c');

Syntax 2
The syntax below defines a tuple by specifying the type.

let tuple_name:(&str,char,i32)=("value",'c',1);


Access the Value of the Tuple
Unlike array which uses [] for accessing an element, the value of the tuple can be accessed using the dot operator (.).
    tuplename.indexvalue
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let person_data = ("Alex", 48, "35kg", "6ft");
    let (w, x, y, z) = person_data;


How to Make a Tuple Mutable?
Just like a variable becomes mutable by adding the mut keyword after let, the same goes for a tuple.

fn main() {
    //define a tuple
    let mut person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
    //modify the value at index 0
    person_data.0 = "John";
    //print the modified value
    println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
}

Print the Tuple
The whole tuple can be traversed using the debug trait.

fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);
}

Output
Tuple - Person Data : ("Alex", 48, "35kg", "6ft")

