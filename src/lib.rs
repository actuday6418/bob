#![allow(non_camel_case_types)]
pub mod standard_function_declarations;
pub mod lexical_analysis;
use std::fs;
use std::io::Write;
use std::io::Read;

pub enum Error{
    BOB_NOT_FOUND,
    PERIOD_NOT_FOUND,
    VERB_EXPECTED,
    IDENTITY_TYPE_EXPECTED,
    IDENTITY_EXISTS,
}

pub struct Headers{
    pub iostream: bool,
}

pub enum Variable_type{
    NUMBER,
    DECIMAL,
    STRING,
}

pub struct Variable{
   pub variable_type: Variable_type,
   pub variable_name: String,
}

impl PartialEq for Variable{
        fn eq(&self, other: &Variable) -> bool {
        *self.variable_name == *other.variable_name
    }
}
impl Eq for Variable{}

pub fn raise(err: Error){
    match err {
       Error::BOB_NOT_FOUND => println!("Call Bob by name!"),
       Error::PERIOD_NOT_FOUND => println!("Periods go at the end of each sentence!"),
       Error::VERB_EXPECTED => println!("A verb is an action word. A function. A verb is what Bob needs to be told!"),
       Error::IDENTITY_TYPE_EXPECTED => println!("Expected a variable type!"),
       Error::IDENTITY_EXISTS => println!("The identity you're trying to declare already exists!"),
       }
}

pub fn iterator(query_vector: &mut Vec<String>,translated_file: &mut fs::File,headers: &mut Headers,variable_stack: &mut Vec<Variable>){
    match query_vector[0].as_str(){
        "write" => standard_function_declarations::write_to_stdout(translated_file,&query_vector[1],headers),
        "read" => standard_function_declarations::read_from_stdin(translated_file,&query_vector[1],headers,variable_stack),
        "let" => standard_function_declarations::variable_assigner(translated_file,&query_vector[1],&query_vector[4],variable_stack),
        _ => raise(Error::VERB_EXPECTED),
    }

}

fn text_prepender_and_curly_appender(data: String){
    let mut temp_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("swapper.cpp")
        .expect("Unable to create a temporary resource");   
   let mut translated_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open("output.cpp")
        .expect("Unable to create a temporary resource");   

    temp_file.write_all(data.as_bytes());
    let mut buffer = [0u8; 4096];
    loop {
      let nbytes = translated_file.read(&mut buffer).expect("Unable to read from file");
      temp_file.write_all(&buffer[..nbytes]).expect("Unable to write buffer");
      if nbytes < buffer.len() { break; }
   }
    temp_file.write_all("}".as_bytes());
    fs::remove_file("output.cpp");
    fs::rename("swapper.cpp","output.cpp");
}

pub fn header_and_token_includer(headers: Headers){
    let mut data: String = String::from("//This is a temp\n");
    if headers.iostream == true{
        data += "#include<iostream>\n"
    }
    data += "int main(){\n";
    text_prepender_and_curly_appender(data);
}
