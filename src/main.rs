use bob::lexical_analysis;
use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::process;

///-----------------------------DOCS-----------------------------------
/// 1. Head to standard_function_declarations to define a new function.
///
/// 2. lib defines a couple useful functions
///
///--------------------------------------------------------------------

fn main() {
    let mut headers: bob::Headers = bob::Headers {
        iostream: false,
        limits: false,
        string: false,
    };
    let arg: Vec<String> = env::args().collect();
    let mut enable_source: bool = false;
    // uses a reader buffer
    if arg.len() < 2 {
        panic!("Enter the name of the Bob source file!");
    }
    if arg.len() == 3 {
        if arg[2] == "--output-source" {
            if fs::metadata("output.cpp").is_ok() {
                fs::remove_file("output.cpp").expect("Couldn't remove file output.cpp!");
                println!("Bob replaced output.cpp!");
            }
            enable_source = true;
        }
    }
    let mut translated_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("output.cpp")
        .expect("File creation failed!");
    let reader = io::BufReader::new(fs::File::open(&arg[1]).expect("Couldn't open that file!"));
    let mut buffer: Vec<char> = vec![];
    let mut last_char: char = 'a';
    let mut variable_stack: Vec<bob::Variable> = Vec::new();
    let mut ignore_char: i32 = 0;
    for line in reader.lines() {
        for c in line.expect("Source read failed!").chars() {
            if ignore_char != 0 {
                if c == '(' {
                    ignore_char += 1;
                } else if c == ')' {
                    ignore_char -= 1;
                }
                last_char = c;
                continue;
            } else if c == '.' && last_char == ' ' {
                buffer.push(c);
                let query = buffer.iter().cloned().collect::<String>();
                let query: String = query.trim().to_string();
                if query.len() == 0 {
                    continue;
                }
                let query: String =
                    lexical_analysis::string_space_remover_and_bracket_replacer(query);
                let mut query: String = lexical_analysis::bob_and_punctuation_remover(query);
                bob::iterator(
                    &mut query,
                    &mut translated_file,
                    &mut headers,
                    &mut variable_stack,
                );
                query.clear();
                buffer.clear();
            } else if c == '(' {
                ignore_char += 1;
            } else if c == ')' {
                bob::raise(bob::Error::MALFORMED_COMMENT);
            } else {
                buffer.push(c);
            }
            last_char = c;
        }
    }
    bob::header_and_token_includer(headers);
    process::Command::new("g++")
        .arg("output.cpp")
        .arg("-o")
        .arg("app")
        .status()
        .expect("Couldn't run g++. Where's g++ at?");
    if !enable_source {
        fs::remove_file("output.cpp").expect("Bob couldn't delete his temporary file");
    }
}
