use std::fs;
use std::env;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments.")
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        return Ok(Config{
            query: query,
            file_path: file_path,
            ignore_case: ignore_case,
        });
    }
    
}

pub fn read_content(file_path: &str) -> Result<String, std::io::Error> {
    let x = fs::read_to_string(file_path)?;
    Ok(x)
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut tmp_vec: Vec<&'a str> = vec![];
    for line in content.lines() {
        if line.contains(query) {
            tmp_vec.push(line);
        }
    };

    tmp_vec
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut tmp_vec: Vec<&'a str> = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            tmp_vec.push(line);
        }
    };

    tmp_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = " aronelu \n hai la joc \nsafe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let content = " aronelu \n hai la joc \nsafe, fast, productive.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, content));
    }
}
