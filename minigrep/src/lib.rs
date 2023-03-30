use std::{
    error::Error,
    fs::{self, read_to_string},
    path::PathBuf,
};

pub fn run(mut config: Config) -> Result<(), Box<dyn Error>> {
    if !PathBuf::from(&config.file_path).is_dir() {
        read_file(&config)
    } else {
        read_dirs(&mut config);
    }
    Ok(())
}

pub fn read_file(config: &Config) {
    let contents = read_to_string(&config.file_path);
    let contents = match contents {
        Ok(c) => c,
        Err(_) => {
            // eprintln!("Error reading file: {} - {:?}", e, file);
            return;
        }
    };
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    print_fn(&config.file_path, results, &config);
}

pub fn read_dirs(config: &mut Config) {
    // println!("Reading directory: {}", config.file_path);
    let grep_blacklist = vec![
        "/target",
        "/.git",
        "/.vscode",
        "/.idea",
        "node_modules",
        "CMakeFiles",
        "build",
        ".iml",
        ".xcodeproj",
    ];
    // let mut count = 0;
    let files = fs::read_dir(&config.file_path).unwrap();

    // let files = files.collect::<Result<Vec<_>, _>>().unwrap();
    // println!("{:?}", files);
    'outer: for i in files {
        let file = i.unwrap().path();
        for b in grep_blacklist.iter() {
            if file
                .to_str()
                .unwrap()
                .to_lowercase()
                .contains(&b.to_lowercase())
            {
                continue 'outer;
            }
        }
        // println!("{:?}", file);
        if file.is_file() {
            // if count > 3 {
            //     break;
            // }
            let contents = match read_to_string(&file) {
                Ok(c) => c,
                Err(_) => {
                    // eprintln!("Error reading file: {} - {:?}", e, file);
                    continue;
                }
            };
            let results = if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            } else {
                search(&config.query, &contents)
            };
            if !results.0.is_empty() {
                print_fn(&file.to_str().unwrap().to_string(), results, &config);
            }
        } else {
            // let mut new_config = config.clone();
            // count += 1;
            let file2 = &file.clone();
            let custom_args = vec![
                "minigrep".to_string(),
                config.query.clone(),
                file2.to_str().unwrap().to_string(),
            ];
            let config2 = Config::build(&custom_args).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {}", err);
                std::process::exit(1);
            });
            // config.file_path = file.to_str().unwrap().to_string();
            if let Err(e) = run(config2) {
                eprintln!("Application error: {e}");
                std::process::exit(1);
            }
        }
    }
}

pub fn print_fn<'a>(filename: &String, results: (Vec<&'a str>, Vec<i32>), config: &Config) {
    let mut count = 0;
    println!("\n\x1b[1m\x1b[35m {filename} \x1b0");
    let conq = &config.query;
    let conq2 = format!("\x1b[31m{}\x1b[0m", conq);
    for line in results.0 {
        let line1 = line.to_string().replace(conq, conq2.as_str());
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
