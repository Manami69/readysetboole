use crate::ex05::negationform::negation_normal_form;
use crate::utils::binary_tree::*;
use std::collections::HashSet;
const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const OPERATION_CHARS: &str = "!&|ABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl LogicalBT {
    pub fn fill_set_tree(formula: &str, sets: Vec<HashSet<i32>>, used_letters: String) -> Self {
        let mut stack = Vec::<TNode>::new();

        for c in formula.chars() {
            let char_type: CharType = match OPERATION_CHARS.find(c) {
                Some(x) => {
                    if x == 0 {
                        CharType::OperationUni
                    } else if x <= 2 {
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
                        _ => panic!("Not happening ;)"),
                    };
                    let right = stack.pop().expect("Formula not well formated");
                    let left = stack.pop().expect("Formula not well formated");

                    stack.push(TNode::new(operation, left, right));
                }
                CharType::Boolean => {
                    stack.push(TNode::new_boxed(
                        Op::Set(sets[used_letters.find(c).unwrap() as usize].clone()),
                        None,
                        None,
                    ));
                }
            }
        }
        let node_head = stack.pop().expect("Formula not well formated");
        if !stack.is_empty() {
            panic!("Wrong number of arguments in formula")
        }

        LogicalBT::new(node_head)
    }
    pub fn set_collapse(&self, full_set: &HashSet<i32>) -> HashSet<i32> {
        match &self.head {
            Some(x) => LogicalBT::set_collapse_run(&Box::new(x.clone()), full_set),
            None => {
                println!("Empty binary tree");
                HashSet::new()
            }
        }
    }

    fn set_collapse_run(node: &Box<TNode>, full_set: &HashSet<i32>) -> HashSet<i32> {
        let mut r: Option<HashSet<i32>> = None;
        let mut l: Option<HashSet<i32>> = None;
        // go to the leafs
        if let Some(left) = &node.left {
            l = Some(LogicalBT::set_collapse_run(left, full_set));
        }

        if let Some(right) = &node.right {
            r = Some(LogicalBT::set_collapse_run(right, full_set));
        }
        // get branch values or false if None
        // (case when we're in a leaf and not using r and l)
        let r = if let Some(x) = r { x } else { HashSet::new() };
        let l = if let Some(x) = l { x } else { HashSet::new() };

        match &node.operation {
            Op::Negation => full_set
                .difference(&l)
                .into_iter()
                .map(|num| *num)
                .collect(),
            Op::Conjuction => l.intersection(&r).into_iter().map(|num| *num).collect(),
            Op::Disjunction => l.union(&r).into_iter().map(|num| *num).collect(),
            Op::Set(val) => val.clone(),
            _ => HashSet::new(),
        }
    }
}

pub fn eval_set<const N: usize, const M: usize>(formula: &str, sets: &[[i32; N]; M]) -> Vec<i32> {
    let mut used_letters = String::new();
    for c in LETTERS.chars() {
        match formula.find(c) {
            Some(_) => {
                used_letters.push(c);
            }
            None => {}
        }
    }
    if used_letters.len() != M {
        panic!("number of letters and number of set are different")
    }

    let nnf_formula = negation_normal_form(formula);
    let full_set: HashSet<i32> = sets.into_iter().flat_map(|num| *num).collect();
    let mut sets_maps: Vec<HashSet<i32>> = Vec::new();
    for set in sets {
        sets_maps.push(set.into_iter().map(|num| *num).collect());
    }
    let tree = LogicalBT::fill_set_tree(&nnf_formula, sets_maps, used_letters);
    tree.set_collapse(&full_set)
        .into_iter()
        .map(|num| num)
        .collect::<Vec<i32>>()
}
