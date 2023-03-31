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
        "/pnpm/",
    ];
    let files = fs::read_dir(&config.file_path).unwrap();

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
        if file.is_file() {
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
            let file2 = &file.clone();
            let custom_args = vec![
                "minigrep".to_string(),
                config.query.clone(),
                file2.to_str().unwrap().to_string(),
            ];
            let config2 = Config::build(custom_args.into_iter()).unwrap_or_else(|err| {
                eprintln!("Problem parsing arguments: {}", err);
                std::process::exit(1);
            });
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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => String::from("."),
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
    (
        Vec::from(
            contents
                .lines()
                .into_iter()
                .filter(|x| x.contains(query))
                .collect::<Vec<_>>(),
        ),
        Vec::from(
            contents
                .lines()
                .into_iter()
                .enumerate()
                .filter(|(_, x)| x.contains(query))
                .map(|(i, _)| i as i32 + 1)
                .collect::<Vec<_>>(),
        ),
    )
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> (Vec<&'a str>, Vec<i32>) {
    (
        Vec::from(
            contents
                .lines()
                .into_iter()
                .filter(|x| x.to_lowercase().contains(&query.to_lowercase()))
                .collect::<Vec<_>>(),
        ),
        Vec::from(
            contents
                .lines()
                .into_iter()
                .enumerate()
                .filter(|(_, x)| x.to_lowercase().contains(&query.to_lowercase()))
                .map(|(i, _)| i as i32 + 1)
                .collect::<Vec<_>>(),
        ),
    )
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
