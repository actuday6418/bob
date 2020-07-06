use std::fs;
use std::io::Write;

///-----------------HOW TO CREATE A NEW FUNCTION-------------
/// 1. Take possession of the keyword you want to use by adding to the match expression under
///    the iterator function in lib.rs.
///
/// 2. Define your function in this file,with the file handle to output.cpp as an
///    argument(translated_file).
/// 
/// 3. Add whatever header your C++ code depends on to the Headers struct in lib.rs. Initialize the header name to false when the enum Headers is
///    instantiated in main.rs
/// 
/// 4. Add a conditional to include your header file to the data variable under the
///    header_and_token_includer function in lib.rs. Any other information may be gleaned from the
///    existing functions. 
/// 
/// 5. DOCUMENT your function, and you're done!
///---------------------------------------------------------

pub fn write(translated_file: &mut fs::File,argument: &str, headers: &mut crate::Headers){
    
    ///-----------WHASSDIS-------------
    /// Accepts a string or expression that is evaluated and written to stdout.

    let mut final_string: String = String::new();
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cout<<".as_bytes());
    if argument.as_bytes()[0] as char== '"'{
        let mut temp = String::from(argument.replace("_"," "));
        final_string = temp;
    }
    (*translated_file).write_all(final_string.as_bytes());
    (*translated_file).write_all(";\n".as_bytes());
}

pub fn read(translated_file: &mut fs::File,var_name: String, headers: &mut crate::Headers){

    ///--------WHASSDIS--------
    /// Accepts a string (variable name) to which data is read from stdin.

    let mut final_string: String = String::new();
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cin>>".as_bytes());
    (*translated_file).write_all(var_name.as_bytes());
    (*translated_file).write_all(";\n".as_bytes());
}


