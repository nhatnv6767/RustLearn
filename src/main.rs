fn all_caps(word: &str) -> String {
    word.to_uppercase();
}
fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    // a macro, this macro tells the compiler that this function is
    // is testing other code
    #[test]
    fn check_all_caps(){
        // asserts_eq will make an assertion that one value is equal to another value
        // and if that assertion does not hold, then the program will abort,
        // thus failing our test
        #[test]
        fn test_name() {
            // three arguments: 1. value want to check
            // 2. the value that we're expecting
            // 3. a message we can display when the test does not work out.
            let result = all_caps("hello");
            let expected = String::from("HELLO");
            assert_eq!(result, expected, "String should be all uppercase");
        }
    }
}
