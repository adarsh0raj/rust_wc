use std::env;
use std::fs;

fn print_help() {
    println!("Usage: rust_wc [OPTION]... [FILE]...");
    println!("Print line, characters, word, and byte counts for a FILE.");
    println!("");
    println!("Without any options, wc counts the number of lines, words, and characters in the input files.");
    println!("  -c, --bytes            print the byte counts");
    println!("  -l, --lines            print the newline counts");
    println!("  -w, --words            print the word counts");
    println!("  -m, --characters       print the character counts");
    println!("  -h, --help             display this help and exit");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("Invalid number of arguments");
        print_help();
    }

    else if args.len() == 2 {
        let filename = &args[1];

        if filename != "-h" {
            println!("Invalid option");
        }
    }
    else {
        let flag = &args[1];
        let filename = &args[2];

        if flag == "-c" {
            let metadata = fs::metadata(filename).expect("Something went wrong reading number of bytes in the file");
            println!("{} {}", metadata.len(), filename);
        }
        else if flag == "-l" {
            let contents = fs::read_to_string(filename).expect("Something went wrong reading number of lines in the file");
            let lines = contents.lines().count();
            println!("{} {}", lines, filename);
        }
        else if flag == "-w" {
            let contents = fs::read_to_string(filename).expect("Something went wrong reading number of words in the file");
            let words = contents.split_whitespace().count();
            println!("{} {}", words, filename);
        }
        else if flag == "-m" {
            let contents = fs::read_to_string(filename).expect("Something went wrong reading number of characters in the file");
            let characters = contents.chars().count();
            println!("{} {}", characters, filename);
        }
        else if flag == "-h" {
            print_help();
        }
        else {
            println!("Invalid option");
            print_help();
        }
    }
}
