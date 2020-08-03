///-----------------Everything that has to do with verifying and pre-processing syntax goes here-----

pub fn string_space_remover_and_bracket_replacer(query: String) -> String{
    let mut i: usize = 0;
    let mut j: usize;
    let mut flag: bool;
    let mut string_vec: Vec<(usize,usize)> = Vec::new();
    while i< query.len() {
        if query.as_bytes()[i] as char == '"' {
            j = i;
            flag = true;
            while j< query.len() {
                if query.as_bytes()[j] as char == '"'{
                    flag = false;
                    string_vec.push((i,j));
                    i = j;
                }
                j= j+1;
            }
                if flag {
                    string_vec.push((i,query.len()));
                    i = query.len();
                }
        }
        i = i+1;
    }
    let mut query = query.to_string();
    for x in string_vec{
      let mut t = query[x.0 .. x.1].replace(" ","_"); 
      t.insert_str(0,&query[.. x.0]);
      t.insert_str(x.1,&query[x.1 ..]);
      query = t;
    }
    query = query.replace("}",")");
    query = query.replace("{","(");
    query
}

pub fn comment_remover(query: &str) -> String{
    let mut i: usize = 0;
    let mut j: usize;
    let mut flag: bool;
    let mut comment_vec: Vec<(usize,usize)> = Vec::new();
    while i < query.len() {
        if query.as_bytes()[i] as char == '(' {
            j = i;
            flag = true;
            while j< query.len() {
                if query.as_bytes()[j] as char == ')'{
                    flag = false;
                    comment_vec.push((i,j));
                    i = j;
                }
                j= j+1;
            }
                if flag {
                    comment_vec.push((i,query.len()));
                    i = query.len();
                }
        }
        i = i+1;
    }
    let mut query = String::from(query);
    for x in comment_vec{
    query.replace_range(x.0 .. x.1 + 1,"");
    }
    query
}

pub fn bob_and_punctuation_remover(query: String) -> String{
    let mut temp: String = "".to_string();
    let mut number_spaces: usize = 0;
    while query.as_bytes()[number_spaces] as char== ' '{
        number_spaces += 1;
    }
    if &query[number_spaces..number_spaces + 4] == "Bob "{
        temp = query[number_spaces + 4 .. ].to_string();
    }
    else{
        crate::raise(crate::Error::BOB_NOT_FOUND);
    }
    if &query[query.len() - 1 ..] == "."{
        temp = temp[..temp.len() - 1].to_string();
    }
    else{
        crate::raise(crate::Error::PERIOD_NOT_FOUND);
    }
    temp
}

