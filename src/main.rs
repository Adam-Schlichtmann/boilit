use clap::Parser;

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
    #[arg(short, long, default_value = "./config.toml")]
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

fn create_file(file: &CreateFile, inputs: &Vec<&str>, args: &Cli) {
    let new_path = format!("./{}/{}", args.path, file.name.replace("[0]", inputs[0]));
    let path = Path::new(&new_path);
    let content = &file.contents.replace("[0]", inputs[0]);
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
    let separated_inputs: Vec<&str> = args.inputs.split(",").collect();
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
        create_file(file, &separated_inputs, &args)
    }

    println!("Successfully created files");
}
