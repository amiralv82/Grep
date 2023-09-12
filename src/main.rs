#![allow(unused)]
use::std::env;
use std::process;
use std::error::Error;
use::std::fs;
use std::path::Path;
extern crate walkdir;
use walkdir::WalkDir;
use regex::Regex;
use rayon::prelude::*;


#[derive(Clone)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub non_match: bool,
    pub directory: bool,
    pub inside:bool,
    pub depth: Option<i32>,
    pub threads: bool,
    pub num_of_threads: Option<i32>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        //invironment variables
        let ignore_case = env::var("IC").is_ok();
        let non_match = env::var("NON").is_ok();
        let directory = env::var("DIR").is_ok();
        let inside = env::var("IN").is_ok();
        let depth = env::var("DPT").ok().map(|d| d.parse::<i32>().ok()).flatten();
        let threads = env::var("TH").is_ok();
        let num_of_threads = env::var("NT").ok().map(|d| d.parse::<i32>().ok()).flatten();
        //What we want to find
        let query = args[1].clone();
        //If we just want to find directories we just need on argument
        if directory {
            let file_path = args[1].clone();
            Ok(Config { query, file_path, ignore_case, non_match, directory, inside,depth,threads,num_of_threads })
        } else {
            if args.len() < 3 {
                if args[1].contains("help---") {
                    println!("\t\tUsage: [OPTION] cargo run [PATTERNS] [FILE]\t\t
                                    Search for PATTERNS in each FILE.\t\t
                                    Example:cargo run PATTERN FILE\n\t\t
                                    Pattern selection and interpretation:\t\t
                                    Boolean:\t\t
                                    IC=\t:Search REGARDLESS of the UPPERCASE and LOWERCASE letters\t\t
                                    NON=\t:Search For FILES That Do Not have that PATTERNS\t\t
                                    DIR=\t:Search inside DIRECTORIES and find ONLY files with that pattern\t\t
                                    !! DONT ADD [FILE] PART IN THIS USAGE!!\t\t
                                    IN=\t:Search inside the FILES FOUNDED in DIRECTORIES\t\t
                                    TH=\t:PARALLEL SEARCHING\n\t\t
                                    With Variables:\t\t
                                    DPT:[1..n]\t:Specify the search DEPTH through DIRECTORIES\t\t
                                    NT:[1..n]\t:Specify the NUMBER OF THREADS in parallel searching\n\t\t
                    ");
                    panic!();
                }else{
                    return Err("not enough arguments");
                }
            }
            let file_path = args[2].clone();
            Ok(Config { query, file_path, ignore_case, non_match, directory, inside,depth,threads,num_of_threads })
        }
    }
}

fn main() {
    // Arguments [what we want to finde] [where we want to find]
    let args: Vec<String> = env::args().collect();
    println!("\t\t====FOR HELP TRY====\n\t\t====cargo run (help---)====");
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    // Parallel Searching
    if config.threads { //[TH(parallel searching)] && [NT(number of threads)]
        // Threadpool
        let number_of_threads = config.num_of_threads.unwrap_or(6) as usize;
        let pool = rayon::ThreadPoolBuilder::new().num_threads(number_of_threads).build().unwrap();
        if config.directory{
            if !config.inside && args.len()>2{
                panic!("You should use one argument in this situation");
            }else if config.inside{
                pool.install(||search_inside(config.clone()));
            }
            pool.install(||search_directory(config));
        }else{
            let contents = fs::read_to_string(config.file_path).expect("no such a file or directory");
    
            if config.ignore_case && config.non_match{
                pool.install(||search_non_insensitive(&config.query, &contents))
            } else if config.ignore_case {
                pool.install(||search_case_insensitive(&config.query, &contents))
            } else if config.non_match{
                pool.install(||search_non(&config.query, &contents))
            }else{
                pool.install(||search(&config.query, &contents))
            };
        }
    }else{
        // Searching ordinary
        if config.directory{
            if !config.inside && args.len()>2{
                panic!("You should use one argument in this situation");
            }else if config.inside{
                search_inside(config.clone());
            }
            search_directory(config);
        }else{
            let contents = fs::read_to_string(config.file_path).expect("no such a file or directory");
    
            if config.ignore_case && config.non_match{
                search_non_insensitive(&config.query, &contents)
            } else if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            } else if config.non_match{
                search_non(&config.query, &contents)
            }else{
                search(&config.query, &contents)
            };
        }
    }
} 
// [DIR](search through directories) using Walkdir
pub fn search_directory<'a>(mut config: Config){
    let folder_path = std::env::current_dir().expect("no such a file or directory");
    let path = Path::new(&folder_path);
    // [DPT] depth of searching by default = 1
    let max_depth = config.depth.unwrap_or(1) as usize; // Convert i32 to usize

    println!("--------------------------------------------------------------------------------------Directories");
    if config.ignore_case && config.non_match{
        let re = Regex::new(&config.query.to_lowercase()).expect("no such a file or directory");
            for file in WalkDir::new(path).min_depth(0).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
            if !re.is_match(file.path().to_string_lossy().to_lowercase().as_ref()){
                println!("{}", file.path().display());
            }
        }
    } else if config.ignore_case {
        let re = Regex::new(&config.query.to_lowercase()).expect("no such a file or directory");
        for file in WalkDir::new(path).min_depth(0).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
            if re.is_match(file.path().to_string_lossy().to_lowercase().as_ref()){
                println!("{}", file.path().display());
            }
        }
    } else if config.non_match{
        let re = Regex::new(&config.query).expect("no such a file or directory");
        for file in WalkDir::new(path).min_depth(0).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
            if !re.is_match(file.path().to_string_lossy().as_ref()){
                println!("{}", file.path().display());
            }
        }
    }else{
        let re = Regex::new(&config.query).expect("no such a file or directory");
        for file in WalkDir::new(path).min_depth(0).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
            if re.is_match(file.path().to_string_lossy().as_ref()){
                println!("{}", file.path().display());
            }
        }
    }
}

//[IN] searching inside files we found in directories
pub fn search_inside<'a>(config: Config){
    let folder_path = std::env::current_dir().expect("no such a file or directory");
    let path = Path::new(&folder_path);
    println!("--------------------------------------------------------------------------------------Inside");
    for file in WalkDir::new(path).min_depth(0).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        //if it is a text file or subtitle file we search through it
        if file.path().to_string_lossy().ends_with(".txt") || file.path().to_string_lossy().ends_with(".srt"){
            println!("Inside file: {}", file.path().display());
            let contents = fs::read_to_string(file.path()).expect("no such a file or directory");
            let results = if config.ignore_case && config.non_match{
                search_non_insensitive(&config.query, &contents)
            } else if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            } else if config.non_match{
                search_non(&config.query, &contents)
            }else{
                search(&config.query, &contents)
            };
        }
    }
}
//Searching in text files and subtitle files
pub fn search<'a>(query: &str, contents: &'a str) {
    let mut matching_results = Vec::new();
    let re = Regex::new(query).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line) {
            matching_results.push((i + 1, line));
        }
    }
    //print results and lines
    for (line_number, line_text) in matching_results {
        println!("{}: {}", line_number, line_text);
    }
}

//[NON] search for files and lines WITHOUT the words we want
pub fn search_non<'a>(query: &str, contents: &'a str) {
    let mut non_matching_results = Vec::new();
    let re = Regex::new(query).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if !re.is_match(line) {
            non_matching_results.push((i + 1, line));
        }
    }
    for (line_number, line_text) in non_matching_results {
        println!("{}: {}", line_number, line_text);
    }
}

//[IC] Ignore Case searching  and  [NON]Non Matching Results
pub fn search_non_insensitive<'a>(query: &str,contents: &'a str,) {
    let mut non_matching_results = Vec::new();
    let re = Regex::new(&query.to_lowercase()).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if !re.is_match(line.to_lowercase().as_str()) {
            non_matching_results.push((i + 1, line));
        }
    }
    for (line_number, line_text) in non_matching_results {
        println!("{}: {}", line_number, line_text);
    }
}
//[IC] Ignore Case searching
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str,) {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    let re = Regex::new(&query.to_lowercase()).unwrap();
    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line.to_lowercase().as_str()) {
            results.push((i + 1, line));
        }
    }
    for (line_number, line_text) in results {
        println!("{}: {}", line_number, line_text);
    }
}