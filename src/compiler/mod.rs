use std::vec::Vec;
use std::rc::Rc;

use crate::{types::*};
use std::collections::HashMap;

pub struct Interpreter;
struct InterpreterStore {
    pub variable_data: HashMap<usize, Rc<BramaAstType>>
}

impl Interpreter {
    pub fn execute(asts: &Vec<Rc<BramaAstType>>) -> Vec<Result<Rc<BramaAstType>, String>> {
        let mut results = Vec::new();
        let mut store   = InterpreterStore { variable_data: HashMap::new() };
        for ast in asts {
            results.push(Interpreter::execute_ast(&mut store, ast.clone()));
        }
        results
    }

    fn execute_ast(store: &mut InterpreterStore, ast: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        match &*ast {
            BramaAstType::Binary { left, operator, right } => Interpreter::executer_binary(store, left.clone(), *operator, right.clone()),
            BramaAstType::Assignment { index, variable: _, expression } => Interpreter::executer_assignment(store, *index, expression.clone()),
            BramaAstType::Variable(index) => Interpreter::executer_variable(store, *index),
            BramaAstType::Percent(_) => Ok(ast),
            BramaAstType::Number(_) => Ok(ast),
            BramaAstType::Time(_) => Ok(ast),
            _ => {
                println!("Operation not implemented {:?}", ast);
                Ok(Rc::new(BramaAstType::None))
            }
        }
    }

    fn executer_variable(store: &mut InterpreterStore, index: usize) -> Result<Rc<BramaAstType>, String> {
        match store.variable_data.get(&index) {
            Some(value) => Ok(value.clone()),
            _ => Err("Variable not found".to_string())
        }
    }

    fn executer_assignment(store: &mut InterpreterStore, index: usize, expression: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed  = Interpreter::execute_ast(store, expression)?;
        store.variable_data.insert(index, computed.clone());
        Ok(computed.clone())
    }

    fn executer_binary(store: &mut InterpreterStore, left: Rc<BramaAstType>, operator: char, right: Rc<BramaAstType>) -> Result<Rc<BramaAstType>, String> {
        let computed_left  = Interpreter::execute_ast(store, left)?;
        let computed_right = Interpreter::execute_ast(store, right)?;

        let result = match operator {
            '+' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number + ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num + r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '-' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) - number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number - ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num - r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '*' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) * number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number * ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num * r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            '/' => {
                match (&*computed_left, &*computed_right) {
                    (BramaAstType::Percent(percent), BramaAstType::Number(number)) => ((number * percent) / 100.0) / number,
                    (BramaAstType::Number(number), BramaAstType::Percent(percent)) => number / ((number * percent) / 100.0),
                    (BramaAstType::Number(l_num), BramaAstType::Number(r_num)) => l_num / r_num,
                    _ => return Err("Syntax error".to_string())
                }
            },
            _ => return Err("Syntax error".to_string())
        };

        Ok(Rc::new(BramaAstType::Number(result)))
    }
}
