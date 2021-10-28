use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String
}

#[derive(Debug, Clone)]
struct NotFoundError(String);

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can not find the query: {}", self.0)
    }
}
impl Error for NotFoundError {}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Lack arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&cfg.filename)?;
    println!("The query is: {}", cfg.query);
    println!("The filename is: {}", cfg.filename);
    for (it, line) in content.lines().enumerate() {
        if line.contains(&cfg.query) {
            println!("Search result: line {}, {}", it + 1, line);
            return Ok(())
        }
    }
    Err(Box::new(NotFoundError(cfg.query)))
}
