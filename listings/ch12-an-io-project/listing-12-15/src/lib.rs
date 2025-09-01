pub fn cerca<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

// ANCHOR: here
// --snip--

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], cerca(query, contents));
    }
}
// ANCHOR_END: here
