pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_two(a: usize) -> usize {
    internal_adder(a, 2)
}

#[cfg(test)] // include this only during test for compilaions
mod tests { // These are the unit test for this file
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn internal() {
        let result = internal_adder(2, 2); // We are testing a private function
        assert_eq!(result, 4);
    }
}
