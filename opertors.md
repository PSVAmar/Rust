What Are Arithmetic Operators?
Arithmetic operators are used to perform arithmetic operations.

The following example shows the use of arithmetic operators in a program:

fn main() {
    let a = 4;
    let b = 3;
    
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("Addition:{}", a + b);
    println!("Subtraction:{}", a - b);
    println!("Multiplication:{}", a * b);
    println!("Division:{}", a / b);
    println!("Modulus:{}", a % b);
}

Output
Operand 1:4, Operand 2:3
Addition:7
Subtraction:1
Multiplication:12
Division:1
Modulus:1


What Are Logical Operators?
Logical operators operate on true / false values.

The following example shows the use of logical operators in a program:
fn main() {
  let a = true;
  let b = false;
  println!("Operand 1:{}, Operand 2:{}", a , b);
  println!("AND:{}", a && b);
  println!("OR:{}", a || b);
  println!("NOT:{}", ! a);
}

Output
Operand 1:true, Operand 2:false
AND:false
OR:true
NOT:false


What Are Comparison Operators?
Comparison Operators are used for comparing the values of two operands.

he following example shows the use of comparison operators in a program:

fn main() {
    let a = 2;
    let b = 3;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("a > b:{}", a > b);
    println!("a < b:{}", a < b);
    println!("a >= b:{}", a >= b);
    println!("a <= b:{}", a <= b);
    println!("a == b:{}", a == b);
    println!("a != b:{}", a != b);
}


Bitwise Operators
What Are Bitwise Operators?
Bitwise operators deal with the binary representation of the operands.
