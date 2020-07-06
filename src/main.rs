use std::env;
use std::fs;
use std::process;
use bob::lexical_analysis;

///-----------------------------DOCS-----------------------------------
/// 1. Head to standard_function_declarations to define a new function.
///
/// 2. lib defines a couple useful functions
///
///--------------------------------------------------------------------

fn main() {
    let mut query = String::new();
    let mut translated_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("output.cpp")
        .expect("File creation failed");
    let mut headers: bob::Headers = bob::Headers{iostream: false};
    let arg: Vec<String> = env::args().collect();
    let query: String = fs::read_to_string(&arg[1])
        .expect("Bob can't see that file!");
    let mut query = &(query.trim());
    let mut query: String = lexical_analysis::comment_remover(query);
    let mut query: String = lexical_analysis::string_space_remover(query);
    let mut query: String = lexical_analysis::bob_and_punctuation_remover(query);
    let mut query_vector: Vec<String> = query.split_whitespace().map(String::from).collect();
    bob::iterator(&mut query_vector,&mut translated_file, &mut headers);
    bob::header_and_token_includer(headers);
    process::Command::new("g++")
        .arg("output.cpp")
        .arg("-o")
        .arg("app")
        .status()
        .expect("Couldn't run g++");
    fs::remove_file("output.cpp");
}
