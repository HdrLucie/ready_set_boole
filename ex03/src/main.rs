use std::env;

pub enum Operator {
    Not,
    And,
    Or,
    Xor,
    Equal,
    Implic
}

pub enum Node {
    Leaf(bool),
    Expression {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    }
}

fn create_tree(expr : String) {
    let mut tree : Vec<Node> = Vec::new();
    for c in expr.chars() {
        println!("{}", c);
        match c {
            '1' => tree.push(Node::Leaf(true)),
            '0' => tree.push(Node::Leaf(false))
        }
    }
}

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Wrong number of args.");
        std::process::exit(1);
    }
    let expr = &args[1];
    create_tree(expr.to_string());
}
