use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::PathBuf;
use colored::*;

const VERSION: &str = "
SFTM (Super Fast Todo Manager) Version 0.1.0

MIT License

Copyright (c) 2024 QUAKTECH.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the \"Software\"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
";

enum Command {
    Add,
    Remove,
    Version,
    Check,
    Show,
    List,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Usage: sftm <command> [args...]".red());
        std::process::exit(1);
    }

    let command = match args[1].as_str() {
        "add" => Command::Add,
        "remove" => Command::Remove,
        "version" => Command::Version,
        "check" => Command::Check,
        "show" => Command::Show,
        "list" => Command::List,
        _ => {
            eprintln!("{}", format!("Unknown command: {}", args[1]).red());
            std::process::exit(1);
        }
    };

    match command {
        Command::Add => {
            if args.len() < 4 {
                eprintln!("{}", "Usage: sftm add <todo> <file>".red());
                std::process::exit(1);
            }
            let todo = &args[2];
            let file = &args[3];
            add_todo(todo, file);
        }
        Command::Remove => {
            if args.len() < 3 {
                eprintln!("{}", "Usage: sftm remove [-f] <line_number/filename> [file]".red());
                std::process::exit(1);
            }
            if args[2] == "-f" {
                if args.len() < 4 {
                    eprintln!("{}", "Usage: sftm remove -f <filename>".red());
                    std::process::exit(1);
                }
                remove_file(&args[3]);
            } else {
                if args.len() < 4 {
                    eprintln!("{}", "Usage: sftm remove <line_number> <file>".red());
                    std::process::exit(1);
                }
                let line_number: usize = args[2].parse().unwrap_or(0);
                let file = &args[3];
                remove_todo(line_number, file);
            }
        }
        Command::Version => {
            println!("{}", VERSION);
        }
        Command::Check => {
            if args.len() < 4 {
                eprintln!("{}", "Usage: sftm check <line_number> <file>".red());
                std::process::exit(1);
            }
            let line_number: usize = args[2].parse().unwrap_or(0);
            let file = &args[3];
            check_todo(line_number, file);
        }
        Command::Show => {
            if args.len() < 3 {
                eprintln!("{}", "Usage: sftm show <file>".red());
                std::process::exit(1);
            }
            let file = &args[2];
            show_todos(file);
        }
        Command::List => {
            list_todofiles();
        }
    }
}

fn get_todo_file_path(file: &str) -> PathBuf {
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let mut path = PathBuf::from("/home");
    path.push(&username);
    path.push(".sftm");
    path.push("todofiles");
    std::fs::create_dir_all(&path).unwrap();
    path.push(file);
    path
}

fn add_todo(todo: &str, file: &str) {
    let path = get_todo_file_path(file);

    println!("Enter a description for the todo:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim();

    let todo_entry = format!("{} - {}\n", todo, description);

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();

    file.write_all(todo_entry.as_bytes()).unwrap();
    println!("{}", "Todo added successfully!".green());
}

fn check_todo(line_number: usize, file: &str) {
    let path = get_todo_file_path(file);
    let file = OpenOptions::new().read(true).write(true).open(&path).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    if line_number == 0 || line_number > lines.len() {
        println!("{}", "Error: Invalid line number".red());
        return;
    }

    let index = line_number - 1;
    if !lines[index].starts_with("✅") {
        lines[index] = format!("✅ {}", lines[index]);
        let mut file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
        println!("{}", "Todo checked off successfully!".green());
    } else {
        println!("{}", "Todo is already checked off".yellow());
    }
}

fn show_todos(file: &str) {
    let path = get_todo_file_path(file);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("✅") {
            println!("{}", line.green());
        } else {
            println!("{}", line);
        }
    }
}

fn remove_todo(line_number: usize, file: &str) {
    let path = get_todo_file_path(file);
    let file = OpenOptions::new().read(true).write(true).open(&path).unwrap();
    let reader = BufReader::new(&file);
    let mut lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    if line_number == 0 || line_number > lines.len() {
        println!("{}", "Error: Invalid line number".red());
        return;
    }

    let index = line_number - 1;
    lines.remove(index);

    let mut file = OpenOptions::new().write(true).truncate(true).open(path).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }
    println!("{}", "Todo removed successfully!".green());
}

fn remove_file(filename: &str) {
    let path = get_todo_file_path(filename);
    match fs::remove_file(path) {
        Ok(_) => println!("{}", format!("File '{}' removed successfully", filename).green()),
        Err(e) => println!("{}", format!("Error removing file '{}': {}", filename, e).red()),
    }
}

fn list_todofiles() {
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let mut path = PathBuf::from("/home");
    path.push(&username);
    path.push(".sftm");
    path.push("todofiles");

    match fs::read_dir(path) {
        Ok(entries) => {
            println!("{}", "Todo files:".bold());
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("  {}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => println!("{}", format!("Error listing todo files: {}", e).red()),
    }
}
