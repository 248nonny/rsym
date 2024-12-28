
#[cfg(test)]
mod tests {
    use rsym::parser::*;


    #[test]
    #[should_panic]
    fn invalid_string_test() {
        string_to_expression("".to_string());
    }
}
