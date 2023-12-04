use clap::Parser;

use colored::Colorize;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use toml::{self};

#[derive(Debug, Deserialize)]
struct Config {
    options: Options,
    config: HashMap<String, Vec<CreateFile>>,
}

#[derive(Debug, Deserialize)]
struct Options {
    require_exact_inputs: bool,
}

#[derive(Debug, Deserialize)]
struct CreateFile {
    name: String,
    contents: String,
    append: bool,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The location of the config file
    #[arg(short, long, default_value = "./boilit.toml")]
    config: String,

    /// The template to build
    #[arg(short, long)]
    template: String,

    /// The input values for the template (comma separated)
    #[arg(short, long, default_value = "")]
    inputs: String,

    /// The path to write create the files at
    #[arg(short, long, default_value = "./")]
    path: String,
}

fn count_inputs(create_file: &CreateFile) -> i32 {
    let mut index = 0;
    while create_file.contents.contains(&format!("[{}]", index))
        || create_file.name.contains(&format!("[{}]", index))
    {
        index += 1
    }
    index
}

fn replace_inputs(content: &String, inputs: &Vec<&str>) -> String {
    let mut temp = String::from(content);
    for (index, input) in inputs.iter().enumerate() {
        let from = format!("[{}]", index);
        temp = temp.replace(&from, input)
    }
    temp
}

fn create_file(file: &CreateFile, inputs: &Vec<&str>, args: &Cli) {
    let new_path = format!("{}/{}", args.path, file.name.replace("[0]", inputs[0]));
    let path = Path::new(&new_path);
    let content = replace_inputs(&file.contents, inputs);

    if !path.exists() {
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        let mut new_file = match fs::OpenOptions::new().write(true).create(true).open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                // Write `msg` to `stderr`.
                eprintln!("Unable open new file: '{}'", path.display());
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        match write!(new_file, "{}", content) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                // Write `msg` to `stderr`.
                eprintln!("Unable write new file: '{}'", path.display());
                // Exit the program with exit code `1`.
                exit(1);
            }
        }
    } else if file.append {
        // Add on to an existing file
        let mut file = match fs::OpenOptions::new().write(true).append(true).open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                // Write `msg` to `stderr`.
                eprintln!("Unable open existing file: '{}'", path.display());
                // Exit the program with exit code `1`.
                exit(1);
            }
        };

        match writeln!(file, "{}", content) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{}", e);
                // Write `msg` to `stderr`.
                eprintln!("Unable to append existing file: '{}'", path.display());
                // Exit the program with exit code `1`.
                exit(1);
            }
        }
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let separated_inputs: Vec<&str> = if args.inputs.len() > 0 {
        args.inputs.split(",").collect()
    } else {
        Vec::new()
    };
    let contents = match fs::read_to_string(&args.config) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(e) => {
            eprintln!("{}", e);
            // Write `msg` to `stderr`.
            eprintln!("Could not read config file, '{}'", args.config);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    let data: Config = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(e) => {
            eprintln!("{}", e);
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from: '{}'", args.config);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
    let files: &Vec<CreateFile> = match data.config.get(&args.template) {
        Some(d) => d,
        // Handle the `error` case.
        None => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to find template: '{}'", args.template);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    for file in files {
        let available_inputs = count_inputs(&file);
        if data.options.require_exact_inputs
            && available_inputs != i32::try_from(separated_inputs.len()).unwrap()
        {
            // Write `msg` to `stderr`.
            eprintln!(
                "{}  Your template has {} different inputs, but {} were provided",
                "Require exact inputs is enabled.\n".red(),
                available_inputs,
                separated_inputs.len()
            );
            // Exit the program with exit code `1`.
            exit(1);
        } else {
            create_file(file, &separated_inputs, &args)
        }
    }

    println!("{}", "Successfully created files".green());
}

#[test]
fn test_count_inputs() {
    let file_0 = CreateFile {
        contents: String::from("How are you doing"),
        name: String::from("Hello"),
        append: false,
    };
    let available = count_inputs(&file_0);
    assert_eq!(available, 0);

    let file_1 = CreateFile {
        contents: String::from("Hello how are you doing [0]"),
        name: String::from("[0]"),
        append: false,
    };
    let available = count_inputs(&file_1);
    assert_eq!(available, 1);

    let file_2 = CreateFile {
        contents: String::from("Hello how are you doing [0]"),
        name: String::from("[1]"),
        append: false,
    };
    let available = count_inputs(&file_2);
    assert_eq!(available, 2);

    let file_3 = CreateFile {
        contents: String::from("Hello how are you doing [0] [2]"),
        name: String::from("[1]"),
        append: false,
    };
    let available = count_inputs(&file_3);
    assert_eq!(available, 3);
}

#[test]
fn test_replace_inputs() {
    let content_1 = String::from("How are you doing?");
    let inputs_1: Vec<&str> = Vec::new();
    let final_content = replace_inputs(&content_1, &inputs_1);
    assert_eq!(final_content, "How are you doing?");

    let content_2 = String::from("How are [0] doing?");
    let inputs_2: Vec<&str> = vec!["you"];
    let final_content = replace_inputs(&content_2, &inputs_2);
    assert_eq!(final_content, "How are you doing?");

    let content_3 = String::from("[1] are [0] doing? [1] is the [2]?");
    let inputs_3: Vec<&str> = vec!["you", "What", "plan"];
    let final_content = replace_inputs(&content_3, &inputs_3);
    assert_eq!(final_content, "What are you doing? What is the plan?");
}
