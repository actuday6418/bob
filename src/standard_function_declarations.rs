use std::fs;
use std::io::Write;

///-----------------HOW TO CREATE A NEW FUNCTION-------------
/// 1. Take possession of the keyword you want to use by adding to the match expression under
///    the iterator function in lib.rs.
///
/// 2. Define your function in this file, with the file handle to output.cpp as an
///    argument(translated_file). The argument names and function(verb) name, both in Bob and here, must be in lower case. You may use the same
///    function definition for similar Bob verbs. For example both write line and write Bob
///    functions are defined in write_to_stdout, differentiated only by a boolean argument(new_line).
///
/// 3. Add whatever header your C++ code depends on to the Headers struct in lib.rs. Initialize the header name (bool value) to false when the enum Headers is
///    instantiated in main.rs.
///
/// 4. Add a conditional to include your header file to the data variable under the
///    header_and_token_includer function in lib.rs. Any other information may be gleaned from the
///    existing functions.
///
/// 5. DOCUMENT your function, and you're done!
///---------------------------------------------------------

pub fn write_to_stdout(
    new_line: bool,
    translated_file: &mut fs::File,
    expression_type_vector_and_expressions: &(Vec<crate::Expression_type>, Vec<String>),
    headers: &mut crate::Headers,
) {
    //!-----------WHASSDIS-------------
    //! Accepts a string or expression that is evaluated and written to stdout. Defines both the
    //! write and write_line Bob functions.
    for expression_type in &expression_type_vector_and_expressions.0 {
        let mut final_string: String = String::new();
        if *expression_type == crate::Expression_type::NUMERIC {
            let argument = expression_type_vector_and_expressions.1.join("");
            final_string = argument.to_string().clone();
            if new_line {
                final_string += "<<std::endl";
            }
        } else if *expression_type == crate::Expression_type::STRING {
            final_string = expression_type_vector_and_expressions.1.join("");
            if new_line {
                final_string += "<<std::endl";
            }
        } else {
            crate::raise(crate::Error::INVALID_EXPRESSION);
        }
        headers.iostream = true;
        (*translated_file)
            .write_all("std::cout<<".as_bytes())
            .expect("Write to output.cpp failed!");
        (*translated_file)
            .write_all(final_string.as_bytes())
            .expect("Write to output.cpp failed!");
        (*translated_file)
            .write_all(";\n".as_bytes())
            .expect("Write to output.cpp failed!");
    }
}

pub fn read_from_stdin(
    translated_file: &mut fs::File,
    variable_name: &String,
    headers: &mut crate::Headers,
    variable_stack: &Vec<crate::Variable>,
) {
    //!--------WHASSDIS--------
    //! Accepts a string (variable name), checks if it's included in the variable stack. If it is, data is read from stdin to it.

    let mut variable_type: crate::Variable_type = crate::Variable_type::NUMBER;
    //Check if the variable is in the stack
    if variable_stack.iter().any(|i| {
        if i.variable_name == variable_name.as_str() {
            variable_type = i.variable_type.clone();
            true
        } else {
            false
        }
    }) {
        headers.iostream = true;
        headers.limits = true;
        if variable_type == crate::Variable_type::STRING {
            (*translated_file)
                .write_all("getline(std::cin,".as_bytes())
                .expect("Write to output.cpp failed!");
            (*translated_file)
                .write_all(variable_name.as_bytes())
                .expect("Write to output.cpp failed!");
            (*translated_file)
                .write_all(");\n".as_bytes())
                .expect("Write to output.cpp failed!");
        } else {
            (*translated_file)
                .write_all("while(true){std::cin>>".as_bytes())
                .expect("Write to output.cpp failed!");
            (*translated_file)
                .write_all(variable_name.as_bytes())
                .expect("Write to output.cpp failed!");
            if variable_type == crate::Variable_type::NUMBER {
                (*translated_file).write_all((";\nif(std::cin.fail()){\nstd::cin.clear();\nstd::cin.ignore(std::numeric_limits<std::streamsize>::max(),".to_owned() + r" '\n');" + "\nstd::cout<<\"Tell Bob a number\"<<std::endl;\n}\nelse\nbreak;\n}\n").as_bytes())
                		.expect("Write to output.cpp failed!");
            } else if variable_type == crate::Variable_type::DECIMAL {
                (*translated_file).write_all((";\nif(std::cin.fail()){\nstd::cin.clear();\nstd::cin.ignore(std::numeric_limits<std::streamsize>::max(),".to_owned() + r" '\n');" + "\nstd::cout<<\"Tell Bob a decimal number!\"<<std::endl;\n}\nelse\nbreak;\n}\n").as_bytes())
              			.expect("Write to output.cpp failed!");
            }
        }
    } else {
        crate::raise(crate::Error::IDENTITY_EXPECTED);
    }
}

pub fn variable_declarer(
    translated_file: &mut fs::File,
    expression_type_vector_and_expressions: &(Vec<crate::Expression_type>, Vec<String>),
    headers: &mut crate::Headers,
    variable_stack: &mut Vec<crate::Variable>,
) {
    //!--------WHASSDIS--------
    //! Checks if the variable name is part of the variable stack, and if it isn't, adds it to it and
    //! declares the variable in C++.

    let mut valid: bool = false;
    let mut variable_type: crate::Variable_type = crate::Variable_type::NUMBER;
    let mut expression_index: usize = 0;
    for expression_type in &expression_type_vector_and_expressions.0 {
        if *expression_type == crate::Expression_type::DECLARER_NUMBER
        {
            (*translated_file)
                    .write_all("int ".as_bytes())
                    .expect("Write to output.cpp failed!");
                valid = true;
                variable_type = crate::Variable_type::NUMBER;
        }
        else if *expression_type == crate::Expression_type::DECLARER_DECIMAL {
                (*translated_file)
                    .write_all("float ".as_bytes())
                    .expect("Write to output.cpp failed!");
                valid = true;
                variable_type = crate::Variable_type::DECIMAL;
        }
        else if *expression_type == crate::Expression_type::DECLARER_STRING {
                    (*translated_file)
                        .write_all("std::string ".as_bytes())
                        .expect("Write to output.cpp failed!");
                    headers.string = true;
                    valid = true;
                    variable_type = crate::Variable_type::STRING;
                }
        if valid {
            let variable = crate::Variable {variable_type: variable_type, variable_name: expression_type_vector_and_expressions.1[expression_index].split_whitespace().map(String::from).collect::<Vec<String>>()[0].clone()};
            (*translated_file)
                .write_all(variable.variable_name.as_bytes())
                .expect("Write to output.cpp failed!");
            (*translated_file)
                .write_all(";\n".as_bytes())
                .expect("Write to output.cpp failed!");
            variable_stack.push(variable);
        }
        else {
            crate::raise(crate::Error::INVALID_EXPRESSION);
        }
        expression_index += 1;
}
}
