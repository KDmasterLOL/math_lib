// use std::{borrow::Borrow, clone, collections::HashMap, env::var_os, rc::Rc, sync::Arc};

// type ResultType = f32;
// pub fn tokenise(equation: String, variables: HashMap<char, Arc<ResultType>>) -> Vec<Token> {
//     let mut tokens = Vec::<Token>::with_capacity(10);
//     let mut iterator = equation.chars().peekable();
//     let mut buffer_num: String = String::new();
//     let mut buffer_string: String = String::new();
//     let mut last_value = false;
//     while let Some(ch) = iterator.next() {
        
//         if ch.is_numeric() {
//             buffer_num.push(ch);
//             if !iterator.peek().unwrap_or(&'\0').is_numeric() {
//                 tokens.push(Token::Constant(buffer_num.parse::<f32>().unwrap()));
//                 last_value = true;
//             }
//         } else if ch.is_ascii_alphabetic() {
//             buffer_string.push(ch);
//             if !iterator.peek().unwrap_or(&'\0').is_ascii_alphabetic() {
//                 if let Some(var) = if buffer_string.len() == 1 {
//                     variables.get(&buffer_string.chars().next().unwrap())
//                 } else {
//                     None
//                 } {
//                     tokens.push(Token::Variable(var.clone()));
//                     buffer_string.clear();
//                     last_value = true;
//                 } else if let Some(unary) = UNARY_OPERATORS.get(buffer_string) {
//                     tokens.push(Token::Operator(Operator::new(
//                         ch,
//                         OperatorKind::Unary(unary.clone()),
//                     )))
//                 }
//             }
//         } else if ch.is_ascii_punctuation() {
//             if let Some(binary) = BINARY_OPERATORS.get_key_value(&ch) {
//                 buffer_string.push(ch);
//             } else if let Some(unary) = UNARY_OPERATORS.get_key_value(&ch.to_string().borrow()) {
//                 buffer_string.push(ch);
//             }
//         }
//     }
//     tokens
// }
// #[derive(Clone)]
// pub enum Token {
//     Operator(Operator),
//     Constant(ResultType),
//     Variable(Arc<ResultType>),
// }
// impl Token {
//     // fn evaluate(&self) -> ResultType {
//     //     match self {
//     //         Token::Operator(operator) => match *operator.data {
//     //             OperatorData::Binary(lhs, rhs) => BINARY_OPERATIONS.get(operator.info.description),
//     //             OperatorData::Unary(exp) => todo!(),
//     //         },
//     //         Token::Constant(_) => todo!(),
//     //         Token::Variable(_) => todo!(),
//     //     }
//     // }
// }
// #[derive(Clone)]
// pub struct Operator {
//     description: char,
//     kind: OperatorKind,
// }
// impl Operator {
//     pub fn new(description: char, kind: OperatorKind) -> Self {
//         Operator { description, kind }
//     }
// }
// #[derive(Clone)]
// pub enum OperatorKind {
//     Binary(BinaryOperator),
//     Unary(UnaryOperator),
// }
// #[derive(Clone)]
// pub struct BinaryOperator {
//     precedence: usize,
//     operation: fn(ResultType, ResultType) -> ResultType,
// }
// impl BinaryOperator {
//     pub fn new(precedence: usize, operation: fn(ResultType, ResultType) -> ResultType) -> Self {
//         Self {
//             precedence,
//             operation,
//         }
//     }
// }
// #[derive(Clone)]
// pub struct UnaryOperator {
//     operation: fn(ResultType) -> ResultType,
// }
// impl UnaryOperator {
//     pub fn new(operation: fn(ResultType) -> ResultType) -> Self {
//         Self { operation }
//     }
// }
// lazy_static! {
//     static ref BINARY_OPERATORS: HashMap<char, BinaryOperator> = {
//         let mut map = HashMap::new();
//         map .insert('+', BinaryOperator::new(1, |x, y| x + y));
//         map .insert('-', BinaryOperator::new(1, |x, y| x - y));
//         map .insert('*', BinaryOperator::new(2, |x, y| x * y));
//         map
//     };
//     static ref UNARY_OPERATORS: HashMap<&'static str, UnaryOperator> = {
//         let mut map = HashMap::new();
//         map .insert("+", UnaryOperator::new(|x| x));
//         map .insert("-", UnaryOperator::new(|x| -x));
//         map .insert("sin", UnaryOperator::new(|x| x.sin()));
//         map
//     };
//     // static ref OPERATORS: HashMap<char, Operator> = {
//     //     let mut map = HashMap::new();
//     //     map.insert(
//     //         '+',
//     //         Operator::new(OperatorInfo {
//     //             description: '+',
//     //             precedence: 1,
//     //         }),
//     //         ,
//     //     );
//     //     // map.insert(
//     //     //     '-',
//     //     //     Operator::new(OperatorInfo {
//     //     //         description: '-',
//     //     //         precedence: 0,
//     //     //     },OperatorKind::Binary(BinaryOperator::new(|x, y| x - y))).into(),
//     //     //     .into(),
//     //     // );
//     //     map.insert(
//     //         '*',
//     //         OperatorInfo {
//     //             operation: |x, y| x * y,
//     //             description: '*',
//     //             precedence: 2,
//     //         }
//     //         .into(),
//     //     );
//     //     // map.insert(
//     //     //     '/',
//     //     //     OperatorInfo {
//     //     //         operation: |x, y| x / y,
//     //     //         description: '/',
//     //     //         precedence: 2,
//     //     //     }
//     //     //     .into(),
//     //     // );
//     //     map
//     // };
//     static ref CONSTANTS: HashMap<char, ResultType> = {
//         let mut map = HashMap::from([('k', 9f32 * 10f32.powf(9f32))]);
//         map
//     };
// }
