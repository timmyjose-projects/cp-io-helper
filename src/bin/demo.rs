extern crate cp_io_helper;

fn main() {
    let nums = cp_io_helper::get_vector::<i32>();

    let (mn, mx) = cp_io_helper::get_pair::<i32, i32>();

    println!("{}", nums.iter()
                    .filter(|&d| *d >= mn && *d <= mx)
                    .count());
}
