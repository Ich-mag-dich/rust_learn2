use std::{
    error::Error,
    fs::{self, read_to_string},
    path::PathBuf,
};

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    if !PathBuf::from(&config.file_path).is_dir() {
        read_file(&config)
    } else {
        read_dir(&mut config);
    }

    Ok(())
}

pub fn read_file(config: &Config) {
    let contents = read_to_string(&config.file_path).unwrap();
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    print_fn(&config.file_path, results, &config);
}

pub fn read_dir(config: &mut Config) {
    let files = fs::read_dir(&config.file_path).unwrap();
    for i in files {
        let file = i.unwrap().path();
        if file.is_file() {
            let contents = String::from(read_to_string(&file).unwrap());
            let results = if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            } else {
                search(&config.query, &contents)
            };
            if !results.0.is_empty() {
                print_fn(&file.to_str().unwrap().to_string(), results, &config);
            }
        } else {
            config.file_path = file.to_str().unwrap().to_string();
            read_dir(config);
        }
    }
}

// pub fn read_files(config: &Config) {}

pub fn print_fn<'a>(filename: &String, results: (Vec<&'a str>, Vec<i32>), config: &Config) {
    let mut count = 0;
    println!("\n\n\x1b[1m\x1b[35m{filename}\x1b0m");
    let conq = &config.query;
    let conq2 = format!("\x1b[31m{}\x1b[0m", conq);
    for line in results.0 {
        // let color = line.replace("{&config.query}", "")
        let line1 = line.to_string().replace(conq, conq2.as_str());
        // println!("{line1}");

        println!("\x1b[32m{}\x1b[0m: {}", results.1[count], line1);
        count += 1;
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = if args[2].is_empty() {
            ".".to_string()
        } else {
            args[2].clone()
        };
        let ignore_case = std::env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> (Vec<&'a str>, Vec<i32>) {
    let mut results = Vec::new();
    let mut results_count = Vec::new();
    let mut count = 1;
    for line in contents.lines() {
        // println!("{count} {line}");
        if line.contains(query) {
            results.push(line);
            results_count.push(count);
        }
        count += 1;
    }
    (results, results_count)
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> (Vec<&'a str>, Vec<i32>) {
    let query = query.to_lowercase();
    let mut count = 1;
    let mut results = Vec::new();
    let mut results_count = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
            results_count.push(count);
        }
        count += 1;
    }
    (results, results_count)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }
