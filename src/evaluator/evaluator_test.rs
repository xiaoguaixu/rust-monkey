
#[cfg(test)]
mod evaluator_test {
    use std::rc::Rc;
    use crate::ast::Node;

    use crate::evaluator::eval;
    use crate::lexer::Lexer;
    use crate::object;
    use crate::object::{Environment};
    use crate::parser::Parser;
    use crate::util::{NIL, Overloaded, VariantValue};

    struct ComValueExpect {
        pub input: String,
        pub value: VariantValue,
    }

    macro_rules! macro_fill_com_value_struct {
        ($tests:ident, $input:literal, $value:expr) => {
            $tests.push(ComValueExpect {
                input: $input.to_string(),
                value: VariantValue::from_value($value),
            })
        }
    }

    #[test]
    fn test_eval_integer_expression() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "5", 5);
        macro_fill_com_value_struct!(tests, "10", 10);
        macro_fill_com_value_struct!(tests, "-5", -5);
        macro_fill_com_value_struct!(tests, "-10", -10);
        macro_fill_com_value_struct!(tests, "5 + 5 + 5 + 5 - 10", 10);
        macro_fill_com_value_struct!(tests, "2 * 2 * 2 * 2 * 2", 32);
        macro_fill_com_value_struct!(tests, "-50 + 100 + -50", 0);
        macro_fill_com_value_struct!(tests, "5 * 2 + 10", 20);
        macro_fill_com_value_struct!(tests, "5 + 2 * 10", 25);
        macro_fill_com_value_struct!(tests, "20 + 2 * -10", 0);
        macro_fill_com_value_struct!(tests, "50 / 2 * 2 + 10", 60);
        macro_fill_com_value_struct!(tests, "2 * (5 + 10)", 30);
        macro_fill_com_value_struct!(tests, "3 * 3 * 3 + 10", 37);
        macro_fill_com_value_struct!(tests, "3 * (3 * 3) + 10", 37);
        macro_fill_com_value_struct!(tests, "(5 + 10 * 2 + 15 / 3) * 2 + -10", 50);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_integer_object(&evaluated, v.value.as_int());
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "true", true);
        macro_fill_com_value_struct!(tests, "false", false);
        macro_fill_com_value_struct!(tests, "1 < 2", true);
        macro_fill_com_value_struct!(tests, "1 > 2", false);
        macro_fill_com_value_struct!(tests, "1 < 1", false);
        macro_fill_com_value_struct!(tests, "1 > 1", false);
        macro_fill_com_value_struct!(tests, "1 == 1", true);
        macro_fill_com_value_struct!(tests, "1 != 1", false);
        macro_fill_com_value_struct!(tests, "1 == 2", false);
        macro_fill_com_value_struct!(tests, "1 != 2", true);
        macro_fill_com_value_struct!(tests, "true == true", true);
        macro_fill_com_value_struct!(tests, "false == false", true);
        macro_fill_com_value_struct!(tests, "true == false", false);
        macro_fill_com_value_struct!(tests, "true != false", true);
        macro_fill_com_value_struct!(tests, "false != true", true);
        macro_fill_com_value_struct!(tests, "(1 < 2) == true", true);
        macro_fill_com_value_struct!(tests, "(1 < 2) == false", false);
        macro_fill_com_value_struct!(tests, "(1 > 2) == true", false);
        macro_fill_com_value_struct!(tests, "(1 > 2) == false", true);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_boolean_object(&evaluated, v.value.as_bool());
        }
    }

    #[test]
    fn test_bang_operator() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "!true", false);
        macro_fill_com_value_struct!(tests, "!false", true);
        macro_fill_com_value_struct!(tests, "!5", false);
        macro_fill_com_value_struct!(tests, "!!true", true);
        macro_fill_com_value_struct!(tests, "!!false", false);
        macro_fill_com_value_struct!(tests, "!!5", true);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_boolean_object(&evaluated, v.value.as_bool());
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "if (true) { 10 }", 10);
        macro_fill_com_value_struct!(tests, "if (false) { 10 }", 10);
        macro_fill_com_value_struct!(tests, "if (1) { 10 }", 10);
        macro_fill_com_value_struct!(tests, "if (1 < 2) { 10 }", 10);
        macro_fill_com_value_struct!(tests, "if (1 > 2) { 10 }", 10);
        macro_fill_com_value_struct!(tests, "if (1 > 2) { 10 } else { 20 }", 20);
        macro_fill_com_value_struct!(tests, "if (1 < 2) { 10 } else { 20 }", 10);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            let value = evaluated.as_ref().unwrap().as_any();
            if value.is::<object::Integer>() {
                test_integer_object(&evaluated, v.value.as_int());
            } else if value.is::<object::NULL>() {
                test_null_object(&evaluated);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "return 10;", 10);
        macro_fill_com_value_struct!(tests, "return 10; 9;", 10);
        macro_fill_com_value_struct!(tests, "return 2 * 5; 9;", 10);
        macro_fill_com_value_struct!(tests, "9; return 2 * 5; 9;", 10);
        macro_fill_com_value_struct!(tests, "if (10 > 1) { return 10; }", 10);
        macro_fill_com_value_struct!(tests, r#"
            if (10 > 1) {
              if (20 > 1) {
                return 10;
              }

              return 1;
            }
        "#, 10);
        macro_fill_com_value_struct!(tests, r#"
            let f = fn(x) {
              return x;
              x + 10;
            };
            f(10);
        "#, 10);
        macro_fill_com_value_struct!(tests, r#"
            let f = fn(x) {
               let result = x + 10;
               return result;
               return 10;
            };
            f(10);
        "#, 20);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_integer_object(&evaluated, v.value.as_int());
        }
    }

    #[test]
    fn test_error_handling() {
        let mut tests: Vec<ComValueExpect> = vec![];
        macro_fill_com_value_struct!(tests, "5 + true;",
			"type mismatch: INTEGER + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, "5 + true; 5;",
			"type mismatch: INTEGER + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, "-true",
			"unknown operator: -BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, "true + false;",
			"unknown operator: BOOLEAN + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, "true + false + true + false;",
			"unknown operator: BOOLEAN + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, "5; true + false; 5",
			"unknown operator: BOOLEAN + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, r#""Hello" - "World""#,
			"unknown operator: STRING - STRING"
		);
        macro_fill_com_value_struct!(tests, r#"if (10 > 1) { true + false; }"#,
			"unknown operator: BOOLEAN + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, r#"
            if (10 > 1) {
              if (10 > 1) {
                return true + false;
              }

              return 1;
            }
            "#,
			"unknown operator: BOOLEAN + BOOLEAN"
		);
        macro_fill_com_value_struct!(tests, r#"foobar"#,
			"identifier not found: foobar"
		);
        macro_fill_com_value_struct!(tests, r#"{"name": "Monkey"}[fn(x) { x }];"#,
			"unusable as hash key: FUNCTION"
		);
        macro_fill_com_value_struct!(tests, r#"999[1]"#,
			"index operator not supported: INTEGER"
		);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());

            if evaluated.is_none() {
                println!("evaluated is null");
                continue;
            }

            let evaluated = evaluated.unwrap();

            let any = evaluated.as_any();
            if any.is::<object::Error>() {
                let msg = any.downcast_ref::<object::Error>().as_ref().unwrap().message.clone();
                if msg != v.value.as_string() {
                    println!("wrong error message. expected={}, got={}", v.value.as_string(), msg);
                }
            } else {
                println!("no error object returned. got={}", evaluated.object_type());
            }
        }
    }

    #[test]
    fn test_let_statements() {
        let mut tests: Vec<ComValueExpect> = vec![];
        macro_fill_com_value_struct!(tests, "let a = 5; a;", 5);
        macro_fill_com_value_struct!(tests, "let a = 5 * 5; a;", 25);
        macro_fill_com_value_struct!(tests, "let a = 5; let b = a; b;", 5);
        macro_fill_com_value_struct!(tests, "let a = 5; let b = a; let c = a + b + 5; c;", 15);
        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_integer_object(&evaluated, v.value.as_int());
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; };";
        let evaluated = test_eval(input);
        if evaluated.is_none() {
            println!("evaluted is null");
            return;
        }
        let evaluated = evaluated.unwrap();
        let any = evaluated.as_any();
        if any.is::<object::Function>() {
            let v = any.downcast_ref::<object::Function>().unwrap();
            if v.parameters.len() != 1 {
                println!("function has wrong parameters. Parameters={}", v.parameters.len());
                return;
            }
            if v.parameters[0].string() != "x" {
                println!("parameter is not 'x'. got={}", v.parameters[0].string());
                return;
            }

            let expected = "(x + 2)";
            if v.body.string() != expected {
                println!("body is not {}. got={}", expected, v.body.string());
                return;
            }
        } else {
            println!("object is not Function. got={:?}", evaluated.object_type());
        }
    }

    #[test]
    fn test_function_application() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "let identity = fn(x) { x; }; identity(5);", 5);
        macro_fill_com_value_struct!(tests, "let identity = fn(x) { return x; }; identity(5);", 5);
        macro_fill_com_value_struct!(tests, "let double = fn(x) { x * 2; }; double(5);", 10);
        macro_fill_com_value_struct!(tests, "let add = fn(x, y) { x + y; }; add(5, 5);", 10);
        macro_fill_com_value_struct!(tests, "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20);
        macro_fill_com_value_struct!(tests, "fn(x) { x; }(5)", 5);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            test_integer_object(&evaluated, v.value.as_int());
        }
    }

    #[test]
    fn test_enclosing_environments() {
        let input = r#"
            let first = 10;
            let second = 10;
            let third = 10;

            let ourFunction = fn(first) {
              let second = 20;
              first + second + third;
            };

            ourFunction(20) + first + second;
        "#;
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, 70);
    }

    #[test]
    fn test_closures() {
        let input = r#"
            let newAdder = fn(x) {
              fn(y) { x + y };
            };

            let addTwo = newAdder(2);
            addTwo(2);
        "#;
        let evaluated = test_eval(input);
        test_integer_object(&evaluated, 4);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""Hello World!""#;
        let evaluated = test_eval(input);
        if evaluated.is_none() {
            println!("evaluated is null");
            return;
        }

        let evaluated = evaluated.unwrap();
        let value = evaluated.as_any();
        if value.is::<object::StringValue>() {
            let value = value.downcast_ref::<object::StringValue>().unwrap().value.clone();
            if value != "Hello World!" {
                println!("String has wrong value. got={}", value);
            }
        } else {
            println!("object is not String. got={}", evaluated.object_type());
        }
    }

    #[test]
    fn test_builtin_functions() {
        let mut tests: Vec<ComValueExpect> = vec![];
        macro_fill_com_value_struct!(tests, r#"len("")"#, 0);
        macro_fill_com_value_struct!(tests, r#"len("four")"#, 4);
        macro_fill_com_value_struct!(tests, r#"len("hello world")"#, 11);
        macro_fill_com_value_struct!(tests, r#"len(1)"#, "argument to `len` not supported, got INTEGER");
        macro_fill_com_value_struct!(tests, r#"len("one", "two")"#, "wrong number of arguments. got=2, want=1");
        macro_fill_com_value_struct!(tests, r#"len([1, 2, 3])"#, 3);
        macro_fill_com_value_struct!(tests, r#"len([])"#, 0);
        macro_fill_com_value_struct!(tests, r#"puts("hello", "world!")"#, NIL);
        macro_fill_com_value_struct!(tests, r#"first([1, 2, 3])"#, 1);
        macro_fill_com_value_struct!(tests, r#"first([])"#, NIL);
        macro_fill_com_value_struct!(tests, r#"first(1)"#, "argument to `first` must be ARRAY, got INTEGER");
        macro_fill_com_value_struct!(tests, r#"last([1, 2, 3])"#, 3);
        macro_fill_com_value_struct!(tests, r#"last([])"#, NIL);
        macro_fill_com_value_struct!(tests, r#"last(1)"#, "argument to `last` must be ARRAY, got INTEGER");
        macro_fill_com_value_struct!(tests, r#"rest([1, 2, 3])"#, vec![2,3]);
        macro_fill_com_value_struct!(tests, r#"rest([])"#, 1);
        macro_fill_com_value_struct!(tests, r#"push([], 1)"#, vec![1]);
        macro_fill_com_value_struct!(tests, r#"push(1, 1)"#, "argument to `push` must be ARRAY, got INTEGER");
        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            match v.value {
                VariantValue::ValueInt(v) => {
                    test_integer_object(&evaluated, v);
                }
                VariantValue::ValueBool(_) => {}
                VariantValue::ValueString(v) => {
                    let evaluated = evaluated.unwrap();
                    let any = evaluated.as_any();
                    if any.is::<object::Error>() {
                        let msg = &any.downcast_ref::<object::Error>().unwrap().message;
                        if v != *msg {
                            println!("wrong error message. expected={}, got={}", v, msg);
                        }
                    } else {
                        println!("object is not Error. got={}", evaluated.object_type());
                        continue;
                    }
                }
                VariantValue::ValueNull(_) => {
                    test_null_object(&evaluated);
                }
                VariantValue::ValueIntArray(v) => {
                    let evaluated = evaluated.unwrap();
                    let any = evaluated.as_any();
                    if any.is::<object::Array>() {
                        let ary = any.downcast_ref::<object::Array>().unwrap();
                        if ary.elements.len() != v.len() {
                            println!("wrong num of elements. want={}, got={}", v.len(), ary.elements.len());
                            continue;
                        }

                        for i in 0..v.len() {
                            test_integer_object(&Some(ary.elements[i].clone()), v[i]);
                        }
                    } else {
                        println!("object is not Array. got={}", evaluated.object_type());
                        continue;
                    }
                }
            }
        }
    }

    #[test]
    fn test_array_literals() {
        let input = r#"[1, 2 * 2, 3 + 3]"#;
        let evaluated = test_eval(input);
        if evaluated.is_none() {
            println!("evaluated is null");
            return;
        }

        let evaluated = evaluated.unwrap();
        let value = evaluated.as_any();
        if value.is::<object::Array>() {
            let v = value.downcast_ref::<object::Array>().unwrap();
            if v.elements.len() != 3 {
                println!("array has wrong num of elements. go={}", v.elements.len());
            }
            test_integer_object(&Some(v.elements[0].clone()), 1);
            test_integer_object(&Some(v.elements[1].clone()), 4);
            test_integer_object(&Some(v.elements[2].clone()), 6);
        } else {
            println!("object is not Array. got={}", evaluated.object_type());
        }
    }

    #[test]
    fn test_array_index_expressions() {
        let mut tests: Vec<ComValueExpect> = vec![];

        macro_fill_com_value_struct!(tests, "[1, 2, 3][0]", 1);
        macro_fill_com_value_struct!(tests, "[1, 2, 3][1]", 2);
        macro_fill_com_value_struct!(tests, "[1, 2, 3][2]", 3);
        macro_fill_com_value_struct!(tests, "let i = 0; [1][i];", 1);
        macro_fill_com_value_struct!(tests, "[1, 2, 3][1 + 1];", 3);
        macro_fill_com_value_struct!(tests, "let myArray = [1, 2, 3]; myArray[2];", 3);
        macro_fill_com_value_struct!(tests, "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];", 6);
        macro_fill_com_value_struct!(tests, "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]", 2);
        macro_fill_com_value_struct!(tests, "[1, 2, 3][3]", 1);
        macro_fill_com_value_struct!(tests, "[1, 2, 3][-1]", 1);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            if evaluated.is_none() {
                println!("evaluated is null");
                continue;
            }

            let value = evaluated.as_ref().unwrap();
            let value = value.as_any();
            if value.is::<object::Integer>() {
                test_integer_object(&evaluated, v.value.as_int());
            } else if value.is::<object::NULL>() {
                test_null_object(&evaluated);
            }
        }
    }

    #[test]
    fn test_hash_literals() {
        // let input = r#"
        //     let two = "two";
        //     {
        //         "one": 10 - 9,
        //         two: 1 + 1,
        //         "thr" + "ee": 6 / 2,
        //         4: 4,
        //         true: 5,
        //         false: 6
        //     }
        // "#;
    }

    #[test]
    fn test_hash_index_expressions() {
        let mut tests: Vec<ComValueExpect> = vec![];
        macro_fill_com_value_struct!(tests, r#"{"foo": 5}["foo"]"#,5);
        macro_fill_com_value_struct!(tests, r#"{"foo": 5}["bar"]"#,5);
        macro_fill_com_value_struct!(tests, r#"let key = "foo"; {"foo": 5}[key]"#,5);
        macro_fill_com_value_struct!(tests, r#"{}["foo"]"#,NIL);
        macro_fill_com_value_struct!(tests, r#"{5: 5}[5]"#,5);
        macro_fill_com_value_struct!(tests, r#"{true: 5}[true]"#,5);
        macro_fill_com_value_struct!(tests, r#"{false: 5}[false]"#,5);

        for v in tests {
            let evaluated = test_eval(v.input.as_str());
            if evaluated.is_none() {
                println!("evaluated is null");
                continue;
            }

            let value = evaluated.as_ref().unwrap();
            let value = value.as_any();
            if value.is::<object::Integer>() {
                test_integer_object(&evaluated, v.value.as_int());
            } else if value.is::<object::NULL>() {
                test_null_object(&evaluated);
            }
        }
    }

    fn test_eval(input: &str) -> Option<Rc<dyn object::Object>> {
        let l = Lexer::new(&input.to_string());
        let mut p = Parser::new(Box::new(l));
        let program = p.parse_program();
        let mut env = Environment::new();
        return eval(&*program, &mut env);
    }

    fn test_integer_object(obj: &Option<Rc<dyn object::Object>>, expected: i64) -> bool {
        if obj.is_none() {
            println!("evaluate is none");
            return false;
        }

        let obj = obj.as_ref().unwrap();
        let value = obj.as_any();

        return match value.downcast_ref::<object::Integer>() {
            None => {
                println!("not Integer: {}", obj.object_type());
                false
            }
            Some(v) => {
                if v.value == expected { true } else {
                    println!("object has wrong value. got={}, want={}", v.value, expected);
                    false
                }
            }
        };
    }

    fn test_null_object(obj: &Option<Rc<dyn object::Object>>) -> bool {
        if obj.is_none() {
            println!("evaluate is none");
            return false;
        }

        let obj = obj.as_ref().unwrap().as_any();

        return if obj.is::<object::NULL>() {
            true
        } else {
            println!("object is not NULL");
            false
        }

    }

    fn test_boolean_object(obj: &Option<Rc<dyn object::Object>>, expected: bool) -> bool {
        if obj.is_none() {
            println!("evaluate is none");
            return false;
        }

        let obj = obj.as_ref().unwrap().as_any();

        return match obj.downcast_ref::<object::Boolean>() {
            None => {
                println!("not Integer");
                false
            }
            Some(v) => {
                if v.value == expected { true } else {
                    println!("object has wrong value. got={}, want={}", v.value, expected);
                    false
                }
            }
        };
    }

}