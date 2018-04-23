pub fn string_return() -> String {
    let s = String::from("a mutable string");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_string_test() {
        let t = string_return();
        assert_eq!(t, "a mutable string");
    }
}
