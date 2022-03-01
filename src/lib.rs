use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    filename: String,
    query: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("need query arg"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("need filename arg"),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
        let query = "Hello";
        let contents = "\
Rust
Hello,Rust
World";
        assert_eq!(vec!["Hello,Rust"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = "\
 Rust
Hello,Rust
World";
        assert_eq!(vec!["Hello,Rust"], search_insensitive(query, contents))
    }
}
