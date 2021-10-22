pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(query: &String, filename: &String) -> Result<Config, &'static str> {
        let query = query.clone();
        let filename = filename.clone();
        Ok(Config { query, filename })
    }
}