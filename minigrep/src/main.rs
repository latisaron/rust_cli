use std::env;
use minigrep::{Config, read_content, search, search_case_insensitive};

// i conceptually refuse to print stuff out to standard error

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();
    // let config = Config::build(&args)?;
    // could also use `let config = Config::build(&args)?;`
    let config = Config::build(&args).unwrap_or_else(|err| {   
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });

    // let content = match fs::read_to_string(file_path) {
    //     Ok(val) => val,
    //     Err(e) => "Error".to_string()
    // };

    let content = read_content(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(config.query, &content)
    } else {
        search(config.query, &content)
    };

    for line in results {
        println!("{}", line);
    };

    Ok(())
}
