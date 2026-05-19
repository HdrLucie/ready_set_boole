use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Equal,
    Implic
}

#[derive(Debug)]
pub enum Node {
    Leaf(char),
    UnaryExpression {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpression {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    }
}

#[derive(Debug)]
pub enum TreeErrors {
    InvalidExpression,
}

// Need to be implemented to use println - it's a trait.
impl fmt::Display for TreeErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TreeErrors::InvalidExpression => write!(f, "Invalid expression"),
        }
    }
}

impl error::Error for TreeErrors {}

impl Operator {
    fn apply_operation(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Operator::Or => lhs | rhs,
            Operator::And => lhs & rhs,
            Operator::Not => !lhs,
            Operator::Xor => lhs ^ rhs,
            Operator::Equal => lhs == rhs,
            Operator::Implic => !lhs | rhs,
        }
    }
}

fn manage_error(c: char) {
    eprintln!("Wrong character : {}", c);
    std::process::exit(1);
}

fn apply(node: &Node) -> bool {
    match node {
        Node::Leaf(v) => *v,
        Node::UnaryExpression { op, child } => {
            let val = apply(child);
            op.apply_operation(val, false)
        },
        Node::BinaryExpression { op, lhs, rhs } => {
            let lhs_val = apply(lhs);
            let rhs_val = apply(rhs);
            op.apply_operation(lhs_val, rhs_val)
        }
    }
}

fn create_tree(expr : String) -> Result<(Node, HashSet<char>),TreeErrors> {
    let mut tree : Vec<Node> = Vec::new();
    let mut variables = HashSet::new();
    for c in expr.chars() {
        match c {
            c if c.is_alphabetic() => {
                tree.push(Node::Leaf(c));
                variables.insert(c);
            },
            '!' => { let child = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::UnaryExpression { op: Operator::Not, child: Box::new(child) });
            },
            '&' => { let rhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                let lhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::BinaryExpression { op: Operator::And, lhs: Box::new(lhs), rhs: Box::new(rhs) });
            },
            '|' => { let rhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                let lhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::BinaryExpression { op: Operator::Or, lhs: Box::new(lhs), rhs: Box::new(rhs) });
            },
            '^' => { let rhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                let lhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::BinaryExpression { op: Operator::Xor, lhs: Box::new(lhs), rhs: Box::new(rhs) });
            },
            '>' => { let rhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                let lhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::BinaryExpression { op: Operator::Implic, lhs: Box::new(lhs), rhs: Box::new(rhs) });
            },
            '=' => { let rhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                let lhs = tree.pop().ok_or(TreeErrors::InvalidExpression)?;
                tree.push(Node::BinaryExpression { op: Operator::Equal, lhs: Box::new(lhs), rhs: Box::new(rhs) });
            },
            _ => manage_error(c),
        }
    }
    Ok((tree.pop().unwrap(), variables))
}

fn eval_formula(expr: String) -> Result<bool, TreeErrors> {
    let (root, _variables) = create_tree(expr)?;
    Ok(apply(&root))
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of args.");
        std::process::exit(1);
    }
    let expr = &args[1];
    match eval_formula(expr.to_string()) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Erreur : {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_boolean_evaluation() {
        assert_eq!(false, eval_formula("10&".to_string()));
        assert_eq!(true, eval_formula("10|".to_string()));
        assert_eq!(true, eval_formula("11>".to_string()));
        assert_eq!(false, eval_formula("10=".to_string()));
        assert_eq!(true, eval_formula("1011||=".to_string()));
        assert_eq!(true, eval_formula("110001>|&&|".to_string()));
        assert_eq!(false, eval_formula("110001>|&&&".to_string()));
        assert_eq!(true, eval_formula("10!&".to_string()));
        assert_eq!(true, eval_formula("1101^=&".to_string()));
        assert_eq!(false, eval_formula("01011>=&^".to_string()));
    }
}
