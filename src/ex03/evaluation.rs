use crate::utils::binary_tree::{CharType, LogicalBT, Op, TNode};
const OPERATION_CHARS: &str = "!&|^>=01";

impl LogicalBT {
    pub fn fill_tree(formula: &str) -> LogicalBT {
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
                    let case: bool = match c {
                        '1' => true,
                        '0' => false,
                        _ => panic!("still not happening O:)"),
                    };
                    stack.push(TNode::bool_node(case));
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
/// 1 or 0 or operation if there is a 01 after an operation it will make a different branch
pub fn eval_formula(formula: &str) -> bool {
    LogicalBT::fill_tree(formula).collapse()
}
