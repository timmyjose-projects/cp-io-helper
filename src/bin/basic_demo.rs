extern crate cp_io_helper;

use cp_io_helper::type_checker::is_of_type;

fn main() {
    // string demo
    println!("Enter a string: ");
    let string = cp_io_helper::get_atom::<String>();
    assert!(is_of_type::<String>(&string));

    // numerics demo
    println!("Enter an i8: ");
    let int8 = cp_io_helper::get_atom::<i8>();
    assert!(is_of_type::<i8>(&int8));

    println!("Enter an i16: ");
    let int16 = cp_io_helper::get_atom::<i16>();
    assert!(is_of_type::<i16>(&int16));

    println!("Enter a usize: ");
    let unsigned_int = cp_io_helper::get_atom::<usize>();
    assert!(is_of_type::<usize>(&unsigned_int));

    // vector demo
    println!("Enter a vector of i32s: ");
    let vi = cp_io_helper::get_vector::<i32>();
    assert!(is_of_type::<Vec<i32>>(&vi));
    
    println!("Enter a vector of chars: ");
    let vc = cp_io_helper::get_vector::<char>();
    assert!(is_of_type::<Vec<char>>(&vc));
    
    println!("Enter a vector of bytes: ");
    let vb = cp_io_helper::get_vector::<u8>();
    assert!(is_of_type::<Vec<u8>>(&vb));

    // pair demo
    println!("Enter a pair of ints: ");
    let pi = cp_io_helper::get_pair::<i32, i32>();
    assert!(is_of_type::<(i32, i32)>(&pi));

    println!("Enter a pair of int and char: ");
    let pic = cp_io_helper::get_pair::<i32, char>();
    assert!(is_of_type::<(i32, char)>(&pic));
}

