use std::fs;
use std::io::Write;
use lib;

pub fn write(translated_file: &mut fs::File,argument: &str, headers: &mut lib::Headers){
    let mut final_data: String =String::new();
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cout<<".as_bytes());
    if argument.as_bytes()[0] as char== '"'{
        let mut temp = String::from(argument.replace("_"," "));
        final_data = temp;
    }
    println!("{}",final_data);
    (*translated_file).write_all(final_data.as_bytes());
    (*translated_file).write_all(";\n".as_bytes());
}
