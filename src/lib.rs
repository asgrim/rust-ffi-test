#[no_mangle]
pub extern "C" fn add_two_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use crate::add_two_numbers;

    #[test]
    fn it_works() {
        assert_eq!(add_two_numbers(2, 2), 4);
    }
}
