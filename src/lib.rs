pub enum Error{
    BOB_NOT_FOUND,
    PERIOD_NOT_FOUND,
}

pub enum Headers{
    IOSTREAM,
}

pub fn raise(err: Error){
    match err {
       Error::BOB_NOT_FOUND => println!("Bob not found"),
       Error::PERIOD_NOT_FOUND => println!("Period not found"),
       }
}

pub static function_keywords: Vec<String> = Vec::new();


