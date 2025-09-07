pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let Ok(query_regex) = build_query_regex(query) else {
        return search_case_insensitive_without_regex(query, contents);
    };

    search_case_insensitive_with_regex(contents, &query_regex)
}

fn search_case_insensitive_with_regex<'a>(
    contents: &'a str,
    query_regex: &regex::Regex,
) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| query_regex.is_match(&line))
        .collect()
}

fn build_query_regex(query: &str) -> Result<regex::Regex, regex::Error> {
    regex::Regex::new(&format!("(?i){query}"))
}

fn search_case_insensitive_without_regex<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive_without_regex() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive_without_regex(query, contents)
        );
    }

    #[test]
    fn case_insensitive_with_regex() {
        let query = "rUsT";
        let query_regex = build_query_regex(query).unwrap();

        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive_with_regex(contents, &query_regex)
        );
    }
}
