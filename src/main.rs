use std::env;
use std::fs;
use std::io::BufRead;
use std::process;
use bob::lexical_analysis;
use std::io;

///-----------------------------DOCS-----------------------------------
/// 1. Head to standard_function_declarations to define a new function.
///
/// 2. lib defines a couple useful functions
///
///--------------------------------------------------------------------

fn main() {
    let mut translated_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("output.cpp")
        .expect("File creation failed");
    let mut headers: bob::Headers = bob::Headers{iostream: false,limits: false};
    let arg: Vec<String> = env::args().collect();
    // uses a reader buffer
    let reader = fs::File::open(&arg[1]).expect("Couldn't open that file");
    let reader = io::BufReader::new(reader);
    let mut variable_stack: Vec<bob::Variable> = Vec::new();
    for query in reader.lines() { // !! Is this the most efficient way to iterate through each line in the source? !!
            let query = query.unwrap();
            let query = &(query.trim());
            let query: String = lexical_analysis::comment_remover(query);
            let query: String = lexical_analysis::string_space_remover(query);
            let mut query: String = lexical_analysis::bob_and_punctuation_remover(query);
            let mut query_vector: Vec<String> = query.split_whitespace().map(String::from).collect::<Vec<String>>();
            bob::iterator(&mut query_vector,&mut translated_file, &mut headers,&mut variable_stack);
            query.clear();
        }
    bob::header_and_token_includer(headers);
    process::Command::new("g++")
        .arg("output.cpp")
        .arg("-o")
        .arg("app")
        .status()
        .expect("Couldn't run g++. Where's g++?");
   fs::remove_file("output.cpp").expect("Bob couldn't delete his temporary file");
}
