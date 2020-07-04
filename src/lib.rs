use std::fs;
use standard_function_declarations;

pub enum Error{
    BOB_NOT_FOUND,
    PERIOD_NOT_FOUND,
    VERB_EXPECTED,
}

pub struct Headers{
    pub iostream: bool,
}

pub fn raise(err: Error){
    match err {
       Error::BOB_NOT_FOUND => println!("Call Bob by name!"),
       Error::PERIOD_NOT_FOUND => println!("Periods go at the end of each sentence!"),
       Error::VERB_EXPECTED => println!("A verb is an action word. A function. A verb is what Bob needs to be told!"),
       }
}

pub fn iterator(query_vector: &mut Vec<String>,translated_file: &fs::File,headers: &mut Headers){
    match query_vector[0].as_str(){
        "write" => standard_function_declarations::write(&translated_file, query_vector[1], &mut headers),
        _ => raise(Error::VERB_EXPECTED),
    }

}
