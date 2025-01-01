use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            // クエリ文字列を取得しませんでした
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            // ファイル名を取得しませんでした
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?; // open file

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With the text:\n{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn text_config() {
        let args = vec![
            "program_name".to_string(),
            "test_query".to_string(),
            "test_filename".to_string(),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "test_query");
        assert_eq!(config.filename, "test_filename");
    }

    #[test]
    fn test_config_missing_arguments() {
        let args = vec!["program_name".to_string(), "test_query".to_string()];
        let config = Config::new(&args);
        assert!(config.is_err());
    }

    #[test]
    fn test_config_valid_file() {
        let filename = "test_file.txt";
        let content = "This is a test file.";
        fs::write(filename, content).unwrap();
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
        };

        let result = run(config);
        assert!(result.is_ok());

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn test_config_missing_file() {
        let config = Config {
            query: "test".to_string(),
            filename: "missing_file.abc".to_string(),
        };

        let result = run(config);
        assert!(result.is_err());
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

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
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
