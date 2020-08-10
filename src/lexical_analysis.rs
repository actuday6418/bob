///-----------------Everything that has to do with verifying and pre-processing syntax goes here-----

pub fn string_space_remover_and_bracket_replacer(query: String) -> String {
    let mut i: usize = 0;
    let mut j: usize;
    let mut flag: bool;
    let mut string_vec: Vec<(usize, usize)> = Vec::new();
    while i < query.len() {
        if query.as_bytes()[i] as char == '"' {
            j = i;
            flag = true;
            while j < query.len() {
                if query.as_bytes()[j] as char == '"' {
                    flag = false;
                    string_vec.push((i, j));
                    i = j;
                }
                j = j + 1;
            }
            if flag {
                string_vec.push((i, query.len()));
                i = query.len();
            }
        }
        i = i + 1;
    }
    let mut query = query.to_string();
    for x in string_vec {
        let mut t = query[x.0..x.1].replace(" ", "_");
        t.insert_str(0, &query[..x.0]);
        t.insert_str(x.1, &query[x.1..]);
        query = t;
    }
    query = query.replace("}", ")");
    query = query.replace("{", "(");
    query
}

pub fn comment_remover(query: &str) -> String {
    let mut i: usize = 0;
    let mut j: usize;
    let mut flag: bool;
    let mut comment_vec: Vec<(usize, usize)> = Vec::new();
    while i < query.len() {
        if query.as_bytes()[i] as char == '(' {
            j = i;
            flag = true;
            while j < query.len() {
                if query.as_bytes()[j] as char == ')' {
                    flag = false;
                    comment_vec.push((i, j));
                    i = j;
                }
                j = j + 1;
            }
            if flag {
                comment_vec.push((i, query.len()));
                i = query.len();
            }
        }
        i = i + 1;
    }
    let mut query = String::from(query);
    for x in comment_vec {
        query.replace_range(x.0..x.1 + 1, "");
    }
    query
}

pub fn bob_and_punctuation_remover(query: String) -> String {
    let mut temp: String = "".to_string();
    let mut number_spaces: usize = 0;
    while query.as_bytes()[number_spaces] as char == ' ' {
        number_spaces += 1;
    }
    if &query[number_spaces..number_spaces + 4] == "Bob " {
        temp = query[number_spaces + 4..].to_string();
    } else {
        crate::raise(crate::Error::BOB_NOT_FOUND);
    }
    if &query[query.len() - 1..] == "." {
        temp = temp[..temp.len() - 1].to_string();
    } else {
        crate::raise(crate::Error::PERIOD_NOT_FOUND);
    }
    temp
}

/// verify syntax and validity of expression and return type and number (separated by and)
pub fn expression_parser(
    query_vector: &mut Vec<String>,
    variable_stack: &mut Vec<crate::Variable>,
) -> (Vec<crate::Expression_type>, Vec<String>) {
    let mut expressions: Vec<String> = query_vector
        .join(" ")
        .split(" and ")
        .map(str::to_string)
        .collect();
    let mut expression_type_vector: Vec<crate::Expression_type> = Vec::new();
    let mut expression_type: crate::Expression_type = crate::Expression_type::STRING;
    for mut expression in &mut expressions {
        let temp_expression: &mut Vec<(String, crate::Token_type)> =
            &mut crate::token_assigner(&mut expression, variable_stack);
        temp_expression.retain(|x| {
            if x.0 == "(" || x.0 == ")" {
                false
            } else {
                true
            }
        });
        let is_valid: bool = match temp_expression[0].1 {
            //Check if the expression is a valid declaring expression of the form '"NAME" be type'.
            crate::Token_type::TO_BE_IDENTITY => {
                if temp_expression.len() == 3 {
                if temp_expression[1].1 == crate::Token_type::OPERATOR_ASSIGNMENT {
                    if temp_expression[2].1 == crate::Token_type::TYPE_STRING {
                        expression_type = crate::Expression_type::DECLARER_STRING;
                        true
                    }
                    else if temp_expression[2].1 == crate::Token_type::TYPE_DECIMAL {
                        expression_type = crate::Expression_type::DECLARER_DECIMAL;
                        true
                    }
else if temp_expression[2].1 == crate::Token_type::TYPE_NUMBER {
                        expression_type = crate::Expression_type::DECLARER_NUMBER;
                        true
}
else {
    false
}
                }
                else {
                    false
                }
                }
                else {
                    false
                }
            },
            //Check if the expression is a valid numeral expression (decimal or number)
            crate::Token_type::NUMBER_IDENTITY
            | crate::Token_type::DECIMAL_IDENTITY
            | crate::Token_type::NUMBER_LITERAL
            | crate::Token_type::DECIMAL_LITERAL => {
                let mut return_bool: bool = false;
                expression_type = crate::Expression_type::NUMERIC;
                if temp_expression.len() != 1 {
                    if temp_expression.last().unwrap().1 == crate::Token_type::NUMBER_IDENTITY
                        || temp_expression.last().unwrap().1 == crate::Token_type::DECIMAL_IDENTITY
                        || temp_expression.last().unwrap().1 == crate::Token_type::NUMBER_LITERAL
                        || temp_expression.last().unwrap().1 == crate::Token_type::DECIMAL_LITERAL
                    {
                        for i in (1..temp_expression.len()).step_by(2) {
                            if (temp_expression[i].1 == crate::Token_type::OPERATOR_PLUS
                                || temp_expression[i].1
                                    == crate::Token_type::OTHER_OPERATOR_ARITHMETIC)
                                && temp_expression[i - 1].1 == crate::Token_type::NUMBER_IDENTITY
                                || temp_expression[i - 1].1 == crate::Token_type::DECIMAL_IDENTITY
                                || temp_expression[i - 1].1 == crate::Token_type::NUMBER_LITERAL
                                || temp_expression[i - 1].1 == crate::Token_type::DECIMAL_LITERAL
                            {
                                return_bool = true;
                            } else {
                                return_bool = false;
                                break;
                            }
                        }
                        return_bool
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            //Check if the expression is a valid string and string literal expression of the form str1+str2..
            crate::Token_type::STRING_LITERAL | crate::Token_type::STRING_IDENTITY => {
                let mut return_bool: bool = false;
                expression_type = crate::Expression_type::STRING;
                for x in temp_expression.iter_mut() {
                    if x.1 == crate::Token_type::STRING_LITERAL {
                        x.0 = x.0.replace("_"," ");
                    }
                }
                if temp_expression.len() != 1 {
                    if temp_expression.last().unwrap().1 == crate::Token_type::STRING_LITERAL
                        || temp_expression.last().unwrap().1 == crate::Token_type::STRING_IDENTITY
                    {
                        for i in (1..temp_expression.len()).step_by(2) {
                            if temp_expression[i].1 == crate::Token_type::OPERATOR_PLUS
                                && (temp_expression[i - 1].1 == crate::Token_type::STRING_LITERAL
                                    || temp_expression[i - 1].1
                                        == crate::Token_type::STRING_IDENTITY)
                            {
                                return_bool = true;
                            } else {
                                return_bool = false;
                                break;
                            }
                        }
                        return_bool
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            _ => false,
        };
        if is_valid {
            expression_type_vector.push(expression_type);
        } else {
             crate::raise(crate::Error::INVALID_EXPRESSION);
        }
        *expression = temp_expression.iter().map(|x| x.0.clone()).collect::<Vec<String>>().join(" ");
    }
    (expression_type_vector, expressions)
}
