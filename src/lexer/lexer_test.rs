#[cfg(test)]
mod lexer_test {
    use crate::lexer::Lexer;
    use crate::token;

    #[derive(Debug, Default, Clone)]
    struct ExpectStruct {
        #[allow(dead_code)]
        expected_type: token::TokenType,
        #[allow(dead_code)]
        expected_literal: String,
    }

    macro_rules! array_item_add {
    ($token_type:ident, $literal:expr) => {
        ExpectStruct {
            expected_type: token::$token_type.to_string(),
            expected_literal: $literal.to_string(),
        }
    }
}


    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
[1, 2];
{"foo": "bar"}"#;

        let tests = [
            array_item_add!(LET, "let"),
            array_item_add!(IDENT, "five"),
            array_item_add!(ASSIGN, "="),
            array_item_add!(INT, "5"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(LET, "let"),
            array_item_add!(IDENT, "ten"),
            array_item_add!(ASSIGN, "="),
            array_item_add!(INT, "10"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(LET, "let"),
            array_item_add!(IDENT, "add"),
            array_item_add!(ASSIGN, "="),
            array_item_add!(FUNCTION, "fn"),
            array_item_add!(LPAREN, "("),
            array_item_add!(IDENT, "x"),
            array_item_add!(COMMA, ","),
            array_item_add!(IDENT, "y"),
            array_item_add!(RPAREN, ")"),
            array_item_add!(LBRACE, "{"),
            array_item_add!(IDENT, "x"),
            array_item_add!(PLUS, "+"),
            array_item_add!(IDENT, "y"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(RBRACE, "}"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(LET, "let"),
            array_item_add!(IDENT, "result"),
            array_item_add!(ASSIGN, "="),
            array_item_add!(IDENT, "add"),
            array_item_add!(LPAREN, "("),
            array_item_add!(IDENT, "five"),
            array_item_add!(COMMA, ","),
            array_item_add!(IDENT, "ten"),
            array_item_add!(RPAREN, ")"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(BANG, "!"),
            array_item_add!(MINUS, "-"),
            array_item_add!(SLASH, "/"),
            array_item_add!(ASTERISK, "*"),
            array_item_add!(INT, "5"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(INT, "5"),
            array_item_add!(LT, "<"),
            array_item_add!(INT, "10"),
            array_item_add!(GT, ">"),
            array_item_add!(INT, "5"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(IF, "if"),
            array_item_add!(LPAREN, "("),
            array_item_add!(INT, "5"),
            array_item_add!(LT, "<"),
            array_item_add!(INT, "10"),
            array_item_add!(RPAREN, ")"),
            array_item_add!(LBRACE, "{"),
            array_item_add!(RETURN, "return"),
            array_item_add!(TRUE, "true"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(RBRACE, "}"),
            array_item_add!(ELSE, "else"),
            array_item_add!(LBRACE, "{"),
            array_item_add!(RETURN, "return"),
            array_item_add!(FALSE, "false"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(RBRACE, "}"),
            array_item_add!(INT, "10"),
            array_item_add!(EQ, "=="),
            array_item_add!(INT, "10"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(INT, "10"),
            array_item_add!(NOT_EQ, "!="),
            array_item_add!(INT, "9"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(STRING, "foobar"),
            array_item_add!(STRING, "foo bar"),
            array_item_add!(LBRACKET, "["),
            array_item_add!(INT, "1"),
            array_item_add!(COMMA, ","),
            array_item_add!(INT, "2"),
            array_item_add!(RBRACKET, "]"),
            array_item_add!(SEMICOLON, ";"),
            array_item_add!(LBRACE, "{"),
            array_item_add!(STRING, "foo"),
            array_item_add!(COLON, ":"),
            array_item_add!(STRING, "bar"),
            array_item_add!(RBRACE, "}"),
            array_item_add!(EOF, ""),
        ];

        let mut l = Lexer::new(&input.to_string());
        for item in tests {
            let tok = l.next_token();

            if tok.token_type != item.expected_type {
                println!("tok: {:#?}", tok);
                println!("item: {:#?}", item);
                println!("tokentype wrong. expected={}, got={}",
                         item.expected_type, tok.token_type);
            }
        }
    }
}

