use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

// By default, case_insentive is false
impl Default for Config {
    fn default() -> Config {
        Config {
            query: String::new(),
            filename: String::new(),
            case_insensitive: false,
        }
    }
}

// define a custom error for Box<dyn Error>
#[derive(Debug, Clone)]
// field-less struct: tuple struct
struct NotFoundError(String);

impl fmt::Display for NotFoundError {
    // formatter decides how to display the content
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can not find the query: {}", self.0)
    }
}
// the implementation of customer error can be empty
impl Error for NotFoundError {}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Lack arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = match args.len() {
            4 => match &args[3][..] {
                "true" => true,
                _ => false
            },
            _ => false
        };


        Ok(Config { query, filename, case_insensitive })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    // error propagation
    let contents = fs::read_to_string(&cfg.filename)?;
    println!("The query is: {}", cfg.query);
    println!("The filename is: {}", cfg.filename);
    let results = if cfg.case_insensitive {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
        return Ok(())
    }
    // initial the Box smart pointer using `::new`
    Err(Box::new(NotFoundError(cfg.query)))
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();    

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();    

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line)
        }
    }
    results

}

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
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn one_result_case_insensitive() {
        let query = "rUST";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}