use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    let ignore_case = std::env::var("CASE_INSENSITIVE").is_ok();

    Ok(Config {
      query,
      filename,
      ignore_case,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  if results.is_empty() {
    println!("No results found");
  }

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}
