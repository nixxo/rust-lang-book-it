// ANCHOR: here
pub fn cerca<'a>(query: &str, contenuti: &'a str) -> Vec<&'a str> {
    contenuti
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
// ANCHOR_END: here

pub fn cerca_case_insensitive<'a>(
    query: &str,
    contenuti: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut risultati = Vec::new();

    for line in contenuti.lines() {
        if line.to_lowercase().contains(&query) {
            risultati.push(line);
        }
    }

    risultati   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contenuti = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contenuti = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            cerca_case_insensitive(query, contenuti)
        );
    }
}
