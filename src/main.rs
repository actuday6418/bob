mod lexical_analysis;
mod lib;
mod standard_function_declarations;
use std::env;
use std::fs;

fn main() {
    let mut query = String::new();
    let translated_file = fs::File::create("output.cpp").expect("File creatioin failed");
    let mut headers: lib::Headers = lib::Headers{iostream: false};
    let arg: Vec<String> = env::args().collect();
    let query: String = fs::read_to_string(&arg[1])
        .expect("Bob can't see that file!");
    let mut query = &(query.trim());
    let mut query: String = lexical_analysis::comment_remover(query);
    let mut query: String = lexical_analysis::string_space_remover(query);
    let mut query: String = lexical_analysis::bob_and_punctuation_remover(query);
    println!("{}",query);
    let mut query_vector: Vec<String> = query.split_whitespace().map(String::from).collect();
    lib::iterator(&mut query_vector,& translated_file, &mut headers);
}
