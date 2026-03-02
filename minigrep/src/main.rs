use minigrep::{search, search_case_insensitive};
use regex::RegexBuilder;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Could not parse arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

// INFO: Box<T> is a smart pointer that allows to store data on the heap with a dynamic size, since the size of the error message can vary.
// dyn Error is a dynamic error type that allows for flexible error handling (we don't know the exact error type, but it implements the Error trait).
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        print_highlighted_regex(line, &config.query, config.ignore_case);
    }

    Ok(())
}

fn print_highlighted(line: &str, query: &str, ignore_case: bool) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";

    let search_line: String = if ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    };
    let search_query: String = if ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };

    let mut search_slice = &search_line[..]; // Create a tracking slice
    let mut current_line = line; // Track the original line slice

    // INFO: String.find() always starts searching from the beginning of the string (index 0)
    // unicode unsafe since use byte slice
    while let Some(pos) = search_slice.find(&search_query) {
        let end_pos = pos + search_query.len();

        print!("{}", &current_line[..pos]);
        print!("{}{}{}", RED, &current_line[pos..end_pos], RESET);

        // Move the slices forward
        search_slice = &search_slice[end_pos..];
        current_line = &current_line[end_pos..];
    }
    println!("{}", current_line); // Print the remainder
}

fn print_highlighted_regex(line: &str, query: &str, ignore_case: bool) {
    const RED: &str = "\x1b[31m";
    const RESET: &str = "\x1b[0m";

    // Escape the query in case it contains special regex characters like '.' or '*'
    let escaped_query = regex::escape(query);

    // Build the regex with the case-insensitive flag if needed
    let re = RegexBuilder::new(&escaped_query)
        .case_insensitive(ignore_case)
        .build()
        .expect("Invalid regex");

    let mut last_end = 0;

    // find_iter handles the "moving forward" logic for you
    for mat in re.find_iter(line) {
        // Print everything from the last match up to the start of this match
        print!("{}", &line[last_end..mat.start()]);

        // Print the highlighted match using its actual range in the string
        print!("{}{}{}", RED, &line[mat.start()..mat.end()], RESET);

        last_end = mat.end();
    }

    // Print the remaining part of the string
    println!("{}", &line[last_end..]);
}

#[derive(Default)]
struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            // is_ok checks whether the environment variable is set
            let ignore_case = env::var("IGNORE_CASE").is_ok();

            Ok(Config {
                query,
                file_path,
                ignore_case,
                // ..Config::default()
            })
        }
    }
}
