print!()
The print!() macro simply prints the output to the console.

Example
The following example prints “Rust Programming Course” in one line.

fn main() {
    print!("Rust Programming");
    print!(" Course");
}

println!()
The println!() macro appends a new line at the end of the string.

Example
The following example prints “Rust Programming” on one line and “Course” on the new line.

fn main() {
    println!("Rust Programming");
    println!("Course");
}

eprint!()
The eprint!() macro displays the output as an error.

Example
The following example prints “Rust Programming” and “Course” on the same line but as an error.

fn main() {
    eprint!("Rust Programming");
    eprint!(" Course");
}

eprintln!()
The eprint!() macro displays the output as an error and appends a new line(\n) at the end of it.

Example
The following example prints “Rust Programming” as an error and appends a new line to it. Then prints “Course” and appends a new line.

fn main() {
    eprintln!("Rust Programming");
    eprintln!("Course");
}