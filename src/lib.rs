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
    IDENTITY_EXPECTED,
    TOKEN_EXPECTED,
}

pub struct Headers{
    pub iostream: bool,
    pub limits: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Variable_type{
    NUMBER,
    DECIMAL,
    STRING,
}
#[derive(PartialEq, Eq)]
pub enum Token_type{
    STRING_IDENTITY,
    NUMBER_IDENTITY,
    DECIMAL_IDENTITY,
    OPERATOR_PLUS,
    OTHER_OPERATOR_ARITHMETIC,
    STRING_LITERAL,
    DECIMAL_LITERAL,
    NUMBER_LITERAL,
}

pub struct Variable{
   pub variable_type: Variable_type,
   pub variable_name: String,
}

pub struct Token{
    pub token_name: String,
    pub token_type: Token_type,
}

impl PartialEq for Variable{
        fn eq(&self, other: &Variable) -> bool {
        *self.variable_name == *other.variable_name
    }
}

impl Eq for Variable{}

pub fn raise(err: Error){ 
    fs::remove_file("output.cpp").expect("Bob didn't see output.cpp");
    match err {
       Error::BOB_NOT_FOUND => panic!("Call Bob by name!"),
       Error::PERIOD_NOT_FOUND => panic!("Periods go at the end of each sentence!"),
       Error::VERB_EXPECTED => panic!("A verb is an action word. A function. A verb is what Bob needs to be told!"),
       Error::IDENTITY_TYPE_EXPECTED => panic!("Expected a variable type!"),
       Error::IDENTITY_EXISTS => panic!("The identity you're trying to declare already exists!"),
       Error::IDENTITY_EXPECTED => panic!("Name an identity!"),
       Error::TOKEN_EXPECTED => panic!("Token expected!"),
       }
}

pub token_assigner(query_vector: &mut Vec<String>,variable_stack: Vec<Variable>) -> Vec<(String,Token_type)>{
	let mut token_vector: Vec<(String,Token_type)> = vec::new();
	let temp: Variable_type = Variable_type::NUMBER;
	for query in query_vector{
		if variable_stack.iter().any(|j| {
			if query == j.variable_name
				temp = j.variable_type;
				true
				}){
			if temp == NUMBER{
				token_vector.push((query,Token_type::NUMBER_IDENTITY));
			}
			if temp == DECIMAL{
				token_vector.push((query,Token_type::DECIMAL_IDENTITY));
			}
			if temp == STRING{
				token_vector.push((query,Token_type::STRING_IDENTITY));
			}
		}
		else if match query{
			"+" => true,
			"-" => true,
			"*" => true,
			"/" => true,
			_ => false,
		}{
			token_vector.push((query,Token_type::OPERATOR));
		}
		else if query.as_bytes[0] as char == '"' && query.as_bytes[query.len() - 1] as char == '"'{
			token_vector.push((query,Token_type::STRING_LITERAL));
		}
		else if !query.parse::<f64>().is_err(){
			if !query.parse::<i32>().is_err(){
			token_vector.push((query,Token_type::NUMBER_LITERAL));
			}
			token_vector.push((query,Token_type::DECIMAL_LITERAL));
		}
		else{
			raise(Error::TOKEN_EXPECTED);
		}
	}
	token_vector
}
		

/// operators declared here and in token assigner. Iterates through the query(sentence) vector, assigns token types to each element and calls the respective functions.
pub fn iterator(query_vector: &mut Vec<String>,translated_file: &mut fs::File,headers: &mut Headers,variable_stack: &mut Vec<Variable>){
    *query_vector = query_vector.join(" ").replace("plus","+").split_whitespace().map(String::from).collect::<Vec<String>>();
    *query_vector = query_vector.join(" ").replace("minus","-").split_whitespace().map(String::from).collect::<Vec<String>>();
    *query_vector = query_vector.join(" ").replace("times","*").split_whitespace().map(String::from).collect::<Vec<String>>();
    *query_vector = query_vector.join(" ").replace("over","/").split_whitespace().map(String::from).collect::<Vec<String>>(); 
    *query_vector = query_vector.join(" ").replace("plus","+").split_whitespace().map(String::from).collect::<Vec<String>>();
    *query_vector = query_vector.join(" ").replace("modulo","%").split_whitespace().map(String::from).collect::<Vec<String>>();
   match query_vector[0].as_str(){
        "write" => {
		if query_vector[1].as_str()  == "line"{
			standard_function_declarations::write_to_stdout(true,translated_file,&token_assigner(query_vector[2..].to_vec()),headers);
		}
		else{		
		standard_function_declarations::write_to_stdout(false,translated_file,&token_assigner(query_vector[1..].to_vec()),headers);
                }
		}
	"read" => standard_function_declarations::read_from_stdin(translated_file,&query_vector[1],headers,variable_stack),
        "let" => standard_function_declarations::variable_assigner(translated_file,&query_vector[1..].to_vec(),variable_stack),
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

    temp_file.write_all(data.as_bytes()).expect("Unable to write to temp file");
    let mut buffer = [0u8; 4096];
    loop {
      let nbytes = translated_file.read(&mut buffer).expect("Unable to read from file");
      temp_file.write_all(&buffer[..nbytes]).expect("Unable to write buffer");
      if nbytes < buffer.len() { break; }
   }
    temp_file.write_all("}".as_bytes()).expect("Unable to write to temp file");
    fs::remove_file("output.cpp").expect("file op failed");
    fs::rename("swapper.cpp","output.cpp").expect("file op failed");
}

pub fn header_and_token_includer(headers: Headers){
    let mut data: String = String::from("//This is a temp\n");
    if headers.iostream == true{
        data += "#include<iostream>\n"
    }
    if headers.limits == true{
        data += "#include<limits>\n"
    }
    data += "int main(){\n";
    text_prepender_and_curly_appender(data);
}
