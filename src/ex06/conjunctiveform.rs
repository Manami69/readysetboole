use crate::ex05::negationform::negation_normal_form;
use crate::utils::binary_tree::*;

const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ!";

impl TNode {
    pub fn conj_distributivity(&mut self) {
        let mut redo = false;
        match self.left.as_mut() {
            Some(node) => node.conj_distributivity(),
            None => {}
        };
        match self.right.as_mut() {
            Some(node) => node.conj_distributivity(),
            None => {}
        };
        let new: Option<Box<TNode>> = match self.operation {
            Op::Disjunction => {
                let mut res: Option<Box<TNode>> = None;
                let mut left = self.left.clone().unwrap();
                let mut right = self.right.clone().unwrap();
                if left.operation == Op::Conjuction {
                    if right.operation != Op::Conjuction {
                        res = Some(Box::new(TNode::conj_node(
                            TNode::new_boxed(Op::Disjunction, self.right.clone(), left.left.take()),
                            TNode::new_boxed(Op::Disjunction, self.right.take(), left.right.take()),
                        )));
                    } else {
                        res = Some(Box::new(TNode::conj_node(
                            TNode::new_boxed(Op::Disjunction, self.left.clone(), right.left.take()),
                            TNode::new_boxed(Op::Disjunction, self.left.take(), right.right.take()),
                        )));
                    }
                    redo = true;
                } else if right.operation == Op::Conjuction {
                    res = Some(Box::new(TNode::conj_node(
                        TNode::new_boxed(Op::Disjunction, self.left.clone(), right.left.take()),
                        TNode::new_boxed(Op::Disjunction, self.left.take(), right.right.take()),
                    )));
                    redo = true;
                }
                res
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
            self.conj_distributivity()
        }
    }
}

fn is_over_test(formula: &str) -> Option<String> {
    if formula.find("&") == None {
        // if there is no & in the formula
        if formula.trim_end_matches("|").find("|") == None {
            // if all | are in the end
            return Some(formula.to_string());
        } else {
            let mut ret = String::new();
            for c in formula.chars() {
                if CHARS.find(c) != None {
                    ret.push(c);
                }
            }
            for c in formula.chars() {
                if c == '|' {
                    ret.push(c);
                }
            }
            return Some(ret);
        }
    }
    if formula.find("|") == None {
        // if there is no | in the formula
        if formula.trim_end_matches("&").find("&") == None {
            // if all & are in the end
            return Some(formula.to_string());
        } else {
            let mut ret = String::new();
            for c in formula.chars() {
                if CHARS.find(c) != None {
                    ret.push(c);
                }
            }
            for c in formula.chars() {
                if c == '&' {
                    ret.push(c);
                }
            }
            return Some(ret);
        }
    }
    if formula.trim_end_matches("&").find("&") == None {
        // if all & are in the end
        return Some(formula.to_string());
    }
    None
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let nnf_formula = negation_normal_form(formula);
    match is_over_test(&nnf_formula) {
        Some(v) => return v,
        None => {}
    };
    let mut tree = LogicalBT::fill_variable_tree(&nnf_formula);
    let mut head = tree.head.unwrap();
    head.conj_distributivity();
    tree.head = Some(head);
    tree.get_string()
}
