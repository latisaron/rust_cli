use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments.")
        }
        let query = &args[1];
        let file_path = &args[2];
        return Ok(Config{
            query: query,
            file_path: file_path,
        });
    }
    
}

pub fn read_content(file_path: &str) -> Result<String, std::io::Error> {
    let x = fs::read_to_string(file_path)?;
    println!("With content: {}", x);
    Ok(x)
}
