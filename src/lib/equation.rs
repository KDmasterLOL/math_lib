// use std::{
//     clone,
//     cmp::Ordering,
//     collections::{
//         binary_heap::{self, Iter},
//         BinaryHeap, HashMap,
//     },
//     f32::MAX,
//     fmt::{Binary, Display, Write},
//     iter::Peekable,
//     rc::Rc,
//     sync::Arc,
// };

// type ResultType = f32;
// type pos_prec = (usize, usize);
// pub struct Equation {
//     text: String,
//     pub tokens: Vec<Token>,
//     vars: HashMap<char, ResultType>,
// }
// impl Equation {
//     pub fn new(text: String, vars: Option<HashMap<char, ResultType>>) -> Equation {
//         let mut equation = Equation {
//             text,
//             tokens: Vec::<Token>::with_capacity(5),
//             vars: vars.unwrap_or_default(),
//         };
//         equation.parse();
//         equation
//     }
//     pub fn parse(&mut self) {
//         let mut num_str_buf = String::new();
//         let mut buf_operators = Vec::<(char, Arc<Operator>)>::with_capacity(3);

//         let mut equation_chars = self.text.chars().peekable();
//         while let Some(ch) = equation_chars.next() {
//             if ch.is_ascii_digit() {
//                 num_str_buf.push(ch);
//                 if equation_chars.peek().unwrap_or(&'\0').is_ascii_digit() == false {
//                     self.tokens
//                         .push(Token::Constant(num_str_buf.parse::<f32>().unwrap()));
//                     num_str_buf.clear();
//                 }
//             } else if let Some(c) = self.vars.get(&ch).or(CONSTANTS.get(&ch)) {
//                 self.tokens.push(Token::Constant(c.clone()));
//             } else if let Some(operator) = OPERATORS.get(&ch) {
//                 if let OperatorKind::Special = operator.kind {
//                     match ch {
//                         '(' => buf_operators.push((ch, operator.clone())),
//                         ')' => {
//                             while let Some(last_operator) = buf_operators.last() {
//                                 match last_operator.1.kind {
//                                     OperatorKind::Special => {
//                                         if last_operator.0 == '(' {
//                                             break;
//                                         }
//                                     }
//                                     _ => {

//                                     }
//                                     OperatorKind::Binary(binary) => {
//                                         binary.rhs = self.tokens.pop().unwrap();
//                                         binary.lhs = self.tokens.pop().unwrap();
//                                         self.tokens.push(Token::Operator(Operat::BinaryOperator(
//                                             binary.clone(),
//                                         )));
//                                     }
//                                     OperatorKind::Unary => todo!(),
//                                 }
//                             }
//                         }
//                     }
//                 } else if let Operat::BinaryOperator(binary) = operator {
//                     while let Some(prev_oper) = buf_operators.last() {
//                         if get_precedence_operator(prev_oper.clone()) >= binary.info.precedence {
//                             let info = buf_operators.pop().unwrap();
//                             let oper = match prev_oper {
//                                 Operat::BinaryOperator(prev_binary) => {
//                                     let mut result = prev_binary.clone();
//                                     result.rhs = self.tokens.pop().unwrap();
//                                     result.lhs = self.tokens.pop().unwrap();
//                                     Operat::BinaryOperator(result)
//                                 }
//                                 Operat::SpecialOperator(_) => todo!(),
//                             };
//                             self.tokens.push(Token::Operator(oper.clone()));
//                         } else {
//                             break;
//                         }
//                     }
//                     buf_operators.push(Operat::BinaryOperator(binary));
//                 }
//             } else {
//                 panic!("Parse error");
//             }
//         }
//         if buf_operators.is_empty() == false {
//             while let Some(prev_oper) = buf_operators.pop() {
//                 let oper = match prev_oper {
//                     Operat::BinaryOperator(mut prev_binary) => {
//                         prev_binary.rhs = self.tokens.pop().unwrap();
//                         prev_binary.lhs = self.tokens.pop().unwrap();
//                         Operat::BinaryOperator(prev_binary)
//                     }
//                     Operat::SpecialOperator(_) => todo!(),
//                 };
//                 self.tokens.push(Token::Operator(oper));
//             }
//         }
//     }
//     pub fn process_operator(operator: Operat) {}
//     pub fn compute(&mut self) -> ResultType {
//         self.tokens.last().unwrap().evaluate()
//     }
//     fn evaluate(&self, token: Token) -> f32 {
//         match token {
//             Token::Constant(c) => c.clone(),
//             // Token::Variable(c) => {}
//             _ => panic!(),
//         }
//     }
// }
// #[derive(Debug, Clone)]
// pub enum Token {
//     Constant(f32),
//     Variable(char),
//     Parantheses(char),
//     Operator(Operator),
// }
// impl Token {
//     fn evaluate(&self) -> ResultType {
//         match self {
//             Token::Constant(c) => c.clone(),
//             Token::Operator(oper) => match oper {
//                 Operat::BinaryOperator(binary) => {
//                     (binary.operation)(binary.lhs.evaluate(), binary.rhs.evaluate())
//                 }
//                 Operat::SpecialOperator(_) => panic!("CANT EVALUATE SPECIAL OPERATOR"),
//             },
//             _ => panic!(),
//         }
//     }
// }
// #[derive(Debug, Clone)]
// pub struct Operator {
//     precedence: usize,
//     kind: OperatorKind,
// }
// impl Operator {
//     pub fn new(precedence: usize, operator_type: OperatorKind) -> Self {
//         Operator {
//             precedence,
//             kind: operator_type,
//         }
//     }
// }
// #[derive(Debug, Clone)]
// pub enum OperatorKind {
//     Binary(BinaryOperator),
//     Unary,
//     Special,
// }
// #[derive(Debug, Clone)]
// struct BinaryOperator {
//     operation: fn(x: ResultType, y: ResultType) -> ResultType,
//     lhs: Token,
//     rhs: Token,
// }
// impl BinaryOperator {
//     const fn new(operation: fn(x: ResultType, y: ResultType) -> ResultType) -> Self {
//         Self {
//             operation,
//             lhs: Token::Constant(0f32),
//             rhs: Token::Constant(0f32),
//         }
//     }
// }
// // fn get_constant(name: char) -> Option<&'static ResultType> {
// //     for oper in CONSTANTS {
// //         if oper.0 == name {
// //             return Some(&oper.1);
// //         }
// //     }
// //     None
// // }
// // fn get_operator(name: char) -> Option<Operat> {
// //     for oper in BINARY_OPERATORS {
// //         if oper.info.description == name {
// //             return Some(Operat::BinaryOperator(Box::new(oper.clone())));
// //         }
// //     }
// //     for oper in SPECIAL_OPERATORS {
// //         if oper.description == name {
// //             return Some(Operat::SpecialOperator(oper.clone()));
// //         }
// //     }
// //     None
// // }
// // #[derive(Debug, Clone)]
// // pub enum Operat {
// //     BinaryOperator(Box<BinaryOperator>),
// //     SpecialOperator(SpecialOperator),
// // }

// // const CONSTANTS: &[(char, ResultType)] = &[];
// // const BINARY_OPERATORS: &[BinaryOperator] = &[
// //     BinaryOperator::new(('+', 1).into(), |x, y| x + y),
// //     BinaryOperator::new(('-', 1).into(), |x, y| x - y),
// //     BinaryOperator::new(('/', 2).into(), |x, y| x / y),
// //     BinaryOperator::new(('*', 2).into(), |x, y| x * y),
// // ];

// // const SPECIAL_OPERATORS: &[SpecialOperator] = &[
// //     OperatorInfo::new('(', 0).into(),
// //     OperatorInfo::new(')', 0).into(),
// // ];

// lazy_static! {
//     static ref OPERATORS: HashMap<char, Arc<Operator>> = {
//         let mut map = HashMap::new();
//         map.insert(
//             '(',
//             Operator::new(                0
//             ,OperatorKind::Special).into(),
//         );
//         map.insert(
//             ')',
//             Operator::new( 1,OperatorKind::Special).into(),
//         );
//         map.insert(
//             '+',
//             Operator::new(OperatorInfo {
//                 description: '+',
//                 precedence: 1,
//             },OperatorKind::Binary(BinaryOperator::new(|x, y| x + y))).into(),
//             .into(),
//         );
//         map.insert(
//             '-',
//             Operator::new(OperatorInfo {
//                 description: '-',
//                 precedence: 0,
//             },OperatorKind::Binary(BinaryOperator::new(|x, y| x - y))).into(),
//             .into(),
//         );
//         map.insert(
//             '*',
//             OperatorInfo {
//                 operation: |x, y| x * y,
//                 description: '*',
//                 precedence: 2,
//             }
//             .into(),
//         );
//         map.insert(
//             '/',
//             OperatorInfo {
//                 operation: |x, y| x / y,
//                 description: '/',
//                 precedence: 2,
//             }
//             .into(),
//         );
//         map
//     };
//     static ref CONSTANTS: HashMap<char, ResultType> = {
//         let mut map = HashMap::from([('k', 9f32 * 10f32.powf(9f32))]);
//         map
//     };
// }
