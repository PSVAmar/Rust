Get Slice 
Slice is basically a portion of an array. It lets you refer to a subset of a contiguous memory location. But unlike an array, the size of the slice is not known at compile time.

Syntax 
A slice is a two-word object, the first word is a data pointer and the second word is a slice length.

Data pointer is a programming language object that points to the memory location of the data, i.e., it stores the memory address of the data.

To declare an array slice, we need to specify the name of the source array and the range of elements to be included in the slice.

Note: If the range of elements is not specified, it will consider the whole array as a slice.

fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //define the slice
    let slice_array1:&[i32] = &arr;
    let slice_array2:&[i32] = &arr[0..2];
    // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
}

Output:
Slice of an array: [1, 2, 3, 4]
Slice of an array: [1, 2]