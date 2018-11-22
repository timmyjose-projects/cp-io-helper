use std::any::Any;

// Check if the given value is of the specified type
pub fn is_of_type<T: 'static>(val: &Any) -> bool {
    val.is::<T>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        assert!(is_of_type::<String>(&String::from("Hello")));
    }

    #[test]
    fn test_not_string() {
        assert!(!is_of_type::<String>(&100));
    }

    #[test]
    fn test_is_vector_of_ints() {
        assert!(is_of_type::<Vec<i32>>(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_is_not_vector_of_ints() {
        assert!(!is_of_type::<Vec<i32>>(&vec!['a', 'b', 'c']));
    }

    #[test]
    fn test_is_vector_of_char() {
        assert!(is_of_type::<Vec<char>>(&vec!['a', 'b', 'c']));
    }

    #[test]
    fn test_is_not_vector_of_char() {
        assert!(!is_of_type::<Vec<char>>(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_is_pair_of_bytes() {
        assert!(is_of_type::<(u8, u8)>(&(b'a', b'b')));
    }

    #[test]
    fn test_is_not_pair_of_bytes() {
        assert!(!is_of_type::<(u8, u8)>(&(1.23, 2.34)));
    }

    #[test]
    fn test_is_pair_of_int_and_char() {
        assert!(is_of_type::<(i32, char)>(&(10, 'c')));
    }

    #[test]
    fn test_is_not_pair_of_int_and_char() {
        assert!(!is_of_type::<(i32, char)>(&(b'1', 1.23)));
    }
}
