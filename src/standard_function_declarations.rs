use std::fs;
use std::io::Write;
use lib;

pub fn write(translated_file: &fs::File,argument: String,headers: &mut lib::Headers){
    if headers.iostream == false{
        headers.iostream = true;
    }
    (*translated_file).write_all("std::cout<<".as_bytes());
}
