use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // the ? will return the error for the calling function to handle
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    
    for line in results {
        println!("{}", line);
    }

    // since we are only really worried about running this function for the side effect we dont
    // need to actually "return" anything. so wrapping () in Ok() is proper in this case
    Ok(())
}

pub struct Config {
    pub query         : String,
    pub filename      : String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // NOTE since arg[0] is the name of the executable we start indexing at 1
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// here we define a function with a lifetime annotation
// letting a user know that the lifetime of the returned value is valid as long as contents is
// will use the smallest lifetime if multiplue values use generic lifetime 'a
// any data created from the referenece of our contents string slice is REQUIRED to have the same
// lifetime so that is really what we accomplish here using the generic lifetime annotation
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(&query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
    .filter(|line| line.to_ascii_lowercase().contains(&query))
    .collect()
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
         
        assert_eq!(vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents));
    
    }
}
