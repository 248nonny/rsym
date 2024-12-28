// Tests for 4 operation parser.

#[cfg(test)]
mod tests {
    use rsym::expression::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        println!("{result:}");
        assert_eq!(result, 4);
    }
}
