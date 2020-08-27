#![allow(non_camel_case_types)]
pub mod lexical_analysis;
pub mod standard_function_declarations;
use std::fs;
use std::io::Read;
use std::io::Write;

pub enum Error {
    BOB_NOT_FOUND,
    PERIOD_NOT_FOUND,
    VERB_EXPECTED,
    IDENTITY_TYPE_EXPECTED,
    IDENTITY_EXISTS,
    IDENTITY_EXPECTED,
    TOKEN_EXPECTED,
    INVALID_EXPRESSION,
    MALFORMED_COMMENT,
}

pub struct Headers {
    pub iostream: bool,
    pub limits: bool,
    pub string: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Variable_type {
    NUMBER,
    DECIMAL,
    STRING,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Expression_type {
    STRING,
    NUMERIC,
    ASSIGNER_STRING,
    ASSIGNER_DECIMAL,
    ASSIGNER_NUMBER,
    DECLARER_STRING,
    DECLARER_DECIMAL,
    DECLARER_NUMBER,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Token_type {
    STRING_IDENTITY,
    NUMBER_IDENTITY,
    DECIMAL_IDENTITY,
    OPERATOR_PLUS,
    OTHER_OPERATOR_ARITHMETIC,
    STRING_LITERAL,
    DECIMAL_LITERAL,
    NUMBER_LITERAL,
    TO_BE_IDENTITY,
    OPERATOR_ASSIGNMENT,
    TYPE_STRING,
    TYPE_NUMBER,
    TYPE_DECIMAL,
}

pub struct Variable {
    pub variable_type: Variable_type,
    pub variable_name: String,
}

pub struct Token {
    pub token_name: String,
    pub token_type: Token_type,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Variable) -> bool {
        *self.variable_name == *other.variable_name
    }
}

impl Eq for Variable {}

/// Error handler for the whole compiler.
pub fn raise(err: Error) {
    fs::remove_file("output.cpp").expect("Bob didn't see output.cpp");
    match err {
        Error::BOB_NOT_FOUND => panic!("Call Bob by name!"),
        Error::PERIOD_NOT_FOUND => panic!("Periods go at the end of each sentence!"),
        Error::VERB_EXPECTED => {
            panic!("A verb is an action word. A function. A verb is what Bob needs to be told!")
        }
        Error::IDENTITY_TYPE_EXPECTED => panic!("Expected a variable type!"),
        Error::IDENTITY_EXISTS => panic!("The identity you're trying to declare already exists!"),
        Error::IDENTITY_EXPECTED => panic!("Name an identity!"),
        Error::TOKEN_EXPECTED => panic!("Token expected!"),
        Error::INVALID_EXPRESSION => panic!("Expression isn't valid"),
        Error::MALFORMED_COMMENT => panic!("Check you parantheses!"),
    }
}

/// Assigns type to each entity in the Bob source so function writers find things easier to do.
/// Also restores the space in strings. (Earlier converted to underscore.)
pub fn token_assigner(
    query_string: &mut String,
    variable_stack: &mut Vec<Variable>,
) -> Vec<(String, Token_type)> {
    let query_vector: Vec<String> = query_string.split(' ').map(str::to_string).collect();
    let mut token_vector: Vec<(String, Token_type)> = Vec::new();
    let mut temp: Variable_type = Variable_type::NUMBER;
    for query in &query_vector {
        if variable_stack.iter().any(|j| {
            if *query == j.variable_name {
                temp = j.variable_type;
                true
            } else {
                false
            }
        }) {
            if temp == Variable_type::NUMBER {
                token_vector.push((query.to_string(), Token_type::NUMBER_IDENTITY));
            } else if temp == Variable_type::DECIMAL {
                token_vector.push((query.to_string(), Token_type::DECIMAL_IDENTITY));
            } else if temp == Variable_type::STRING {
                token_vector.push((query.to_string(), Token_type::STRING_IDENTITY));
            }
        } else if match query.as_str() {
            "-" => true,
            "*" => true,
            "/" => true,
            ")" => true,
            "(" => true,
            "%" => true,
            _ => false,
        } {
            token_vector.push((query.to_string(), Token_type::OTHER_OPERATOR_ARITHMETIC));
        } else if query == "string" {
            token_vector.push((query.to_string(), Token_type::TYPE_STRING));
        } else if query == "number" {
            token_vector.push((query.to_string(), Token_type::TYPE_NUMBER));
        } else if query == "decimal" {
            token_vector.push((query.to_string(), Token_type::TYPE_DECIMAL));
        } else if query == "+" {
            token_vector.push((query.to_string(), Token_type::OPERATOR_PLUS));
        } else if query == "be" {
            token_vector.push((query.to_string(), Token_type::OPERATOR_ASSIGNMENT));
        } else if query.as_bytes()[0] as char == '"'
            && query.as_bytes()[query.to_string().len() - 1] as char == '"'
        {
            token_vector.push((query.to_string(), Token_type::STRING_LITERAL));
        } else if !query.parse::<f64>().is_err() {
            if !query.parse::<i32>().is_err() {
                token_vector.push((query.to_string(), Token_type::NUMBER_LITERAL));
            } else {
                token_vector.push((query.to_string(), Token_type::DECIMAL_LITERAL));
            }
        } else {
            token_vector.push((query.to_string(), Token_type::TO_BE_IDENTITY));
        }
    }
    *query_string = query_vector.join(" ");
    token_vector
}

/// operators declared here and in token assigner. Iterates through the query(sentence) vector, assigns token types to each element and calls the respective functions.
pub fn iterator(
    query: &mut String,
    translated_file: &mut fs::File,
    headers: &mut Headers,
    variable_stack: &mut Vec<Variable>,
) {
    *query = query.replace("plus", "+");
    *query = query.replace("minus", "-");
    *query = query.replace("times", "*");
    *query = query.replace("over", "/");
    *query = query.replace("plus", "+");
    *query = query.replace("modulo", "%");
    let mut query_vector: Vec<String> = query
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();
    //Put the second word of all two worded standard functions here
    let expression_and_type = if query_vector[1] == "line" {(lexical_analysis::expression_parser(
            &mut query_vector[2..].to_vec(),
            variable_stack,
            )).to_owned()
    }
    else {
 (lexical_analysis::expression_parser(
                        &mut query_vector[1..].to_vec(),
                        variable_stack,
                    )).to_owned()

    };
    let expression_and_type = &expression_and_type;
let mut i: usize = 0;
    match query_vector[0].as_str() {
        "write" => {
            if query_vector[1].as_str() == "line" {
                while i<expression_and_type.0.len(){
                standard_function_declarations::write_to_stdout(
                    true,
                    translated_file,
                    (expression_and_type.0[i],expression_and_type.1[i].clone())
,
                    headers,
                );
                i += 1;}
            } else {
                while i < expression_and_type.0.len() {
                standard_function_declarations::write_to_stdout(
                    false,
                    translated_file,
                        (expression_and_type.0[i],expression_and_type.1[i].clone()),

                    headers,
                );
                i += 1;
                }
            }
        },
        "read" => standard_function_declarations::read_from_stdin(
            translated_file,
            &query_vector[1],
            headers,
            variable_stack,
        ),
        "let" => while i<expression_and_type.0.len() {
            standard_function_declarations::variable_declarer(
            translated_file,
            (expression_and_type.0[i],expression_and_type.1[i].clone())
,
            headers,
            variable_stack,
        );
            i += 1;
        },
        _ => raise(Error::VERB_EXPECTED),
    }}

fn text_prepender_and_curly_appender(data: String) {
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

    temp_file
        .write_all(data.as_bytes())
        .expect("Unable to write to temp file");
    let mut buffer = [0u8; 4096];
    loop {
        let nbytes = translated_file
            .read(&mut buffer)
            .expect("Unable to read from file");
        temp_file
            .write_all(&buffer[..nbytes])
            .expect("Unable to write buffer");
        if nbytes < buffer.len() {
            break;
        }
    }
    temp_file
        .write_all("}".as_bytes())
        .expect("Unable to write to temp file");
    fs::remove_file("output.cpp").expect("file op failed");
    fs::rename("swapper.cpp", "output.cpp").expect("file op failed");
}

pub fn header_and_token_includer(headers: Headers) {
    let mut data: String = String::from("//This is a temp\n");
    if headers.iostream == true {
        data += "#include<iostream>\n"
    }
    if headers.limits == true {
        data += "#include<limits>\n"
    }
    if headers.string == true {
        data += "#include<string>\n"
    }
    data += "int main(){\n";
    text_prepender_and_curly_appender(data);
}
