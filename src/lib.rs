pub mod type_checker;

use std::io;
use std::str::FromStr;
use std::fmt::Debug;

// Get a single item
pub fn get_atom<T>() -> T
where
      T: Debug + FromStr,
      T::Err: Debug
{
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read input");

    T::from_str(input.trim()).unwrap()
}

// Get a vector of items
pub fn get_vector<T>() -> Vec<T> 
where
    T: Debug + FromStr,
    T::Err: Debug
{
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read input");

    input.trim().split_whitespace()
                .map(|e| T::from_str(e).unwrap())
                .collect::<Vec<_>>()
}

// Get a pair of items of the same type
pub fn get_pair<T, U>() -> (T, U) 
where
    T: Debug + FromStr,
    T::Err: Debug,
    U: Debug + FromStr,
    U::Err: Debug
{
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("failed to read input");

    let p  = input.trim().split_whitespace()
                             .collect::<Vec<_>>();

    (T::from_str(&p[0].trim()).unwrap(), U::from_str(&p[1].trim()).unwrap())
}
