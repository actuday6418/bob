use std::fs;
use std::io::Write;

///-----------------HOW TO CREATE A NEW FUNCTION-------------
/// 1. Take possession of the keyword you want to use by adding to the match expression under
///    the iterator function in lib.rs.
///
/// 2. Define your function in this file, with the file handle to output.cpp as an
///    argument(translated_file).
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

pub fn write_to_stdout(translated_file: &mut fs::File,argument: &str, headers: &mut crate::Headers){
    
    ///-----------WHASSDIS-------------
    /// Accepts a string or expression that is evaluated and written to stdout.

    let mut final_string: String = String::new();
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cout<<".as_bytes())
        .expect("Write to output.cpp failed!");
    if argument.as_bytes()[0] as char== '"'{
        let mut temp = String::from(argument.replace("_"," "));
        final_string = temp;
    }
    (*translated_file).write_all(final_string.as_bytes())
        .expect("Write to output.cpp failed!");
    (*translated_file).write_all(";\n".as_bytes())
        .expect("Write to output.cpp failed!");
}

pub fn read_from_stdin(translated_file: &mut fs::File,variable_name: &String,headers: &mut crate::Headers,variable_stack: & Vec<crate::Variable>){

    ///--------WHASSDIS--------
    /// Accepts a string (variable name), checks if it's included in the variable stack. If it is, data is read from stdin to it.

    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cin>>".as_bytes())
        .expect("Write to output.cpp failed!");
    (*translated_file).write_all(variable_name.as_bytes())
        .expect("Write to output.cpp failed!");
    (*translated_file).write_all(";\n".as_bytes())
        .expect("Write to output.cpp failed!");
}

pub fn variable_assigner(translated_file: &mut fs::File,variable_name: &String,variable_type: &String,variable_stack: &mut Vec<crate::Variable>){

    ///--------WHASSDIS--------
    /// Checks if the variable name is part of the variable stack, and if it isn't, adds it to it and
    /// declares the variable in C++.
    let variable_type = match variable_type.as_str(){
        "number" => crate::Variable_type::NUMBER,
        "decimal" => crate::Variable_type::DECIMAL,
        "string" => crate::Variable_type::STRING,
    };
    let variable: crate::Variable = crate::Variable{ variable_type: variable_type, variable_name: *variable_name };
    if variable_stack.iter().any(|&i| i == variable){
        crate::raise(crate::Error::IDENTITY_EXISTS);
    }
    else{
        variable_stack.push(variable);
        match variable_type{
            crate::Variable_type::NUMBER => (*translated_file).write_all("int ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
            crate::Variable_type::DECIMAL => (*translated_file).write_all("float ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
            crate::Variable_type::STRING => (*translated_file).write_all("std::string ".as_bytes())
                                           .expect("Write to output.cpp failed!"),
        }
        (*translated_file).write_all(variable_name.as_bytes())
          .expect("Write to output.cpp failed!");
        (*translated_file).write_all(";\n".as_bytes())
          .expect("Write to output.cpp failed!");
    }
}




