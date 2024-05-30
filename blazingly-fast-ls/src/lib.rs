use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case = std::env::var("IGNORE_CASE").is_ok();

            return Ok(Config {
                query,
                file_path,
                ignore_case,
            });
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "hello";
        let contents = "\
        hello world
        how are you?
        i am good.
        ";

        assert_eq!(vec!["hello world"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = "\
        Hello world
        how are you?
        i am good.
        ";

        assert_eq!(
            vec!["Hello world"],
            search_case_insensitive(query, contents)
        );
    }
}
