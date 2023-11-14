#[cfg(test)]
mod parse_test {
    use crate::ast;
    use crate::ast::{ASTNode, Node};
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::util::{Overloaded, VariantValue};
    use crate::util::VariantValue::ValueString;

    struct LetExpectStruct {
        pub input: String,
        pub expected_identifier: String,
        pub value: VariantValue,
    }

    struct ReturnExpectStruct {
        pub input: String,
        pub value: VariantValue,
    }

    struct PrefixExpectStruct {
        pub input: String,
        pub operator: String,
        pub value: VariantValue,
    }

    struct InfixExpectStruct {
        pub input: String,
        pub left_value: VariantValue,
        pub operator: String,
        pub right_value: VariantValue,
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
            LetExpectStruct {
                input: "let x = 5;".to_string(),
                expected_identifier: "x".to_string(),
                value: VariantValue::ValueInt(5),
            },
            LetExpectStruct {
                input: "let y = true;".to_string(),
                expected_identifier: "y".to_string(),
                value: VariantValue::ValueBool(true),
            },
            LetExpectStruct {
                input: "let foobar = y;".to_string(),
                expected_identifier: "foobar".to_string(),
                value: VariantValue::ValueString("y".to_string()),
            },
        ];

        for v in tests {
            let l = Lexer::new(&v.input);
            let mut p = Parser::new(Box::new(l));
            let program = p.parse_program();
            print_parser_errors(&p.errors());
            if let ASTNode::Program(program) = program {
                if program.statements.len() != 1 {
                    println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                    continue;
                }

                let let_statement = match &program.statements[0] {
                    ASTNode::LetStatement(v) => {
                        v
                    }
                    _ => { continue; }
                };

                let expression = &*let_statement.value;
                check_let_statement(let_statement, v.expected_identifier.as_str());
                check_literal_expression(expression, &v.value);
            } else {
                print_parser_errors(&p.errors());
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ReturnExpectStruct {
                input: "return 5;".to_string(),
                value: VariantValue::ValueInt(5),
            },
            ReturnExpectStruct {
                input: "return true;".to_string(),
                value: VariantValue::ValueBool(true),
            },
            ReturnExpectStruct {
                input: "return foobar;".to_string(),
                value: VariantValue::ValueString("foobar".to_string()),
            },
        ];

        for v in tests {
            let l = Lexer::new(&v.input);
            let mut p = Parser::new(Box::new(l));
            let program = p.parse_program();
            print_parser_errors(&p.errors());
            if let ASTNode::Program(program) = program {
                if program.statements.len() != 1 {
                    println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                    continue;
                }
                let statement = match &program.statements[0] {
                    ASTNode::ReturnStatement(v) => {
                        v
                    }
                    _ => { continue; }
                };

                let expression = statement.return_value.as_ref();
                check_literal_expression(expression, &v.value);
            }
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let mut tests: Vec<PrefixExpectStruct> = vec![];

        macro_rules! macro_fill_prefix_struct {
            ($input:literal, $operator:literal, $value:expr) => {
                tests.push(PrefixExpectStruct {
                    input: $input.to_string(),
                    operator: $operator.to_string(),
                    value: VariantValue::from_value($value),
                })
            }
        }

        macro_fill_prefix_struct!("!5;", "!", 5);
        macro_fill_prefix_struct!("-15;", "-", 15);
        macro_fill_prefix_struct!("!foobar;", "!", "foobar");
        macro_fill_prefix_struct!("-foobar;", "-", "foobar");
        macro_fill_prefix_struct!("!true;", "!", true);
        macro_fill_prefix_struct!("!false;", "!", false);

        for v in tests {
            let l = Lexer::new(&v.input);
            let mut p = Parser::new(Box::new(l));
            let program = p.parse_program();
            print_parser_errors(&p.errors());
            if let ASTNode::Program(program) = program {
                if program.statements.len() != 1 {
                    println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                    continue;
                }

                let statement = match &program.statements[0] {
                    ASTNode::ExpressionStatement(v) => {
                        v
                    }
                    _ => { continue; }
                };

                let expression = match &*statement.expression {
                    ASTNode::PrefixExpression(v) => {
                        v
                    }
                    _ => { continue; }
                };

                if expression.operator != v.operator {
                    println!("operator error");
                }
                if !check_literal_expression(&expression.right, &v.value) {
                    println!("error!")
                }
            }
        }
    }


    #[test]
    fn test_parsing_infix_expressions() {
        let mut tests: Vec<InfixExpectStruct> = vec![];
        macro_rules! macro_fill_infix_struct {
            ($input:literal, $left_value:literal, $operator:literal, $right_value:literal) => {
                tests.push(InfixExpectStruct {
                    input: $input.to_string(),
                    left_value: VariantValue::from_value($left_value),
                    operator: $operator.to_string(),
                    right_value: VariantValue::from_value($right_value),
                })
            }
        }

        macro_fill_infix_struct!("5 + 5;", 5, "+", 5);
        macro_fill_infix_struct!("5 - 5;", 5, "-", 5);
        macro_fill_infix_struct!("5 * 5;", 5, "*", 5);
        macro_fill_infix_struct!("5 / 5;", 5, "/", 5);
        macro_fill_infix_struct!("5 > 5;", 5, ">", 5);
        macro_fill_infix_struct!("5 < 5;", 5, "<", 5);
        macro_fill_infix_struct!("5 == 5;", 5, "==", 5);
        macro_fill_infix_struct!("5 != 5;", 5, "!=", 5);
        macro_fill_infix_struct!("foobar + barfoo;", "foobar", "+", "barfoo");
        macro_fill_infix_struct!("foobar - barfoo;", "foobar", "-", "barfoo");
        macro_fill_infix_struct!("foobar * barfoo;", "foobar", "*", "barfoo");
        macro_fill_infix_struct!("foobar / barfoo;", "foobar", "/", "barfoo");
        macro_fill_infix_struct!("foobar > barfoo;", "foobar", ">", "barfoo");
        macro_fill_infix_struct!("foobar < barfoo;", "foobar", "<", "barfoo");
        macro_fill_infix_struct!("foobar == barfoo;", "foobar", "==", "barfoo");
        macro_fill_infix_struct!("foobar != barfoo;", "foobar", "!=", "barfoo");
        macro_fill_infix_struct!("true == true", true, "==", true);
        macro_fill_infix_struct!("true != false", true, "!=", false);
        macro_fill_infix_struct!("false == false", false, "==", false);

        for v in tests {
            let l = Lexer::new(&v.input);
            let mut p = Parser::new(Box::new(l));
            let program = p.parse_program();
            print_parser_errors(&p.errors());
            if let ASTNode::Program(program) = program {
                if program.statements.len() != 1 {
                    println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                    continue;
                }

                let statement = match &program.statements[0] {
                    ASTNode::ExpressionStatement(v) => {
                        v
                    }
                    _ => { continue; }
                };

                if !check_infix_expression(statement.expression.as_ref(),
                                           &v.left_value, &v.operator, &v.right_value) {
                    println!("error!")
                }
            }
        }
    }

    #[derive(Debug)]
    struct OperateStruct {
        #[allow(dead_code)]
        pub input: String,
        #[allow(dead_code)]
        pub expected: String,
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let mut tests: Vec<OperateStruct> = vec![];
        macro_rules! macro_fill_operator_struct {
            ($input:literal, $expected:literal) => {
                tests.push(OperateStruct {
                    input: $input.to_string(),
                    expected: $expected.to_string(),
                })
            }
        }

        macro_fill_operator_struct!("-a * b", "((-a) * b)");
        macro_fill_operator_struct!("!-a", "(!(-a))");
        macro_fill_operator_struct!("a + b + c", "((a + b) + c)");
        macro_fill_operator_struct!("a + b - c", "((a + b) - c)");
        macro_fill_operator_struct!("a * b * c", "((a * b) * c)");
        macro_fill_operator_struct!("a * b / c", "((a * b) / c)");
        macro_fill_operator_struct!("a + b / c", "(a + (b / c))");
        macro_fill_operator_struct!("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)");
        macro_fill_operator_struct!("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)");
        macro_fill_operator_struct!("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))");
        macro_fill_operator_struct!("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))");
        macro_fill_operator_struct!("3 + 4 * 5 == 3 * 1 + 4 * 5", "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))");
        macro_fill_operator_struct!("true", "true");
        macro_fill_operator_struct!("false", "false");
        macro_fill_operator_struct!("3 > 5 == false", "((3 > 5) == false)");
        macro_fill_operator_struct!("3 < 5 == true", "((3 < 5) == true)");
        macro_fill_operator_struct!("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)");
        macro_fill_operator_struct!("(5 + 5) * 2", "((5 + 5) * 2)");
        macro_fill_operator_struct!("2 / (5 + 5)", "(2 / (5 + 5))");
        macro_fill_operator_struct!("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))");
        macro_fill_operator_struct!("-(5 + 5)", "(-(5 + 5))");
        macro_fill_operator_struct!("!(true == true)", "(!(true == true))");
        macro_fill_operator_struct!("a + add(b * c) + d", "((a + add((b * c))) + d)");
        macro_fill_operator_struct!("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))", "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))");
        macro_fill_operator_struct!("add(a + b + c * d / f + g)", "add((((a + b) + ((c * d) / f)) + g))");
        macro_fill_operator_struct!("[1, 2][0]", "([1, 2][0])");
        macro_fill_operator_struct!("a * [1, 2, 3, 4][b * c] * d", "((a * ([1, 2, 3, 4][(b * c)])) * d)");
        macro_fill_operator_struct!("add(a * b[2], b[1], 2 * [1, 2][1])", "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))");


        for v in tests {
            let l = Lexer::new(&v.input);
            let mut p = Parser::new(Box::new(l));
            let program = p.parse_program();
            print_parser_errors(&p.errors());
            if let ASTNode::Program(program) = program {
                let actual = program.string();
                if actual != v.expected {
                    println!("expected={}, got={}", v.expected, actual);
                }
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let l = Lexer::new(&input.to_string());
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        print_parser_errors(&p.errors());
        if let ASTNode::Program(program) = program {
            if program.statements.len() != 1 {
                println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let statement = match &program.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            let expression = match &*statement.expression {
                ASTNode::IfExpression(v) => {
                    v
                }
                _ => { return; }
            };

            if !check_infix_expression(&*expression.condition, &ValueString("x".to_string()),
                                       "<", &ValueString("y".to_string())) {
                return;
            }

            let block_statement = match &*expression.consequence {
                ASTNode::BlockStatement(v) => {
                    v
                }
                _ => { return; }
            };

            if block_statement.statements.len() != 1 {
                println!("consequence does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let consequence = match &block_statement.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            if !check_identifier(&consequence.expression, "x") {
                return;
            }

            if !expression.alternative.is_none() {
                println!("exp.Alternative.Statements was not nil");
            }
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let l = Lexer::new(&input.to_string());
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        print_parser_errors(&p.errors());
        if let ASTNode::Program(program) = program {
            if program.statements.len() != 1 {
                println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let statement = match &program.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            let expression = match &*statement.expression {
                ASTNode::IfExpression(v) => {
                    v
                }
                _ => { return; }
            };

            if !check_infix_expression(&expression.condition, &ValueString("x".to_string()),
                                       "<", &ValueString("y".to_string())) {
                return;
            }

            let block_statement = match &*expression.consequence {
                ASTNode::BlockStatement(v) => {
                    v
                }
                _ => { return; }
            };

            if block_statement.statements.len() != 1 {
                println!("consequence does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let consequence = match &block_statement.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            if !check_identifier(&*consequence.expression, "x") {
                return;
            }

            if expression.alternative.is_none() {
                println!("exp.Alternative.Statements was nil");
            }

            let alternative = match &*expression.alternative {
                ASTNode::BlockStatement(v) => {
                    v
                }
                _ => { return; }
            };

            if alternative.statements.len() != 1 {
                return;
            }

            let alternative = &alternative.statements[0];
            let alternative = match &alternative {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };
            if !check_identifier(&alternative.expression, "y") {
                return;
            }
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";
        let l = Lexer::new(&input.to_string());
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        print_parser_errors(&p.errors());
        if let ASTNode::Program(program) = program {
            if program.statements.len() != 1 {
                println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let statement = match &program.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            let expression = match &*statement.expression {
                ASTNode::FunctionLiteral(v) => {
                    v
                }
                _ => { return; }
            };

            if expression.parameters.len() != 2 {
                println!("function literal parameters wrong. want 2, got={}", expression.parameters.len());
                return;
            }

            let parameter = &expression.parameters[0];
            check_literal_expression(parameter, &ValueString("x".to_string()));

            let parameter = &expression.parameters[1];
            check_literal_expression(parameter, &ValueString("y".to_string()));

            let statement = match &*expression.body {
                ASTNode::BlockStatement(v) => {
                    v
                }
                _ => { return; }
            };

            let statement = match &statement.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            check_infix_expression(&statement.expression, &ValueString("x".to_string()),
                                   "+", &ValueString("y".to_string()));
        }
    }

    #[test]
    fn test_parsing_hash_literals_string_keys() {
        let input = r#"{"one": 1, "two": 2, "three": 3}"#;
        //let input = r#"{}"#;
        let l = Lexer::new(&input.to_string());
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        print_parser_errors(&p.errors());
        if let ASTNode::Program(program) = program {
            if program.statements.len() != 1 {
                println!("program.Statements does not contain 1 statements. got={}", program.statements.len());
                return;
            }

            let statement = match &program.statements[0] {
                ASTNode::ExpressionStatement(v) => {
                    v
                }
                _ => { return; }
            };

            let expression = match &*statement.expression {
                ASTNode::HashLiteral(v) => {
                    v
                }
                _ => { return; }
            };

            for (key, value) in &expression.pairs {
                println!("hash key: {}, value: {}", key.string(), value.string());
            }
        }
    }

    fn print_parser_errors(errors: &Vec<String>) {
        if errors.len() == 0 {
            return;
        }
        println!("{}", " parser errors:");
        for msg in errors {
            println!("{}", msg);
        }
    }

    fn check_let_statement(statement: &ast::LetStatement, name: &str) -> bool {
        if statement.token_literal() != "let" {
            println!("s.TokenLiteral not 'let'. got={}", statement.token_literal());
            return false;
        }

        if statement.name.token_literal() != name {
            println!("letStmt.Name.TokenLiteral() not {}. got={}", name, statement.name.token_literal());
            return false;
        }
        true
    }

    //
    fn check_literal_expression(expression: &ASTNode, value: &VariantValue) -> bool {
        return match value {
            VariantValue::ValueInt(v) => {
                check_integer_literal(expression, *v)
            }
            VariantValue::ValueBool(v) => {
                check_boolean_literal(expression, *v)
            }
            VariantValue::ValueString(v) => {
                check_identifier(expression, v)
            }
            _ => { false }
        };
    }

    fn check_integer_literal(expression: &ASTNode, value: i64) -> bool {
        match expression {
            ASTNode::IntegerLiteral(expression) => {
                if expression.value != value {
                    println!("integ.Value not {}. got={}", value, expression.value);
                    return false;
                }
                if expression.token.literal != format!("{}", value) {
                    println!("integ.TokenLiteral not {}. got={}", value, expression.token.literal);
                    return false;
                }
            }
            _ => {
                println!("not IntegerLiteral");
                return false;
            }
        }

        true
    }

    fn check_identifier(expression: &ASTNode, value: &str) -> bool {
        match expression {
            ASTNode::Identifier(expression) => {
                if expression.value != value {
                    println!("integ.Value not {}. got={}", value, expression.value);
                    return false;
                }
                if expression.token.literal != format!("{}", value) {
                    println!("integ.TokenLiteral not {}. got={}", value, expression.token.literal);
                    return false;
                }
            }
            _ => {
                println!("not Identifier");
                return false;
            }
        }

        true
    }

    fn check_boolean_literal(expression: &ASTNode, value: bool) -> bool {
        match expression {
            ASTNode::Boolean(expression) => {
                if expression.value != value {
                    println!("integ.Value not {}. got={}", value, expression.value);
                    return false;
                }
                if expression.token.literal != format!("{}", value) {
                    println!("integ.TokenLiteral not {}. got={}", value, expression.token.literal);
                    return false;
                }
            }
            _ => {}
        }

        true
    }

    fn check_infix_expression(expression: &ASTNode, left: &VariantValue,
                              operator: &str, right: &VariantValue) -> bool {
        match expression {
            ASTNode::InfixExpression(expression) => {
                if !check_literal_expression(&expression.left, left) {
                    return false;
                }

                if expression.operator != operator {
                    println!("exp.Operator is not {}. got={}", operator, expression.operator);
                    return false;
                }

                if !check_literal_expression(&expression.right, right) {
                    return false;
                }
            }
            _ => {}
        }

        true
    }
}