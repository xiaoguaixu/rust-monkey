// #[cfg(test)]
// mod ast_test {
//     use crate::{ast, token};
//     use crate::ast::base::Node;
//
//     #[test]
//     fn test_ast() {
//         let statement = ast::LetStatement {
//             token: token::Token {
//                 token_type: token::LET.to_string(),
//                 literal: "let".to_string(),
//             },
//             name: Box::new(ast::Identifier {
//                 token: token::Token {
//                     token_type: token::IDENT.to_string(),
//                     literal: "myVar".to_string(),
//                 },
//                 value: "myVar".to_string(),
//             }),
//             value: Some(Box::new(ast::Identifier {
//                 token: token::Token {
//                     token_type: token::IDENT.to_string(),
//                     literal: "anotherVar".to_string(),
//                 },
//                 value: "anotherVar".to_string(),
//             })),
//         };
//
//         let program = ast::Program {
//             statements: vec![
//                 Box::new(statement)
//             ],
//         };
//
//         println!("{}", program.string());
//     }
// }
