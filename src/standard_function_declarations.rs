use std::fs;
use std::io::Write;

///-----------------HOW TO CREATE A NEW FUNCTION-------------
/// 1. Take possession of the keyword you want to use by adding to the match expression under
///    the iterator function in lib.rs.
///
/// 2. Define your function in this file, with the file handle to output.cpp as an
///    argument(translated_file). The argument names here and function(verb) name, both in Bob and here, must be in snake case. You may use the same 
///    function definition for similar Bob verbs. For example both write_line and write Bob
///    functions are defined in write_to_stdout, differentiated only by a boolean argument(new_line).
/// 
/// 3. Add whatever header your C++ code depends on to the Headers struct in lib.rs. Initialize the header name to false when the enum Headers is
///    instantiated in main.rs.
/// 
/// 4. Add a conditional to include your header file to the data variable under the
///    header_and_token_includer function in lib.rs. Any other information may be gleaned from the
///    existing functions. 
/// 
/// 5. DOCUMENT your function, and you're done!
///---------------------------------------------------------

pub fn write_to_stdout(new_line: bool,translated_file: &mut fs::File,argument_vector: & Vec<String>, headers: &mut crate::Headers,variable_stack: & Vec<crate::Variable>){
    //!-----------WHASSDIS-------------
    //! Accepts a string or expression that is evaluated and written to stdout. Defines both the
    //! write and write_line Bob functions. 

    let mut final_string: String = String::new();
    if argument_vector.iter()
        .all(|i| (variable_stack.iter().any(|j| j.variable_name == *i) || vec!["+".to_string(),"-".to_string()].iter().any(|j| j == i))){
        let argument = argument_vector.join("");
            final_string = argument.to_string().clone();
            if new_line{
                final_string += "<<std::endl";
            }
    }
    if argument_vector.len() == 1 && argument_vector[0].as_bytes()[0] as char== '"'{
        let mut temp = String::from(argument_vector[0].replace("_"," "));
        if new_line {
            temp = temp[ .. temp.len() - 1].to_string() + r"\n" + "\"";
        }
        final_string = temp;
    }
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cout<<".as_bytes())
        .expect("Write to output.cpp failed!");
    (*translated_file).write_all(final_string.as_bytes())
        .expect("Write to output.cpp failed!");
    (*translated_file).write_all(";\n".as_bytes())
        .expect("Write to output.cpp failed!"); 
}

pub fn read_from_stdin(translated_file: &mut fs::File,variable_name: &String,headers: &mut crate::Headers,variable_stack: & Vec<crate::Variable>){

    //!--------WHASSDIS--------
    //! Accepts a string (variable name), checks if it's included in the variable stack. If it is, data is read from stdin to it.
    
    let mut variable_type: crate::Variable_type = crate::Variable_type::NUMBER;
    if variable_stack.iter().any(|i| {
            if i.variable_name == variable_name.as_str() {
                variable_type = i.variable_type.clone();
                true
            }
            else{
                false
            }
    }
            ){ // !! Pass the data type also and display relevant error message !!
        if headers.iostream == false{
            headers.iostream = true;
        }
        if headers.limits == false{
            headers.limits = true;
        }
        (*translated_file).write_all("while(true){std::cin>>".as_bytes())
            .expect("Write to output.cpp failed!");
        (*translated_file).write_all(variable_name.as_bytes())
            .expect("Write to output.cpp failed!");
        if variable_type == crate::Variable_type::NUMBER{
            (*translated_file).write_all((";\nif(std::cin.fail()){\nstd::cin.clear();\nstd::cin.ignore(std::numeric_limits<std::streamsize>::max(),".to_owned() + r" '\n');" + "\nstd::cout<<\"Tell Bob a number\"<<std::endl;\n}\nelse\nbreak;\n}\n").as_bytes())
                .expect("Write to output.cpp failed!");
        }
        else if variable_type == crate::Variable_type::DECIMAL{
            (*translated_file).write_all((";\nif(std::cin.fail()){\nstd::cin.clear();\nstd::cin.ignore(std::numeric_limits<std::streamsize>::max(),".to_owned() + r" '\n');" + "\nstd::cout<<\"Tell Bob a decimal number!\"<<std::endl;\n}\nelse\nbreak;\n}\n").as_bytes())
                .expect("Write to output.cpp failed!");
        }
    }
    else{ 
        crate::raise(crate::Error::IDENTITY_EXPECTED);
    }
}

pub fn variable_assigner(translated_file: &mut fs::File,argument_vector: & Vec<String>,variable_stack: &mut Vec<crate::Variable>){

    //!--------WHASSDIS--------
    //! Checks if the variable name is part of the variable stack, and if it isn't, adds it to it and
    //! declares the variable in C++.
    
    if argument_vector[1] == "be".to_string() && (argument_vector[2] == "a" || argument_vector[2] == "an"){
    let variable_name = argument_vector[0].clone();
    let variable_type = argument_vector[3].clone();
    let variable_type = match variable_type.as_str(){
        "number" => crate::Variable_type::NUMBER,
        "decimal" => crate::Variable_type::DECIMAL,
        "string" => crate::Variable_type::STRING,
        _ => {
            crate::raise(crate::Error::IDENTITY_TYPE_EXPECTED);
            crate::Variable_type::NUMBER
        }
    };
    let variable: crate::Variable = crate::Variable{ variable_type: variable_type, variable_name: variable_name.clone().to_string() };
    if variable_stack.iter().any(|i| *i == variable){
        crate::raise(crate::Error::IDENTITY_EXISTS);
    }
    else{
        match variable.variable_type{
            crate::Variable_type::NUMBER => (*translated_file).write_all("int ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
            crate::Variable_type::DECIMAL => (*translated_file).write_all("float ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
            crate::Variable_type::STRING => (*translated_file).write_all("std::string ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
        }
        (*translated_file).write_all(variable.variable_name.as_bytes())
          .expect("Write to output.cpp failed!");
        (*translated_file).write_all(";\n".as_bytes())
          .expect("Write to output.cpp failed!");
        variable_stack.push(variable);
    }
    }
    else{
        crate::raise(crate::Error::VERB_EXPECTED);
    }
}
