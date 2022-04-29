use crate::utils::binary_tree::{CharType, LogicalBT, Op, TNode};
const OPERATION_CHARS: &str = "!&|^>=ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl LogicalBT {
    pub fn fill_variable_tree(formula: &str) -> Self {
        let mut stack = Vec::<TNode>::new();

        for c in formula.chars() {
            let char_type: CharType = match OPERATION_CHARS.find(c) {
                Some(x) => {
                    if x == 0 {
                        CharType::OperationUni
                    } else if x <= 5 {
                        CharType::OperationBin
                    } else {
                        CharType::Boolean
                    }
                }
                None => panic!("char unrecognised in formula"),
            };
            match char_type {
                CharType::OperationUni => {
                    let node = stack.pop().expect("Formula not well formated");
                    stack.push(TNode::neg_node(node));
                }
                CharType::OperationBin => {
                    let operation: Op = match c {
                        '&' => Op::Conjuction,
                        '|' => Op::Disjunction,
                        '^' => Op::ExlDisjunction,
                        '>' => Op::MatCondition,
                        '=' => Op::LogicalEquivalence,
                        _ => panic!("Not happening ;)"),
                    };
                    let right = stack.pop().expect("Formula not well formated");
                    let left = stack.pop().expect("Formula not well formated");

                    stack.push(TNode::new(operation, left, right));
                }
                CharType::Boolean => {
                    stack.push(TNode::var_node(c));
                }
            }
        }
        let node_head = stack.pop().expect("Formula not well formated");
        if !stack.is_empty() {
            panic!("Wrong number of arguments in formula")
        }

        LogicalBT::new(node_head)
    }
}

impl TNode {
    pub fn negation(&mut self) {
        let mut redo = false;
        match self.left.as_mut() {
            Some(node) => node.negation(),
            None => {}
        };
        match self.right.as_mut() {
            Some(node) => node.negation(),
            None => {}
        };
        let new: Option<Box<TNode>> = match self.operation {
            Op::Negation => {
                let mut next_node = self.left.clone().unwrap();
                match next_node.operation {
                    Op::Negation => {
                        redo = true;
                        next_node.left.take()
                    }
                    Op::Conjuction => {
                        redo = true;
                        Some(Box::new(TNode::disj_node(
                            TNode::new_boxed(Op::Negation, next_node.left.take(), None),
                            TNode::new_boxed(Op::Negation, next_node.right.take(), None),
                        )))
                    }
                    Op::Disjunction => {
                        redo = true;
                        Some(Box::new(TNode::conj_node(
                            TNode::new_boxed(Op::Negation, next_node.left.take(), None),
                            TNode::new_boxed(Op::Negation, next_node.right.take(), None),
                        )))
                    }
                    _ => None,
                }
            }
            Op::MatCondition => {
                redo = true;
                Some(Box::new(TNode::new_boxed(
                    Op::Disjunction,
                    Some(Box::new(TNode::new_boxed(
                        Op::Negation,
                        self.left.take(),
                        None,
                    ))),
                    self.right.take(),
                )))
            }
            Op::LogicalEquivalence => {
                redo = true;
                Some(Box::new(TNode::conj_node(
                    TNode::new_boxed(Op::MatCondition, self.left.clone(), self.right.clone()),
                    TNode::new_boxed(Op::MatCondition, self.right.take(), self.left.take()),
                )))
            }
            Op::ExlDisjunction => {
                redo = true;
                Some(Box::new(TNode::disj_node(
                    TNode::new_boxed(
                        Op::Conjuction,
                        self.left.clone(),
                        Some(Box::new(TNode::new_boxed(
                            Op::Negation,
                            self.right.clone(),
                            None,
                        ))),
                    ),
                    TNode::new_boxed(
                        Op::Conjuction,
                        Some(Box::new(TNode::new_boxed(
                            Op::Negation,
                            self.left.take(),
                            None,
                        ))),
                        self.right.take(),
                    ),
                )))
            }
            _ => None,
        };
        match new {
            Some(val) => {
                self.operation = val.operation;
                self.left = val.left;
                self.right = val.right;
            }
            None => {}
        };
        if redo {
            self.negation()
        }
    }
}
pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = LogicalBT::fill_variable_tree(formula);
    let mut head = tree.head.unwrap();
    head.negation();
    tree.head = Some(head);
    tree.get_string()
}
