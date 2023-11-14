use std::collections::HashMap;

use crate::const_str_val_declare;
use crate::token;

//use phf::phf_map;

const_str_val_declare!(ILLEGAL, "ILLEGAL");
const_str_val_declare!(EOF, "EOF");

// Identifiers + literals
const_str_val_declare!(IDENT, "IDENT");
const_str_val_declare!(INT, "INT");
const_str_val_declare!(STRING, "STRING");

// Operators
const_str_val_declare!(ASSIGN, "=");
const_str_val_declare!(PLUS, "+");
const_str_val_declare!(MINUS, "-");
const_str_val_declare!(BANG, "!");
const_str_val_declare!(ASTERISK, "*");
const_str_val_declare!(SLASH, "/");


const_str_val_declare!(LT, "<");
const_str_val_declare!(GT, ">");

const_str_val_declare!(EQ, "==");
const_str_val_declare!(NOT_EQ, "!=");


// Delimiters
const_str_val_declare!(COMMA, ",");
const_str_val_declare!(SEMICOLON, ";");
const_str_val_declare!(COLON, ":");

const_str_val_declare!(LPAREN, "(");
const_str_val_declare!(RPAREN, ")");
const_str_val_declare!(LBRACE, "{");
const_str_val_declare!(RBRACE, "}");
const_str_val_declare!(LBRACKET, "[");
const_str_val_declare!(RBRACKET, "]");


// Keywords
const_str_val_declare!(FUNCTION, "FUNCTION");
const_str_val_declare!(LET, "LET");
const_str_val_declare!(TRUE, "TRUE");
const_str_val_declare!(FALSE, "FALSE");
const_str_val_declare!(IF, "IF");
const_str_val_declare!(ELSE, "ELSE");
const_str_val_declare!(RETURN, "RETURN");


pub type TokenType = String;

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub token_type: String,
    pub literal: String,
}

// static KEYWORDS: phf::Map<&'static str, &'static str> = phf_map! {
//     "fn" => FUNCTION,
// 	"let" =>    LET,
// 	"true" =>   TRUE,
// 	"false" =>  FALSE,
// 	"if" =>     IF,
// 	"else" =>   ELSE,
// 	"return" => RETURN,
// };

thread_local! {
    pub static KEYWORDS: HashMap<&'static str, &'static str> =HashMap::from([
        ("fn" , FUNCTION),
        ("let" ,    LET),
        ("true" ,   TRUE),
        ("false" ,  FALSE),
        ("if" ,     IF),
        ("else" ,   ELSE),
        ("return" , RETURN),
    ])
}


pub fn lookup_ident(ident: &String) -> TokenType {
    // match KEYWORDS.get(ident.as_str()) {
    //     Some(v) => {
    //         v.to_string()
    //     }
    //     None => token::IDENT.to_string()
    // }

    KEYWORDS.with(|val| {
        if let Some(v) = val.get(ident.as_str()) {
            return v.to_string();
        }
        return token::IDENT.to_string();
    })
}

