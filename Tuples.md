What are Tuples?
Tuples are heterogeneous sequences of elements, meaning, each element in a tuple can have a different data type. Just like arrays, tuples are of a fixed length.


Define a Tuple
A tuple can be defined by writing let followed by the name of the tuple and then enclosing the values within the parenthesis.

Syntax 1
The syntax below defines a tuple without specifying the type. However, the compiler can infer the type.


let tuple_name = ("value",'c',1)  //first is string,second is char,third is int

Syntax2:
The syntax below defines a tuple by specifying the type.

let tuple_naame :(&str,char,i32) = ("value1",'c',1);


Example of the Tuple in rust
#[allow(unused_variables, unused_mut)]
fn main() {
    //define a tuple
    let person_data = ("Alex", 48, "35kg", "6ft");
    // define a tuple with type annotated
    let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
}

